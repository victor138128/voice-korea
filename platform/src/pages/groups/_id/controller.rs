use dioxus::prelude::*;

use crate::prelude::Language;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProjectType {
    #[default]
    Investigation, //조사
    PublicOpinion, //공론
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProjectStatus {
    #[default]
    Ready, //준비
    InProgress, //진행
    Finished,   //마감
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupDetail {
    pub group: String,
    pub register_date: String,
    pub group_members: Vec<GroupMember>,
    pub group_projects: Vec<GroupProject>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupProject {
    pub project_type: ProjectType,
    pub project_subject: String,
    pub panels: Vec<String>,
    pub periods: String, //FIXME: fix to start date, end date format
    pub project_status: ProjectStatus,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupMember {
    pub member_id: String,
    pub email: String,
    pub profile_image: Option<String>,
    pub profile_name: Option<String>,
    pub group: String,
    pub role: String,
    pub projects: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Controller {
    pub group: Signal<GroupDetail>,
    pub groups: Signal<Vec<String>>,
    pub roles: Signal<Vec<String>>,
}

impl Controller {
    pub fn init(_lang: Language, _group_id: String) -> Self {
        let mut ctrl = Self {
            group: use_signal(|| GroupDetail::default()),
            groups: use_signal(|| vec![]),
            roles: use_signal(|| {
                vec![
                    "관리자".to_string(),
                    "공론 관리자".to_string(),
                    "분석가".to_string(),
                    "중계자".to_string(),
                    "강연자".to_string(),
                ]
            }),
        };
        ctrl.groups.set(vec![
            "보이스코리아".to_string(),
            "보이스코리아1".to_string(),
            "보이스코리아2".to_string(),
            "보이스코리아3".to_string(),
        ]);

        ctrl.group.set(GroupDetail {
            group: "보이스코리아".to_string(),
            register_date: "2024년 8월 12일".to_string(),
            group_members: vec![
                GroupMember {
                    member_id: "1".to_string(),
                    email: "email@email.com".to_string(),
                    profile_image: None,
                    profile_name: Some("보이스".to_string()),
                    group: "보이스코리아".to_string(),
                    role: "관리자".to_string(),
                    projects: vec!["공론주제".to_string(), "조사주제".to_string()],
                },
                GroupMember {
                    member_id: "2".to_string(),
                    email: "email@email.com".to_string(),
                    profile_image: None,
                    profile_name: Some("보이스".to_string()),
                    group: "보이스코리아".to_string(),
                    role: "관리자".to_string(),
                    projects: vec!["공론주제".to_string(), "조사주제".to_string()],
                },
                GroupMember {
                    member_id: "3".to_string(),
                    email: "email@email.com".to_string(),
                    profile_image: None,
                    profile_name: Some("보이스".to_string()),
                    group: "보이스코리아".to_string(),
                    role: "관리자".to_string(),
                    projects: vec!["공론주제".to_string(), "조사주제".to_string()],
                },
                GroupMember {
                    member_id: "4".to_string(),
                    email: "email@email.com".to_string(),
                    profile_image: None,
                    profile_name: Some("보이스".to_string()),
                    group: "보이스코리아".to_string(),
                    role: "관리자".to_string(),
                    projects: vec!["공론주제".to_string(), "조사주제".to_string()],
                },
            ],
            group_projects: vec![
                GroupProject {
                    project_type: ProjectType::Investigation,
                    project_subject: "조사주제".to_string(),
                    panels: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    periods: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Finished,
                },
                GroupProject {
                    project_type: ProjectType::PublicOpinion,
                    project_subject: "공론주제".to_string(),
                    panels: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    periods: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Finished,
                },
                GroupProject {
                    project_type: ProjectType::PublicOpinion,
                    project_subject: "공론주제".to_string(),
                    panels: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    periods: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Finished,
                },
                GroupProject {
                    project_type: ProjectType::Investigation,
                    project_subject: "조사주제".to_string(),
                    panels: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    periods: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Finished,
                },
                GroupProject {
                    project_type: ProjectType::Investigation,
                    project_subject: "조사주제".to_string(),
                    panels: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    periods: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Finished,
                },
                GroupProject {
                    project_type: ProjectType::Investigation,
                    project_subject: "조사주제".to_string(),
                    panels: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    periods: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Finished,
                },
                GroupProject {
                    project_type: ProjectType::Investigation,
                    project_subject: "조사주제".to_string(),
                    panels: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    periods: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Finished,
                },
                GroupProject {
                    project_type: ProjectType::Investigation,
                    project_subject: "조사주제".to_string(),
                    panels: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    periods: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Finished,
                },
                GroupProject {
                    project_type: ProjectType::Investigation,
                    project_subject: "조사주제".to_string(),
                    panels: vec![
                        "패널1".to_string(),
                        "패널2".to_string(),
                        "패널3".to_string(),
                    ],
                    periods: "2025.10.01 ~ 2025.12.02".to_string(),
                    project_status: ProjectStatus::Finished,
                },
            ],
        });

        ctrl
    }

    pub fn get_group(&self) -> GroupDetail {
        (self.group)()
    }

    pub fn get_groups(&self) -> Vec<String> {
        (self.groups)()
    }

    pub fn get_roles(&self) -> Vec<String> {
        (self.roles)()
    }
}
