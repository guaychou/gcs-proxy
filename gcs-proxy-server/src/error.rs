use actix_web::http::StatusCode;
use actix_web::Error as ActixWebError;
use actix_web::{HttpResponse, ResponseError};
use gcs_proxy_deps::cloud_storage::Error as GcsError;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("`{0}`")]
    GcsError(#[from] GcsError),
    #[error("`{0}`")]
    ActixWebError(#[from] ActixWebError),
    #[error("not authorized")]
    Unauthorized,
}

impl AppError {
    fn get_codes(&self) -> StatusCode {
        match *self {
            AppError::GcsError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ActixWebError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Unauthorized => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let json_data = json!({
            "api_version": "v1",
            "msg": self.to_string(),
            "success": false
        });
        HttpResponse::build(self.get_codes()).json(json_data)
    }
}
