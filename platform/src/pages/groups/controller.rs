use dioxus::prelude::*;

use dioxus_translate::Language;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupSummary {
    pub group_id: String,
    pub group_name: String,
    pub member_count: u64,
    pub member_list: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
    pub groups: Signal<Vec<GroupSummary>>,
}

impl Controller {
    pub fn init(_lang: Language) -> Self {
        let mut ctrl = Self {
            groups: use_signal(|| vec![]),
        };

        ctrl.groups.set(vec![
            GroupSummary {
                group_id: "1".to_string(),
                group_name: "보이스코리아".to_string(),
                member_count: 10,
                member_list: vec![
                    "보이스".to_string(),
                    "보이스1".to_string(),
                    "보이스2".to_string(),
                    "보이스3".to_string(),
                    "보이스4".to_string(),
                    "보이스5".to_string(),
                    "보이스6".to_string(),
                    "보이스7".to_string(),
                    "보이스8".to_string(),
                    "보이스9".to_string(),
                ],
            },
            GroupSummary {
                group_id: "2".to_string(),
                group_name: "보이스코리아".to_string(),
                member_count: 5,
                member_list: vec![
                    "보이스".to_string(),
                    "보이스1".to_string(),
                    "보이스2".to_string(),
                    "보이스3".to_string(),
                    "보이스4".to_string(),
                ],
            },
            GroupSummary {
                group_id: "3".to_string(),
                group_name: "보이스코리아".to_string(),
                member_count: 3,
                member_list: vec![
                    "보이스".to_string(),
                    "보이스1".to_string(),
                    "보이스2".to_string(),
                ],
            },
            GroupSummary {
                group_id: "4".to_string(),
                group_name: "보이스코리아".to_string(),
                member_count: 10,
                member_list: vec![
                    "보이스".to_string(),
                    "보이스1".to_string(),
                    "보이스2".to_string(),
                    "보이스3".to_string(),
                    "보이스4".to_string(),
                    "보이스5".to_string(),
                    "보이스6".to_string(),
                    "보이스7".to_string(),
                    "보이스8".to_string(),
                    "보이스9".to_string(),
                ],
            },
            GroupSummary {
                group_id: "5".to_string(),
                group_name: "보이스코리아".to_string(),
                member_count: 7,
                member_list: vec![
                    "보이스".to_string(),
                    "보이스1".to_string(),
                    "보이스2".to_string(),
                    "보이스3".to_string(),
                    "보이스4".to_string(),
                    "보이스5".to_string(),
                    "보이스6".to_string(),
                ],
            },
            GroupSummary {
                group_id: "6".to_string(),
                group_name: "보이스코리아".to_string(),
                member_count: 6,
                member_list: vec![
                    "보이스".to_string(),
                    "보이스1".to_string(),
                    "보이스2".to_string(),
                    "보이스3".to_string(),
                    "보이스4".to_string(),
                    "보이스5".to_string(),
                ],
            },
            GroupSummary {
                group_id: "7".to_string(),
                group_name: "보이스코리아".to_string(),
                member_count: 10,
                member_list: vec![
                    "보이스".to_string(),
                    "보이스1".to_string(),
                    "보이스2".to_string(),
                    "보이스3".to_string(),
                    "보이스4".to_string(),
                    "보이스5".to_string(),
                    "보이스6".to_string(),
                    "보이스7".to_string(),
                    "보이스8".to_string(),
                    "보이스9".to_string(),
                ],
            },
            GroupSummary {
                group_id: "8".to_string(),
                group_name: "보이스코리아".to_string(),
                member_count: 10,
                member_list: vec![
                    "보이스".to_string(),
                    "보이스1".to_string(),
                    "보이스2".to_string(),
                    "보이스3".to_string(),
                    "보이스4".to_string(),
                    "보이스5".to_string(),
                    "보이스6".to_string(),
                    "보이스7".to_string(),
                    "보이스8".to_string(),
                    "보이스9".to_string(),
                ],
            },
            GroupSummary {
                group_id: "9".to_string(),
                group_name: "보이스코리아".to_string(),
                member_count: 10,
                member_list: vec![
                    "보이스".to_string(),
                    "보이스1".to_string(),
                    "보이스2".to_string(),
                    "보이스3".to_string(),
                    "보이스4".to_string(),
                    "보이스5".to_string(),
                    "보이스6".to_string(),
                    "보이스7".to_string(),
                    "보이스8".to_string(),
                    "보이스9".to_string(),
                ],
            },
            GroupSummary {
                group_id: "10".to_string(),
                group_name: "보이스코리아".to_string(),
                member_count: 10,
                member_list: vec![
                    "보이스".to_string(),
                    "보이스1".to_string(),
                    "보이스2".to_string(),
                    "보이스3".to_string(),
                    "보이스4".to_string(),
                    "보이스5".to_string(),
                    "보이스6".to_string(),
                    "보이스7".to_string(),
                    "보이스8".to_string(),
                    "보이스9".to_string(),
                ],
            },
        ]);

        ctrl
    }

    pub fn get_groups(&self) -> Vec<GroupSummary> {
        (self.groups)()
    }
}
