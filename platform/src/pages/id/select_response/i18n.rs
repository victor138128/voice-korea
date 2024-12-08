use crate::utils::context::Language;

pub struct SelectResponseTranslate {
    pub question_list: String,
    pub question_list_info_first: String,
    pub question_list_info_second: String,
    pub collect_response_title: String,
    pub collect_response_description: String,
    pub add_attribute: String,
    pub save: String,
    pub back: String,
}

pub fn translate(lang: Language) -> SelectResponseTranslate {
    match lang {
        Language::En => SelectResponseTranslate {
            question_list: "Question List".to_string(),
            question_list_info_first: "There are a total of ".to_string(),
            question_list_info_second: " questions.".to_string(),
            collect_response_title: "Select response properties".to_string(),
            collect_response_description: "Please select a response attribute.".to_string(),
            add_attribute: "Add Attribute".to_string(),
            save: "Save".to_string(),
            back: "Back".to_string(),
        },
        Language::Ko => SelectResponseTranslate {
            question_list: "설문 목록".to_string(),
            question_list_info_first: "총 ".to_string(),
            question_list_info_second: "개의 질문이 있습니다.".to_string(),
            collect_response_title: "응답 속성 선택".to_string(),
            collect_response_description: "응답 속성을 선택해주세요.".to_string(),
            add_attribute: "속성 추가하기".to_string(),
            save: "저장".to_string(),
            back: "돌아가기".to_string(),
        },
    }
}
