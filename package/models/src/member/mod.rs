use serde::{Deserialize, Serialize};

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
    pub group: Option<String>,
    pub role: Option<String>,
    pub projects: Option<Vec<MemberProject>>, //FIXME: implement project model sepalately after public opinion, investigation api implemented
}

impl InviteMember {
    pub fn new(
        id: String,
        email: String,
        name: String,
        group: Option<String>,
        role: Option<String>,
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
    pub email: String,
    pub name: String,
    pub group: Option<String>,
    pub role: Option<String>,
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
    pub email: String,
    pub name: Option<String>,
    pub group: Option<String>,
    pub role: Option<String>,
    // pub projects: Option<Vec<MemberProject>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq)]
pub struct UpdateMemberRequest {
    pub name: Option<String>,
    pub group: Option<String>,
    pub role: Option<String>,
    // pub projects: Option<Vec<MemberProject>>,
}

impl Into<Member> for (CreateMemberRequest, String) {
    fn into(self) -> Member {
        let (req, id) = self;
        let now = chrono::Utc::now().timestamp_millis();

        Member {
            id,
            r#type: Member::get_type(),
            gsi1: Member::get_gsi1(req.email.clone()),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            email: req.email,
            name: req.name,
            group: req.group,
            role: req.role,
            // projects: req.projects,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberActionRequest {
    Update(UpdateMemberRequest),
    Delete,
}
