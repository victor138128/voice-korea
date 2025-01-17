use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateAttributeRequest {
    pub name: String,
    pub attribute: Vec<PanelAttributeDetailInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateAttributeRequest {
    pub name: String,
    pub attribute: Vec<PanelAttributeDetailInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeSummary {
    pub id: String,
    pub name: String,
    pub attribute: Vec<PanelAttributeDetailInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct PanelAttributeDetailInfo {
    pub id: Option<String>,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeActionRequest {
    Create(CreateAttributeRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeByIdActionRequest {
    Delete,
    Update(UpdateAttributeRequest),
}
