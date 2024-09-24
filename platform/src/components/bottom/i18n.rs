use crate::utils::context::Language;
pub struct BottomTranslate {
    pub address: String,
    pub company: String,
    pub company_address: String,
    pub ceo: String,
    pub register_address: String,
    pub copyright: String,
}

pub fn translate(lang: Language) -> BottomTranslate {
    if lang == Language::En {
        BottomTranslate {
            address: "Floor 0, 00, XX-ro 4-gil, XX-gu, Seoul".to_string(),
            company: "Biyard Co., Ltd.".to_string(),
            company_address: "Floor 0, 00, XX-ro, XX-gu, Seoul".to_string(),
            ceo: "CEO XXX".to_string(),
            register_address: "Business Registration Number 591-87-01919".to_string(),
            copyright: "Copyright © 2024 Union Labs Corp. All rights reserved.".to_string(),
        }
    } else {
        BottomTranslate {
            address: "서울광역시 XX구 XX로4길 00, 0층".to_string(),
            company: "(주) 바이야드".to_string(),
            company_address: "서울광역시 XX구 XX로 00, 0층".to_string(),
            ceo: "대표 XXX".to_string(),
            register_address: "사업자등록번호 591-87-01919".to_string(),
            copyright: "Copyright © 2024 Union Labs Corp. All rights reserved.".to_string(),
        }
    }
}
