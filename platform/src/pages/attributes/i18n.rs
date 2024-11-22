use crate::utils::context::Language;

pub struct AttributeTranslate {
    pub temporary_save: String,
    pub attribute_status: String,
    pub selected_attribute: String,
    pub add_attribute: String,
    pub search_result: String,
    pub tabel_label: Vec<String>,
    pub search_hint: String,
    pub cancel: String,
    pub save: String,
}

pub fn translate(lang: Language) -> AttributeTranslate {
    match lang {
        Language::En => AttributeTranslate {
            temporary_save: "Temporary Save".to_string(),
            attribute_status: "Attribute Status".to_string(),
            selected_attribute: "Currently Selected Attribute".to_string(),
            add_attribute: "Add Attribute".to_string(),
            search_result: "Search Results".to_string(),
            tabel_label: vec![
                "No.".to_string(),
                "Attributes".to_string(),
                "Selected Attributes".to_string(),
            ],
            search_hint: "Please enter the property you want to search for".to_string(),
            cancel: "Cancel".to_string(),
            save: "Save".to_string(),
        },
        Language::Ko => AttributeTranslate {
            temporary_save: "임시 저장".to_string(),
            attribute_status: "속성 현황".to_string(),
            selected_attribute: "현재 선택된 속성".to_string(),
            add_attribute: "속성 추가하기".to_string(),
            search_result: "검색 결과".to_string(),
            tabel_label: vec![
                "No.".to_string(),
                "속성".to_string(),
                "선택 속성".to_string(),
            ],
            search_hint: "검색을 원하는 속성을 입력해 주세요".to_string(),
            cancel: "취소".to_string(),
            save: "저장".to_string(),
        },
    }
}
