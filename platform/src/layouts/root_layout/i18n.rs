use crate::utils::context::Language;

pub struct RootLayoutTranslate {
    pub logout: String,
    pub overview: String,
    pub search_project: String,
    pub import_project: String,
    pub survey_management: String,
    pub questionnaire_management: String,
    pub question_bank: String,
    pub property_management: String,
    pub property_status: String,
    pub user_settings: String,
}

pub fn translate(lang: Language) -> RootLayoutTranslate {
    match lang {
        Language::En => RootLayoutTranslate {
            logout: "Logout".to_string(),
            overview: "Overview".to_string(),
            search_project: "Search Project".to_string(),
            import_project: "Import Existing Project".to_string(),
            survey_management: "Survey Management".to_string(),
            questionnaire_management: "Questionnaire Management".to_string(),
            question_bank: "Question Bank".to_string(),
            property_management: "Property Management".to_string(),
            property_status: "Property Status".to_string(),
            user_settings: "User Settings".to_string(),
        },
        Language::Ko => RootLayoutTranslate {
            logout: "Logout".to_string(),
            overview: "Overview".to_string(),
            search_project: "프로젝트 검색".to_string(),
            import_project: "기존 프로젝트 가져오기".to_string(),
            survey_management: "설문 관리".to_string(),
            questionnaire_management: "설문지 관리".to_string(),
            question_bank: "질문 뱅크".to_string(),
            property_management: "속성 관리".to_string(),
            property_status: "속성 현황".to_string(),
            user_settings: "사용자 설정".to_string(),
        },
    }
}
