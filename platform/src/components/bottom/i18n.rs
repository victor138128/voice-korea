use crate::utils::context::Language;
pub struct BottonTranslate {
    pub address: String,
    pub company: String,
    pub company_address: String,
    pub ceo: String,
    pub register_address: String,
    pub copyright: String,
}

pub fn translate(lang: Language) -> BottonTranslate {
    if lang == Language::En {
        BottonTranslate {
            address: "서울광역시 XX구 XX로4길 00, 0층".to_string(),
            company: "(주) 바이야드".to_string(),
            company_address: "서울광역시 XX구 XX로 00, 0층".to_string(),
            ceo: "대표 XXX".to_string(),
            register_address: "사업자등록번호 591-87-01919".to_string(),
            copyright: "Copyright © 2024 Union Labs Corp. All rights reserved.".to_string(),
        }
    } else {
        BottonTranslate {
            address: "서울광역시 XX구 XX로4길 00, 0층".to_string(),
            company: "(주) 바이야드".to_string(),
            company_address: "서울광역시 XX구 XX로 00, 0층".to_string(),
            ceo: "대표 XXX".to_string(),
            register_address: "사업자등록번호 591-87-01919".to_string(),
            copyright: "Copyright © 2024 Union Labs Corp. All rights reserved.".to_string(),
        }
    }
}
