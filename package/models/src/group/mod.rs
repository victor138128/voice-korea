use serde::{Deserialize, Serialize};

use crate::member::MemberProject;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupMember {
    pub user_id: String,
    pub name: String,
    pub email: String,

    pub group: Option<String>,
    pub role: Option<String>,
    pub projects: Option<Vec<MemberProject>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GroupProject {
    pub project_id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Group {
    pub id: String,
    pub r#type: String,
    pub gsi1: String,
    pub creator: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub name: String,
    pub members: Vec<GroupMember>,                  //그룹 내 팀원
    pub public_opinion_projects: Vec<GroupProject>, //공론 프로젝트
    pub investigation_projects: Vec<GroupProject>,  //조사 프로젝트
}

impl Group {
    pub fn new(id: String, user_id: String) -> Self {
        let mut group = Group::default();
        let now = chrono::Utc::now().timestamp_millis();
        group.id = id;
        group.r#type = Group::get_type();
        group.gsi1 = Group::get_gsi1(&user_id);
        group.creator = user_id;
        group.created_at = now;
        group.updated_at = now;
        group.deleted_at = None;
        group
    }

    pub fn get_gsi1(user_id: &str) -> String {
        format!("{}#{}", Self::get_type(), user_id)
    }

    pub fn get_gsi_deleted(user_id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), user_id)
    }

    pub fn get_deleted_type() -> String {
        "deleted#group".to_string()
    }

    pub fn get_type() -> String {
        "group".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateGroupRequest {
    pub name: String,
    pub members: Vec<GroupMember>,                  //그룹 내 팀원
    pub public_opinion_projects: Vec<GroupProject>, //공론 프로젝트
    pub investigation_projects: Vec<GroupProject>,  //조사 프로젝트
}

impl Into<Group> for (CreateGroupRequest, String, String) {
    fn into(self) -> Group {
        let (req, id, user_id) = self;
        let now = chrono::Utc::now().timestamp_millis();

        Group {
            id,
            r#type: Group::get_type(),
            gsi1: Group::get_gsi1(&user_id),
            creator: user_id,
            created_at: now,
            updated_at: now,
            deleted_at: None,
            name: req.name,
            members: req.members,
            public_opinion_projects: req.public_opinion_projects,
            investigation_projects: req.investigation_projects,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupActionRequest {
    UpdateName(String),
    Delete,
}
