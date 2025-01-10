use chrono::{Local, TimeZone};
use dioxus::prelude::*;
use models::prelude::GroupMemberResponse;

use crate::service::group_api::GroupApi;

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
    pub group_members: Vec<GroupMemberResponse>,
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

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub group: Signal<GroupDetail>,
    pub groups: Signal<Vec<String>>,
    pub roles: Signal<Vec<String>>,
    pub group_resource: Resource<Result<models::prelude::GroupResponse, ServerFnError>>,
}

impl Controller {
    pub fn init(_lang: dioxus_translate::Language, group_id: String) -> Self {
        let api: GroupApi = use_context();
        let group_resource: Resource<Result<models::prelude::GroupResponse, ServerFnError>> =
            use_resource(move || {
                let api = api.clone();
                let group_id = group_id.clone();
                async move { api.get_group(group_id.clone()).await }
            });
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
            group_resource,
        };
        ctrl.groups.set(vec![
            "보이스코리아".to_string(),
            "보이스코리아1".to_string(),
            "보이스코리아2".to_string(),
            "보이스코리아3".to_string(),
        ]);

        let group = if let Some(v) = group_resource.value()() {
            match v {
                Ok(d) => {
                    let seconds = d.created_at / 1000;
                    let nanoseconds = (d.created_at % 1000) * 1_000_000;
                    let datetime = Local.timestamp_opt(seconds, nanoseconds as u32).unwrap();

                    let formatted_date = datetime.format("%Y년 %m월 %d일").to_string();

                    let data = GroupDetail {
                        group: d.name.clone(),
                        register_date: formatted_date.clone(),
                        group_members: d.members,
                        group_projects: vec![],
                    };

                    data
                }
                Err(_) => GroupDetail::default(),
            }
        } else {
            GroupDetail::default()
        };

        ctrl.group.set(group);

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

    pub async fn remove_group(&mut self, group_id: String) {
        let api: GroupApi = use_context();
        let _ = api.remove_group(group_id).await;
        self.group_resource.restart();
    }

    pub async fn update_group_name(&mut self, group_id: String, group_name: String) {
        let api: GroupApi = use_context();
        let _ = api.update_group_name(group_id, group_name).await;
        self.group_resource.restart();
    }
}
