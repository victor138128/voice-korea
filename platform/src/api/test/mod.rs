use candid::CandidType;
use dioxus::prelude::{server, server_fn, ServerFnError};
use serde::{Deserialize, Serialize};
use server_fn::codec::{GetUrl, Json};

#[derive(CandidType, Deserialize, Serialize, Clone, PartialEq)]
pub struct Test {
    pub text: String,
}

#[server(endpoint = "/test", input=GetUrl, output=Json)]
pub async fn get_test_api() -> Result<Test, ServerFnError> {
    Ok(Test {
        text: "hello".to_string(),
    })
}
