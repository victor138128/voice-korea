use crate::utils::context::Language;

pub struct WriteQuestionTranslate {
    pub add_question: String,
    pub back_label: String,
    pub save_label: String,
}

pub fn translate(lang: Language) -> WriteQuestionTranslate {
    match lang {
        Language::En => WriteQuestionTranslate {
            add_question: "Add Question".to_string(),
            back_label: "Go Back".to_string(),
            save_label: "Save".to_string(),
        },
        Language::Ko => WriteQuestionTranslate {
            add_question: "질문 추가하기".to_string(),
            back_label: "돌아가기".to_string(),
            save_label: "저장".to_string(),
        },
    }
}
