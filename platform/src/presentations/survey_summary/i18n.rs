use crate::utils::context::Language;

pub struct SurveySummaryTranslate {
    pub start_survey: String,
    pub set_response_date: String,
    pub year: String,
    pub month: String,
    pub day: String,
    pub survey_summary: String,
    pub select_panel_and_attribute: String,
    pub response_info: String,
    pub response_list: String,
    pub response_list_first: String,
    pub response_list_second: String,
    pub back: String,
    pub response_list_first_1: String,
    pub response_list_second_2: String,
}

pub fn translate(lang: Language) -> SurveySummaryTranslate {
    match lang {
        Language::En => SurveySummaryTranslate {
            start_survey: "Start Survey".to_string(),
            set_response_date: "Response time settings".to_string(),
            year: "YY".to_string(),
            month: "MM".to_string(),
            day: "EE".to_string(),
            survey_summary: "Questionnaire Summary".to_string(),
            select_panel_and_attribute: "Selection panel and properties".to_string(),
            response_info: "Number of survey respondents".to_string(),
            response_list: "Survey list".to_string(),
            response_list_first: "Total".to_string(),
            response_list_second: "have two questions.".to_string(),
            back: "Back".to_string(),
            response_list_first_1: "Total".to_string(),
            response_list_second_2: "".to_string(),
        },
        Language::Ko => SurveySummaryTranslate {
            start_survey: "설문 시작".to_string(),
            set_response_date: "응답 기간 설정".to_string(),
            year: "년".to_string(),
            month: "월".to_string(),
            day: "일".to_string(),
            survey_summary: "설문지 요약".to_string(),
            select_panel_and_attribute: "선택 패널 및 속성".to_string(),
            response_info: "명의 설문 대상자".to_string(),
            response_list: "설문 목록".to_string(),
            response_list_first: "총".to_string(),
            response_list_second: "개의 질문이 있습니다.".to_string(),
            back: "돌아가기".to_string(),
            response_list_first_1: "총".to_string(),
            response_list_second_2: "개".to_string(),
        },
    }
}
