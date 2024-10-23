#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use serde::{Deserialize, Serialize};

use crate::models::survey::{Age, Gender, Quota, RegionCode, SalaryTier};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct DevNotifyAttrcertRequest {
    salary_tier: Option<SalaryTier>,
    region_code: Option<RegionCode>,
    gender: Option<Gender>,
    age: Option<Age>,
}

#[server(endpoint = "/v1/attrcerts", input = Json, output = Json)]
pub async fn notify_attrcert(
    proof: String,
    _dev: DevNotifyAttrcertRequest,
) -> Result<(), ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/attrcerts: {:?}", proof);

    Ok(())
}
