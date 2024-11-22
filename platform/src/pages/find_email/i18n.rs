use crate::utils::context::Language;

pub struct FindEmailTranslate {
    pub find_email: String,
    pub name_label: String,
    pub name_hint: String,
    pub phone_label: String,
    pub phone_hint: String,
    pub send_authentication_number: String,
    pub authentication_number: String,
    pub authentication_number_description: Vec<String>,
    pub check_title: String,
    pub check_description: Vec<String>,

    pub get_email_description: String,
    pub email_address: String,
    pub go_to_login: String,
}

pub fn translate(lang: Language) -> FindEmailTranslate {
    match lang {
        Language::En => FindEmailTranslate {
            find_email: "Find Email".to_string(),
            name_label: "Name".to_string(),
            name_hint: "OOO".to_string(),
            phone_label: "CellPhone".to_string(),
            phone_hint: "010-0000-0000".to_string(),
            send_authentication_number: "Send Authentication Number".to_string(),
            authentication_number: "Authentication Number".to_string(),
            authentication_number_description: vec!["- Please enter the authentication number (6 digits) within 3 minutes.".to_string(), "- If the authentication time has expired, please resend the authentication number and then enter it.".to_string()],
            check_title: "Check it out!".to_string(),
            check_description: vec!["- Identity verification is only possible using a mobile phone activated in your name.".to_string(), "- If the authentication time has expired, please resend the authentication number and then enter it.".to_string()],
            get_email_description: "We found your registered email with the following information".to_string(),
            email_address: "Email Address".to_string(),
            go_to_login: "To the login screen".to_string(),
        },
        Language::Ko => FindEmailTranslate {
            find_email: "이메일 찾기".to_string(),
            name_label: "이름".to_string(),
            name_hint: "OOO".to_string(),
            phone_label: "휴대폰".to_string(),
            phone_hint: "010-0000-0000".to_string(),
            send_authentication_number: "인증번호 전송".to_string(),
            authentication_number: "인증번호".to_string(),
            authentication_number_description: vec!["- 3분 이내로 인증번호(6자리)를 입력해 주세요.".to_string(), "- 인증시간이 초과된 경우 인증번호를 재발송 하신 후 입력해 주세요.".to_string()],
            check_title: "확인하세요!".to_string(),
            check_description: vec!["- 본인인증은 본인명의로 개통된 휴대폰으로만 가능합니다.".to_string(), "- 휴대폰 본인인증 절차가 정상적으로 이루어지지 않을 경우 XXXX(XXXX-XXXX)로 문의하시기 바랍니다.".to_string()],
            get_email_description: "다음 정보로 가입된 이메일을 찾았습니다.".to_string(),
            email_address: "이메일 주소".to_string(),
            go_to_login: "로그인 화면으로".to_string(),
        },
    }
}
