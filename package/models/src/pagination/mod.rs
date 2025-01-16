#[derive(Debug, serde::Deserialize)]
pub struct Pagination {
    pub size: Option<usize>,
    pub bookmark: Option<String>,
    pub keyword: Option<String>, //keyword가 있을 경우 검색 수행
}
