use dioxus::prelude::*;
use models::prelude::{
    AttributeSummary, PanelAttributeDetailInfo, PanelAttributeInfo, PanelSummary,
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    panels: Signal<Vec<PanelSummary>>,
    attributes: Signal<Vec<AttributeSummary>>,
}

impl Controller {
    pub fn init(_lang: dioxus_translate::Language) -> Self {
        //FIXME: fix to api
        let attributes = vec![
            AttributeSummary {
                id: "1".to_string(),
                name: "직업".to_string(),
                attribute: vec![
                    PanelAttributeDetailInfo {
                        id: Some("1".to_string()),
                        name: "속성1".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("2".to_string()),
                        name: "속성2".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("3".to_string()),
                        name: "속성3".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("4".to_string()),
                        name: "속성4".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("5".to_string()),
                        name: "속성5".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("6".to_string()),
                        name: "속성6".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("7".to_string()),
                        name: "속성7".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("8".to_string()),
                        name: "속성8".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("9".to_string()),
                        name: "속성9".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("10".to_string()),
                        name: "속성10".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("11".to_string()),
                        name: "속성11".to_string(),
                    },
                ],
            },
            AttributeSummary {
                id: "2".to_string(),
                name: "성별".to_string(),
                attribute: vec![
                    PanelAttributeDetailInfo {
                        id: Some("1".to_string()),
                        name: "속성1".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("2".to_string()),
                        name: "속성2".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("3".to_string()),
                        name: "속성3".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("4".to_string()),
                        name: "속성4".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("5".to_string()),
                        name: "속성5".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("6".to_string()),
                        name: "속성6".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("7".to_string()),
                        name: "속성7".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("8".to_string()),
                        name: "속성8".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("9".to_string()),
                        name: "속성9".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("10".to_string()),
                        name: "속성10".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("11".to_string()),
                        name: "속성11".to_string(),
                    },
                ],
            },
            AttributeSummary {
                id: "3".to_string(),
                name: "나이".to_string(),
                attribute: vec![
                    PanelAttributeDetailInfo {
                        id: Some("1".to_string()),
                        name: "속성1".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("2".to_string()),
                        name: "속성2".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("3".to_string()),
                        name: "속성3".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("4".to_string()),
                        name: "속성4".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("5".to_string()),
                        name: "속성5".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("6".to_string()),
                        name: "속성6".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("7".to_string()),
                        name: "속성7".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("8".to_string()),
                        name: "속성8".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("9".to_string()),
                        name: "속성9".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("10".to_string()),
                        name: "속성10".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("11".to_string()),
                        name: "속성11".to_string(),
                    },
                ],
            },
            AttributeSummary {
                id: "4".to_string(),
                name: "학력".to_string(),
                attribute: vec![
                    PanelAttributeDetailInfo {
                        id: Some("1".to_string()),
                        name: "속성1".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("2".to_string()),
                        name: "속성2".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("3".to_string()),
                        name: "속성3".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("4".to_string()),
                        name: "속성4".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("5".to_string()),
                        name: "속성5".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("6".to_string()),
                        name: "속성6".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("7".to_string()),
                        name: "속성7".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("8".to_string()),
                        name: "속성8".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("9".to_string()),
                        name: "속성9".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("10".to_string()),
                        name: "속성10".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("11".to_string()),
                        name: "속성11".to_string(),
                    },
                ],
            },
            AttributeSummary {
                id: "5".to_string(),
                name: "거주지".to_string(),
                attribute: vec![
                    PanelAttributeDetailInfo {
                        id: Some("1".to_string()),
                        name: "속성1".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("2".to_string()),
                        name: "속성2".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("3".to_string()),
                        name: "속성3".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("4".to_string()),
                        name: "속성4".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("5".to_string()),
                        name: "속성5".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("6".to_string()),
                        name: "속성6".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("7".to_string()),
                        name: "속성7".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("8".to_string()),
                        name: "속성8".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("9".to_string()),
                        name: "속성9".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("10".to_string()),
                        name: "속성10".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("11".to_string()),
                        name: "속성11".to_string(),
                    },
                ],
            },
            AttributeSummary {
                id: "6".to_string(),
                name: "국적".to_string(),
                attribute: vec![
                    PanelAttributeDetailInfo {
                        id: Some("1".to_string()),
                        name: "속성1".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("2".to_string()),
                        name: "속성2".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("3".to_string()),
                        name: "속성3".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("4".to_string()),
                        name: "속성4".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("5".to_string()),
                        name: "속성5".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("6".to_string()),
                        name: "속성6".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("7".to_string()),
                        name: "속성7".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("8".to_string()),
                        name: "속성8".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("9".to_string()),
                        name: "속성9".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("10".to_string()),
                        name: "속성10".to_string(),
                    },
                    PanelAttributeDetailInfo {
                        id: Some("11".to_string()),
                        name: "속성11".to_string(),
                    },
                ],
            },
        ];

        //FIXME: fix to api
        let panels = vec![
            PanelSummary {
                id: "1".to_string(),
                name: "패널명1".to_string(),
                count: 50,
                attribute: vec![
                    PanelAttributeInfo {
                        id: Some("1".to_string()),
                        name: "직업".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("2".to_string()),
                        name: "성별".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("3".to_string()),
                        name: "나이".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("4".to_string()),
                        name: "학력".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("5".to_string()),
                        name: "거주지".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("6".to_string()),
                        name: "국적".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                ],
            },
            PanelSummary {
                id: "2".to_string(),
                name: "패널명2".to_string(),
                count: 50,
                attribute: vec![
                    PanelAttributeInfo {
                        id: Some("1".to_string()),
                        name: "직업".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("2".to_string()),
                        name: "성별".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("3".to_string()),
                        name: "나이".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("4".to_string()),
                        name: "학력".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("5".to_string()),
                        name: "거주지".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("6".to_string()),
                        name: "국적".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                ],
            },
            PanelSummary {
                id: "3".to_string(),
                name: "패널명3".to_string(),
                count: 50,
                attribute: vec![
                    PanelAttributeInfo {
                        id: Some("1".to_string()),
                        name: "직업".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("2".to_string()),
                        name: "성별".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("3".to_string()),
                        name: "나이".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("4".to_string()),
                        name: "학력".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("5".to_string()),
                        name: "거주지".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("6".to_string()),
                        name: "국적".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                ],
            },
            PanelSummary {
                id: "4".to_string(),
                name: "패널명4".to_string(),
                count: 50,
                attribute: vec![
                    PanelAttributeInfo {
                        id: Some("1".to_string()),
                        name: "직업".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("2".to_string()),
                        name: "성별".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("3".to_string()),
                        name: "나이".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("4".to_string()),
                        name: "학력".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("5".to_string()),
                        name: "거주지".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("6".to_string()),
                        name: "국적".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                ],
            },
            PanelSummary {
                id: "5".to_string(),
                name: "패널명5".to_string(),
                count: 50,
                attribute: vec![
                    PanelAttributeInfo {
                        id: Some("1".to_string()),
                        name: "직업".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("2".to_string()),
                        name: "성별".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("3".to_string()),
                        name: "나이".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("4".to_string()),
                        name: "학력".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("5".to_string()),
                        name: "거주지".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("6".to_string()),
                        name: "국적".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                ],
            },
            PanelSummary {
                id: "6".to_string(),
                name: "패널명6".to_string(),
                count: 50,
                attribute: vec![
                    PanelAttributeInfo {
                        id: Some("1".to_string()),
                        name: "직업".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("2".to_string()),
                        name: "성별".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("3".to_string()),
                        name: "나이".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("4".to_string()),
                        name: "학력".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("5".to_string()),
                        name: "거주지".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("6".to_string()),
                        name: "국적".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                ],
            },
            PanelSummary {
                id: "7".to_string(),
                name: "패널명7".to_string(),
                count: 50,
                attribute: vec![
                    PanelAttributeInfo {
                        id: Some("1".to_string()),
                        name: "직업".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("2".to_string()),
                        name: "성별".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("3".to_string()),
                        name: "나이".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("4".to_string()),
                        name: "학력".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("5".to_string()),
                        name: "거주지".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                    PanelAttributeInfo {
                        id: Some("6".to_string()),
                        name: "국적".to_string(),
                        attribute: vec![
                            PanelAttributeDetailInfo {
                                id: Some("1".to_string()),
                                name: "속성1".to_string(),
                            },
                            PanelAttributeDetailInfo {
                                id: Some("2".to_string()),
                                name: "속성2".to_string(),
                            },
                        ],
                    },
                ],
            },
        ];
        let ctrl = Self {
            panels: use_signal(|| panels),
            attributes: use_signal(|| attributes),
        };
        ctrl
    }

    pub fn get_panels(&self) -> Vec<PanelSummary> {
        (self.panels)()
    }

    pub fn get_attributes(&self) -> Vec<AttributeSummary> {
        (self.attributes)()
    }
}
