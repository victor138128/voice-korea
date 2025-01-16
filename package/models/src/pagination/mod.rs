#[derive(Debug, serde::Deserialize)]
pub struct Pagination {
    pub size: Option<usize>,
    pub bookmark: Option<String>,
}
