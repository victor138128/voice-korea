use by_axum::axum::http::StatusCode;
use by_axum::axum::response::{IntoResponse, Response};
use by_axum::axum::Json;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("input error: {0}")]
    ValidationError(String),

    #[allow(dead_code)]
    #[error("DynamoDB Create Failed. Reason({0})")]
    DynamoCreateException(String),

    #[allow(dead_code)]
    #[error("DynamoDB Query Failed. Reason({0})")]
    DynamoQueryException(String),

    #[error("Wrong User Login info ({0})")]
    InvalidCredentials(String),

    #[error("JWT Generation Failed. Reason({0})")]
    JWTGenerationFail(String),

    #[error("AWS SES Service is Failed. Reason({0})")]
    SESServiceError(String),

    #[error("Email verification code {0} does not match")]
    AuthKeyNotMatch(String),

    #[error("Email already used")]
    DuplicateUser,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::DynamoCreateException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::DynamoQueryException(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InvalidCredentials(_) => StatusCode::UNAUTHORIZED,
            ApiError::JWTGenerationFail(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::SESServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AuthKeyNotMatch(_) => StatusCode::NOT_ACCEPTABLE,
            ApiError::DuplicateUser => StatusCode::CONFLICT,
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
