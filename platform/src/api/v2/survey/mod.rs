use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json},
    ServerFnError, *,
};

#[server(endpoint = "/v2/surveys", input = GetUrl, output = Json)]
pub async fn list_surveys() -> Result<Vec<models::prelude::ListSurveyResponse>, ServerFnError> {
    use crate::utils::api::ReqwestClient;

    let client = ReqwestClient::new()?;
    let res = client.get("/v1/survey").send().await?;
    let res = res.error_for_status()?;

    let mut survey: Vec<models::prelude::ListSurveyResponse> = res.json().await?;

    // let res = client.get("/v1/survey/draft").send().await?;
    // let res = res.error_for_status()?;

    // let survey_draft: models::prelude::ListSurveyDraftResponse = res.json().await?;

    Ok(survey)
}

#[server(endpoint = "/v2/survey", input = GetUrl, output = Json)]
pub async fn get_survey(id: String) -> Result<models::prelude::Survey, ServerFnError> {
    use crate::utils::api::ReqwestClient;

    let client = ReqwestClient::new()?;
    let res = client.get(&format!("/v1/survey/{}", id)).send().await?;
    let res = res.error_for_status()?;
    Ok(res.json().await?)
}

//change survey status from draft to in_progress
#[server(endpoint = "/v2/survey/create", input = Json, output = Json)]
pub async fn create_survey(
    id: String,
) -> Result<models::prelude::ProgressSurveyResponse, ServerFnError> {
    use crate::utils::api::ReqwestClient;

    let client = ReqwestClient::new()?;
    let res = client.post(&format!("/v1/survey/{}", id)).send().await?;
    let res = res.error_for_status()?;
    Ok(res.json().await?)
}

#[server(endpoint = "/v2/survey/draft/upsert", input = Json, output = Json)]
pub async fn upsert_survey_draft(
    req: models::prelude::UpsertSurveyDraftRequest,
) -> Result<String, ServerFnError> {
    use crate::utils::api::ReqwestClient;

    let client = ReqwestClient::new()?;
    let res = client
        .patch(&format!("/v1/survey/draft"))
        .json(&req)
        .send()
        .await?;
    let res = res.error_for_status()?;
    Ok(res.json().await?)
}
