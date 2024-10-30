use crate::utils::context::Language;

pub struct SelectResponseDetailTranslate {
    pub temporary_save: String,
    pub attribute_title: String,
    pub attribute_description: String,
    pub attribute_select_label: String,
    pub nation: String,
    pub gender: String,
    pub age: String,
    pub add_attribute: String,
    pub cancel: String,
    pub save: String,
}

pub fn translate(lang: Language) -> SelectResponseDetailTranslate {
    match lang {
        Language::En => SelectResponseDetailTranslate {
            temporary_save: "Temporary Save".to_string(),
            attribute_title: "Select response properties".to_string(),
            attribute_description: "How many responses are needed?".to_string(),
            attribute_select_label: "Select attributes of survey subjects".to_string(),
            nation: "Nation".to_string(),
            gender: "Gender".to_string(),
            age: "Age".to_string(),
            add_attribute: "Add Attribute".to_string(),
            cancel: "Cancel".to_string(),
            save: "Save".to_string(),
        },
        Language::Ko => SelectResponseDetailTranslate {
            temporary_save: "임시 저장".to_string(),
            attribute_title: "응답 속성 선택".to_string(),
            attribute_description: "몇개의 응답이 필요합니까?".to_string(),
            attribute_select_label: "설문 대상자의 속성 선택".to_string(),
            nation: "국가".to_string(),
            gender: "성별".to_string(),
            age: "연령".to_string(),
            add_attribute: "속성 추가하기".to_string(),
            cancel: "취소".to_string(),
            save: "저장".to_string(),
        },
    }
}
