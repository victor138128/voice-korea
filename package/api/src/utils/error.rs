use by_axum::axum::http::StatusCode;
use by_axum::axum::response::{IntoResponse, Response};
use by_axum::axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("input error: {0}")]
    ValidationError(String),

    #[error("DynamoDB Create Failed. Reason({0})")]
    DynamoCreateException(String),
    #[error("DynamoDB Query Failed. Reason({0})")]
    DynamoQueryException(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::DynamoCreateException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DynamoQueryException(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let error_id = uuid::Uuid::new_v4();
        let body = Json(json!({
                    "error": {
                        "id": error_id.to_string(),
                        "message": self.to_string(),
                    }
        }));

        (status_code, body).into_response()
    }
}
