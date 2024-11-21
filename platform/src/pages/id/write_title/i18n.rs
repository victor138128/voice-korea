use crate::utils::context::Language;

pub struct WriteTitleTranslate {
    pub temporary_storage: String,
    pub write_title: String,
    pub write_title_hint: String,
    pub cancel: String,
    pub store: String,
}

pub fn translate(lang: Language) -> WriteTitleTranslate {
    match lang {
        Language::En => WriteTitleTranslate {
            temporary_storage: "Temporary Storage".to_string(),
            write_title: "Write a Questionnaire Title".to_string(),
            write_title_hint: "Please enter the title of the questionnaire.".to_string(),
            cancel: "Cancel".to_string(),
            store: "Save".to_string(),
        },
        Language::Ko => WriteTitleTranslate {
            temporary_storage: "임시 저장".to_string(),
            write_title: "설문지 제목 작성하기".to_string(),
            write_title_hint: "설문지의 제목을 입력해주세요.".to_string(),
            cancel: "취소".to_string(),
            store: "저장".to_string(),
        },
    }
}
