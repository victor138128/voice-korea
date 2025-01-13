use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationMemberResponse {
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub user_id: String,
    pub organization_id: String,
    pub organization_name: String,
    pub creator: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OrganizationMember {
    pub id: String,
    pub r#type: String,
    pub gsi1: String, //user_id
    pub gsi2: String, //user_id#organization_id
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub user_id: String,
    pub organization_id: String,
}

impl OrganizationMember {
    pub fn new(id: String, user_id: String, organization_id: String) -> Self {
        //uuid, user_id, organization_id
        let mut organization_member = OrganizationMember::default();
        let now = chrono::Utc::now().timestamp_millis();
        organization_member.id = id;
        organization_member.r#type = OrganizationMember::get_type();
        organization_member.gsi1 = OrganizationMember::get_gsi1(&user_id);
        organization_member.gsi2 = OrganizationMember::get_gsi2(&user_id, &organization_id);
        organization_member.created_at = now;
        organization_member.updated_at = now;
        organization_member.deleted_at = None;

        organization_member.user_id = user_id;
        organization_member.organization_id = organization_id;

        organization_member
    }

    pub fn get_gsi1(user_id: &str) -> String {
        format!("{}#{}", Self::get_type(), user_id)
    }

    pub fn get_gsi2(user_id: &str, organization_id: &str) -> String {
        format!("{}#{}#{}", Self::get_type(), user_id, organization_id)
    }

    pub fn get_gsi1_deleted(user_id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), user_id)
    }

    pub fn get_gsi2_deleted(user_id: &str, organization_id: &str) -> String {
        format!(
            "{}#{}#{}",
            Self::get_deleted_type(),
            user_id,
            organization_id
        )
    }

    pub fn get_deleted_type() -> String {
        "deleted#organization#member".to_string()
    }

    pub fn get_type() -> String {
        "organization#member".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Organization {
    pub id: String,
    pub r#type: String,
    pub gsi1: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub deleted_at: Option<i64>,

    pub name: String,
    pub user_id: String,
}

impl Organization {
    pub fn new(id: String, user_id: String, email_address: String) -> Self {
        let mut organization = Organization::default();
        let now = chrono::Utc::now().timestamp_millis();
        organization.id = id;
        organization.r#type = Organization::get_type();
        organization.gsi1 = Organization::get_gsi1(&email_address);
        organization.created_at = now;
        organization.updated_at = now;
        organization.deleted_at = None;
        organization.user_id = user_id;
        organization.name = email_address;

        organization
    }

    pub fn get_gsi1(id: &str) -> String {
        format!("{}#{}", Self::get_type(), id)
    }

    pub fn get_gsi1_deleted(id: &str) -> String {
        format!("{}#{}", Self::get_deleted_type(), id)
    }

    pub fn get_deleted_type() -> String {
        "deleted#organization".to_string()
    }

    pub fn get_type() -> String {
        "organization".to_string()
    }
}
