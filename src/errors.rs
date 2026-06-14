use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use validator::ValidationErrors;

pub enum AppError {
    NotFound(String),
    Validation(validator::ValidationErrors),
}

impl AppError {
    pub fn not_found(message: impl Into<String>) -> Self {
        return AppError::NotFound(message.into());
    }
}

#[derive(Serialize)]
struct JsonError {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<ValidationErrors>,
}

impl IntoResponse for JsonError {
    fn into_response(self) -> Response {
        return Json(self).into_response();
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        return match self {
            AppError::NotFound(message) => (
                StatusCode::NOT_FOUND,
                JsonError {
                    code: "NOT_FOUND".to_string(),
                    message,
                    details: None,
                },
            ),
            AppError::Validation(errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                JsonError {
                    code: "UNPROCESSABLE_ENTITY".to_string(),
                    message: "invalid data submitted".into(),
                    details: Some(errors),
                },
            ),
        }
        .into_response();
    }
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        return AppError::Validation(errors);
    }
}
