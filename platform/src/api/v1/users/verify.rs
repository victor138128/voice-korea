#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub verification_code: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VerifyEmailResponse {}

#[server(endpoint = "/v1/users/verify", input = PostUrl, output = Json)]
pub async fn verify_email(req: VerifyEmailRequest) -> Result<VerifyEmailResponse, ServerFnError> {
    tracing::debug!("/v1/users/verify: {:?}", req);

    Err(ServerFnError::ServerError(
        "not implemented yet".to_string(),
    ))
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetVerificationCodeRequest {
    pub email: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetVerificationCodeResponse {}

#[server(endpoint = "/v1/users/verify", input = GetUrl, output = Json)]
pub async fn get_verification_code(
    req: GetVerificationCodeRequest,
) -> Result<GetVerificationCodeResponse, ServerFnError> {
    tracing::debug!("/v1/users/verify: {:?}", req);
    // TODO: Send a verification code to the email

    Err(ServerFnError::ServerError(
        "not implemented yet".to_string(),
    ))
}
