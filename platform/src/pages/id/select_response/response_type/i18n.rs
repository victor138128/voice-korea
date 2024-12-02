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

    pub select_responsive_panel: String,
    pub select_responsive_panel_description: String,
    pub selection_panel_groups: String,
    pub select_all: String,
    pub search_results: String,
    pub panel: String,

    pub search_hint: String,
    pub search_result: String,
    pub selected_attribute: String,
    pub attribute_setting: String,
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

            select_responsive_panel: "Select Responsive Panel".to_string(),
            select_responsive_panel_description: "Select a panel to offer your survey".to_string(),
            selection_panel_groups: "Selection panel recruitment group".to_string(),
            select_all: "Select All".to_string(),
            search_results: "Search Results".to_string(),
            panel: "Panel".to_string(),

            search_result: "Search Results".to_string(),
            selected_attribute: "Currently Selected Attribute".to_string(),
            search_hint: "Please enter the property you want to search for".to_string(),

            attribute_setting: "Attribute Setting".to_string(),
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

            select_responsive_panel: "응답 패널 선택".to_string(),
            select_responsive_panel_description: "설문조사를 제공할 패널 선택".to_string(),
            selection_panel_groups: "선택 패널 모집군".to_string(),
            select_all: "전체 선택하기".to_string(),
            search_results: "검색 결과".to_string(),
            panel: "패널".to_string(),

            search_result: "검색 결과".to_string(),
            selected_attribute: "현재 선택된 속성".to_string(),
            search_hint: "검색을 원하는 속성을 입력해 주세요".to_string(),

            attribute_setting: "속성 설정".to_string(),
        },
    }
}
