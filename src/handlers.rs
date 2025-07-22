use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::Column;
use crate::models::{ActiveModel, Entity as Urls};
use crate::response::{ApiResponse, empty_data};

#[derive(serde::Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(serde::Serialize)]
pub struct ShortenResponse {
    pub shortened_url: String,
}

pub async fn shorten_handler(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<ShortenRequest>,
) -> impl IntoResponse {
    let code = Uuid::new_v4().to_string()[..8].to_string();

    let new_url = ActiveModel {
        id: Set(Uuid::new_v4()),
        url: Set(payload.url.clone()),
        code: Set(code.clone()),
        count: Set(0),
    };

    if let Err(err) = new_url.insert(&db).await {
        return ApiResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to save URL: {}", err),
            empty_data(),
        );
    }

    let app_url = std::env::var("APP_URL").expect("APP_URL must be set");
    let response = ShortenResponse {
        shortened_url: format!("{}/{}", app_url, code),
    };

    ApiResponse::new(
        StatusCode::OK,
        "URL shortened successfully".to_string(),
        serde_json::to_value(response).unwrap(),
    )
}

pub async fn redirect_handler(
    State(db): State<DatabaseConnection>,
    Path(code): Path<String>,
) -> Redirect {
    if let Ok(Some(mut url)) = Urls::find().filter(Column::Code.eq(code)).one(&db).await {
        url.count += 1;
        let mut active_model: ActiveModel = url.clone().into();
        active_model.count = Set(url.count);

        tokio::spawn(async move {
            let _ = active_model.update(&db).await;
        });
        
        Redirect::permanent(&url.url)
    } else {
        Redirect::permanent("https://google.com")
    }
}

pub async fn ping_handler() -> impl IntoResponse {
    ApiResponse::new(StatusCode::OK, "pong".to_string(), empty_data())
}
