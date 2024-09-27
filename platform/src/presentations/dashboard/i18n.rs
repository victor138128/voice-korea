use crate::utils::context::Language;

pub struct DashboardTranslate {
    pub search: String,
    pub create_survey: String,
    pub survey_name: String,
    pub response_count: String,
    pub final_update_date: String,
    pub status: String,
    pub draft: String,
    pub in_progress: String,
    pub complete: String,
    pub update_date: String,
    pub add_question_description: String,
    pub response: String,
    pub edit_survey: String,
    pub analysis_result: String,
}

pub fn translate(lang: Language) -> DashboardTranslate {
    match lang {
        Language::En => DashboardTranslate {
            search: "Search".to_string(),
            create_survey: "Create a Survey".to_string(),
            survey_name: "Questionnaire Name".to_string(),
            response_count: "Response Count".to_string(),
            final_update_date: "Final Update Date".to_string(),
            status: "Status".to_string(),
            draft: "Draft".to_string(),
            in_progress: "In Progress".to_string(),
            complete: "Complete".to_string(),
            update_date: "Update Date".to_string(),
            add_question_description: "Add a Question".to_string(),
            response: " response".to_string(),
            edit_survey: "Edit Survey".to_string(),
            analysis_result: "Analysis Result".to_string(),
        },
        Language::Ko => DashboardTranslate {
            search: "Search".to_string(),
            create_survey: "설문 만들기".to_string(),
            survey_name: "설문지 이름".to_string(),
            response_count: "응답 수".to_string(),
            final_update_date: "최종 수정 날짜".to_string(),
            status: "상태".to_string(),
            draft: "초안".to_string(),
            in_progress: "진행중".to_string(),
            complete: "완료".to_string(),
            update_date: "수정 날짜".to_string(),
            add_question_description: "질문을 추가하세요".to_string(),
            response: "개 응답".to_string(),
            edit_survey: "설문 편집하기".to_string(),
            analysis_result: "결과 분석".to_string(),
        },
    }
}
