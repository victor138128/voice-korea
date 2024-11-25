use crate::utils::context::Language;

pub struct ResponseReportTranslate {
    pub survey_summary: String,
    pub individual_response: String,
    pub response_report: String,
    pub response_download: String,
    pub total_respondents: String,
    pub respondent_type: String,
    pub status: String,
    pub final_update_date: String,
    pub time_taken: String,
    pub attribute: String,
    pub panel: String,
    pub response_history: String,
    pub draft: String,
    pub in_progress: String,
    pub complete: String,
}

pub fn translate(lang: Language) -> ResponseReportTranslate {
    match lang {
        Language::En => ResponseReportTranslate {
            survey_summary: "Survey Summary".to_string(),
            individual_response: "Individual Response".to_string(),
            response_report: "Response Report".to_string(),
            response_download: "Response Download".to_string(),
            total_respondents: "Total Respondents".to_string(),
            respondent_type: "Respondent Type".to_string(),
            status: "Status".to_string(),
            final_update_date: "Final Update Date".to_string(),
            time_taken: "Time Taken".to_string(),
            attribute: "Attribute".to_string(),
            panel: "Panel".to_string(),
            response_history: "Response History".to_string(),
            draft: "Draft".to_string(),
            in_progress: "In Progress".to_string(),
            complete: "Complete".to_string(),
        },
        Language::Ko => ResponseReportTranslate {
            survey_summary: "설문 요약".to_string(),
            individual_response: "개별 응답".to_string(),
            response_report: "응답 보고서".to_string(),
            response_download: "응답 다운로드".to_string(),
            total_respondents: "총 응답자".to_string(),
            respondent_type: "응답자 유형".to_string(),
            status: "상태".to_string(),
            final_update_date: "마지막 수정 날짜".to_string(),
            time_taken: "소요 시간".to_string(),
            attribute: "속성".to_string(),
            panel: "패널".to_string(),
            response_history: "응답 내역".to_string(),
            draft: "초안".to_string(),
            in_progress: "진행중".to_string(),
            complete: "완성".to_string(),
        },
    }
}
