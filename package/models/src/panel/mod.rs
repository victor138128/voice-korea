use serde::{Deserialize, Serialize};

use crate::prelude::PanelAttributeDetailInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpsertPanelRequest {
    pub id: Option<String>,
    pub name: String,
    pub count: i64,
    pub attribute: Vec<PanelAttributeInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PanelAttributeInfo {
    pub id: Option<String>,
    pub name: String,
    pub attribute: Vec<PanelAttributeDetailInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PanelSummary {
    pub id: String,
    pub name: String,
    pub count: i64,
    pub attribute: Vec<PanelAttributeInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelActionRequest {
    Delete,
}
