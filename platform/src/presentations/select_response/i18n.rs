use crate::utils::context::Language;

pub struct SelectResponseTranslate {
    pub temporary_save: String,
    pub question_list: String,
    pub question_list_info_first: String,
    pub question_list_info_second: String,
    pub collect_response_title: String,
    pub collect_response_description: String,
    pub select_response_attribute_title: String,
    pub select_response_attribute_description: String,
    pub select_response_panel_title: String,
    pub select_response_panel_description: String,
    pub back: String,
}

pub fn translate(lang: Language) -> SelectResponseTranslate {
    match lang {
        Language::En => SelectResponseTranslate {
            temporary_save: "Temporary Save".to_string(),
            question_list: "Question List".to_string(),
            question_list_info_first: "There are a total of ".to_string(),
            question_list_info_second: " questions.".to_string(),
            collect_response_title: "Collect Response".to_string(),
            collect_response_description: "Select Response Properties and Response Panel.".to_string(),
            select_response_attribute_title: "Select response properties".to_string(),
            select_response_attribute_description: "Please select the characteristics of the respondent you wish to participate in the survey.
            Select respondents to participate in the survey based on the selected attributes.".to_string(),
            select_response_panel_title: "Select response panel".to_string(),
            select_response_panel_description: "Select respondents who want to participate in the survey.
            You can provide a survey to respondents who have the desired attributes.".to_string(),
            back: "Back".to_string(),
            
        },
        Language::Ko => SelectResponseTranslate {
            temporary_save: "임시 저장".to_string(),
            question_list: "설문 목록".to_string(),
            question_list_info_first: "총 ".to_string(),
            question_list_info_second: "개의 질문이 있습니다.".to_string(),
            collect_response_title: "응답 수집".to_string(),
            collect_response_description: "응답 속성 및 응답 패널을 선택해주세요.".to_string(),
            select_response_attribute_title: "응답 속성 선택".to_string(),
            select_response_attribute_description: "설문에 참여하기 원하는 응답자의 속성을 선택해주세요.
            선택한 속성을 기반으로 설문에 참여할 응답자를 선별합니다.".to_string(),
            select_response_panel_title: "응답 패널 선택".to_string(),
            select_response_panel_description: "설문에 참여하기 원하는 응답자를 선별해 보세요.
            응답자 중 원하는 속성을 가진 응답자에게 설문을 제공할 수 있습니다.".to_string(),
            back: "돌아가기".to_string(),
        },
    }
}
