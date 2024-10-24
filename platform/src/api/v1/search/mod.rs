use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json},
    ServerFnError, *,
};
use serde::{Deserialize, Serialize};

use crate::api::common::CommonQueryResponse;
use crate::models::survey::{Age, Gender, ProofId, RegionCode, SalaryTier};

#[server(endpoint = "/v1/search", input = GetUrl, output = Json)]
pub async fn search(q: String) -> Result<CommonQueryResponse<SearchResult>, ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/search: {:?}", q);

    Ok(CommonQueryResponse {
        items: vec![SearchResult {
            proof_id: "proof-id".to_string(),
            salary_tier: Some(1),
            region_code: Some(2),
            age: Some(Age::Specific(20)),
            gender: None,
        }],
        bookmark: None,
    })
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum SearchQuery {
    SearchPanel {
        // FIXME: it can be modified, because it is a part of noncelab.
        // e.g. 1, 2, 3, 4, 5
        salary_tier: Option<SalaryTier>,

        // e.g. 02(Seoul), 051(Busan) and so on.
        region_code: Option<RegionCode>,
        gender: Option<Gender>,
        age: Option<Age>,
    },
    SearchAttiributeCertificate {
        // FIXME: it can be modified, because it is a part of noncelab.
        // e.g. 1, 2, 3, 4, 5
        salary_tier: Option<SalaryTier>,

        // e.g. 02(Seoul), 051(Busan) and so on.
        region_code: Option<RegionCode>,
        gender: Option<Gender>,
        age: Option<Age>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    // FIXME: it can be modified, because it is a part of noncelab.
    proof_id: ProofId,
    salary_tier: Option<SalaryTier>,
    region_code: Option<RegionCode>,
    gender: Option<Gender>,
    age: Option<Age>,
}
