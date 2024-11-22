use crate::utils::context::Language;
pub struct LoginTranslate {
    pub login: String,
    pub email: String,
    pub password: String,
    pub find_email: String,
    pub reset_pw: String,
    pub create_account: String,
    pub check_title: String,
    pub check_description_1: String,
    pub check_description_2: String,

    pub not_matched_error: String,
    pub not_exists_user_error: String,
    pub login_failed_error: String,
}

pub fn translate(lang: Language) -> LoginTranslate {
    match lang {
        Language::En => LoginTranslate {
            login: "Login".to_string(),
            email: "Email".to_string(),
            password: "Password".to_string(),
            find_email: "Find Email".to_string(),
            reset_pw: "Reset Password".to_string(),
            create_account: "Create Account".to_string(),
            check_title: "Check it out!".to_string(),
            check_description_1: "- The password must consist of at least 8 characters and a combination of letters, numbers, and special symbols.".to_string(),
            check_description_2: "- If you lose your ID or password or exceed 5 errors, please reset your website password or contact customer service (XXXX-XXXX).".to_string(),
            not_matched_error: "You entered your email or password incorrectly.".to_string(),
            not_exists_user_error: "This user does not exist.".to_string(),
            login_failed_error: "You have failed to log in. Please try again later.".to_string(),
        },
        Language::Ko => LoginTranslate {
            login: "로그인".to_string(),
            email: "이메일".to_string(),
            password: "비밀번호".to_string(),
            find_email: "이메일 찾기".to_string(),
            reset_pw: "비밀번호재설정".to_string(),
            create_account: "회원가입".to_string(),
            check_title: "확인하세요!".to_string(),
            check_description_1: "- 비밀번호는 영문, 숫자, 특수기호 조합으로 8자 이상 구성되어야 합니다.".to_string(),
            check_description_2: "- ID, 비밀번호를 분실, 5회 오류횟수 초과시에는 홈페이지 비밀번호 재설정 또는 고객센터(XXXX-XXXX)로 문의해 주세요.".to_string(),
            not_matched_error: "이메일 또는 비밀번호를 잘못 입력하셨습니다.".to_string(),
            not_exists_user_error: "존재하지 않는 유저입니다.".to_string(),
            login_failed_error: "로그인을 실패하셨습니다. 잠시 후 다시 시도해보세요.".to_string(),
        },
    }
}
