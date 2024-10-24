use crate::utils::context::Language;

pub struct WriteQuestionTranslate {
    pub add_question: String,
    pub back_label: String,
    pub save_label: String,
    pub delete: String,
    pub update: String,
    pub temporary_save: String,
    pub input_question: String,
    pub next_question: String,
    pub cancel_label: String,
}

pub fn translate(lang: Language) -> WriteQuestionTranslate {
    match lang {
        Language::En => WriteQuestionTranslate {
            add_question: "Add Question".to_string(),
            back_label: "Go Back".to_string(),
            save_label: "Save".to_string(),
            delete: "Delete".to_string(),
            update: "Update".to_string(),
            temporary_save: "Temporary Save".to_string(),
            input_question: "Please enter your question.".to_string(),
            next_question: "Next Question".to_string(),
            cancel_label: "Cancel".to_string(),
        },
        Language::Ko => WriteQuestionTranslate {
            add_question: "질문 추가하기".to_string(),
            back_label: "돌아가기".to_string(),
            save_label: "저장".to_string(),
            delete: "삭제".to_string(),
            update: "수정".to_string(),
            temporary_save: "임시 저장".to_string(),
            input_question: "질문을 입력하세요.".to_string(),
            next_question: "다음 질문".to_string(),
            cancel_label: "취소".to_string(),
        },
    }
}
