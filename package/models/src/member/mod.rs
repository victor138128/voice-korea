use serde::{Deserialize, Serialize};
use crate::{
    organization::OrganizationMember,
    group::Group,
};
use crate::prelude::Role;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq)]
pub struct MemberProject {
    pub project_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Member {
    pub id: String,
    pub r#type: String,
    pub gsi1: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub email: String,
    pub name: Option<String>,
    pub group: Option<String>,
    pub role: Option<String>,
    //FIXME: implement project model sepalately after public opinion, investigation api implemented
    // pub projects: Option<Vec<MemberProject>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListMemberResponse {
    pub members: Vec<OrganizationMember>,
    pub role_count: Vec<u64>,
    pub bookmark: Option<String>,
}

// FIXME: depreciated data structure (-> OrganizationMember)
impl Member {
    pub fn new(
        id: String,
        email: String,
        name: Option<String>,
        group: Option<String>,
        role: Option<String>,
        // projects: Option<Vec<MemberProject>>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            r#type: Member::get_type(),
            id,
            gsi1: Member::get_gsi1(email.clone()),
            created_at: now,
            updated_at: now,
            deleted_at: None,

            email,
            name,
            group,
            role,
            // projects,
        }
    }

    pub fn get_gsi_deleted(email: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), email)
    }

    pub fn get_deleted_type() -> String {
        "deleted#member".to_string()
    }

    pub fn get_gsi1(email: String) -> String {
        format!("{}#{}", Self::get_type(), email)
    }

    pub fn get_type() -> String {
        "member".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InviteMember {
    pub id: String,
    pub r#type: String,
    pub gsi1: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub email: String,
    pub name: String,
    pub group: Option<GroupInfo>,
    pub role: Option<Role>,
    pub projects: Option<Vec<MemberProject>>, //FIXME: implement project model sepalately after public opinion, investigation api implemented
}

impl InviteMember {
    pub fn new(
        id: String,
        email: String,
        name: String,
        group: Option<GroupInfo>,
        role: Option<Role>,
        projects: Option<Vec<MemberProject>>,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_millis();
        Self {
            r#type: InviteMember::get_type(),
            id,
            gsi1: InviteMember::get_gsi1(email.clone()),
            created_at: now,
            updated_at: now,
            deleted_at: None,

            email,
            name,
            group,
            role,
            projects,
        }
    }

    pub fn get_gsi_deleted(email: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), email)
    }

    pub fn get_deleted_type() -> String {
        "deleted#member#invite".to_string()
    }

    pub fn get_gsi1(email: String) -> String {
        format!("{}#{}", Self::get_type(), email)
    }

    pub fn get_type() -> String {
        "member#invite".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq)]
pub struct InviteMemberRequest {
    pub user_id: String,
    pub email: String,
    pub name: String,
    pub group: Option<GroupInfo>,
    pub role: Option<Role>,
    pub projects: Option<Vec<MemberProject>>,
}

impl Into<InviteMember> for (InviteMemberRequest, String) {
    fn into(self) -> InviteMember {
        let (req, invite_id) = self;
        let now = chrono::Utc::now().timestamp_millis();

        InviteMember {
            id: invite_id,
            r#type: InviteMember::get_type(),
            gsi1: InviteMember::get_gsi1(req.email.clone()),
            created_at: now,
            updated_at: now,
            deleted_at: None,

            email: req.email,
            name: req.name,
            group: req.group,
            role: req.role,
            projects: req.projects,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateMemberRequest {
    pub user_id: String,
    pub org_id: String,
    pub name: Option<String>,
    pub group: Option<GroupInfo>,
    pub role: Option<Role>,
    // pub projects: Option<Vec<MemberProject>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq)]
pub struct GroupInfo {
    pub id: String,
    pub name: String,
}

impl TryFrom<Group> for GroupInfo {
    type Error = std::fmt::Error;

    fn try_from(group: Group) -> Result<Self, Self::Error> {
        Ok(Self {
            id: group.id,
            name: group.name,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq)]
pub struct UpdateMemberRequest {
    pub name: Option<String>,     //user_name
    pub group: Option<GroupInfo>, //group_id
    pub role: Option<String>,     //role_name
                                  // pub projects: Option<Vec<MemberProject>>,
}

impl Into<Member> for (CreateMemberRequest, String) {
    fn into(self) -> Member {
        let (req, id) = self;
        let now = chrono::Utc::now().timestamp_millis();

        Member {
            id,
            r#type: Member::get_type(),
            gsi1: Member::get_gsi1("".to_string()), // deprecated
            created_at: now,
            updated_at: now,
            deleted_at: None,
            email: "".to_string(), // deprecated
            name: req.name,
            group: if req.group.is_none() {
                None
            } else {
                Some(req.group.unwrap().name)
            },  
            role: Some("".to_string()), // deprecated
            // projects: req.projects,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberActionRequest {
    Update(UpdateMemberRequest),
    Delete,
    AddProject(MemberProject),
    RemoveProject(String), //project_id
}
