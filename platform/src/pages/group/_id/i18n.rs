use crate::utils::context::Language;

pub struct GroupDetailTranslate {
    pub organization_management: String,
    pub group_management: String,
    pub see_detail: String,
    pub register_date: String,

    pub group_team_member: String,
    pub add_member: String,
    pub name: String,
    pub group: String,
    pub role: String,
    pub project: String,

    pub common_project: String,
    pub add_project: String,
    pub item: String,
    pub panel: String,
    pub period: String,
    pub status: String,
}

pub fn translate(lang: Language) -> GroupDetailTranslate {
    match lang {
        Language::En => GroupDetailTranslate {
            organization_management: "Organization Management".to_string(),
            group_management: "Group Management".to_string(),
            see_detail: "See Detail".to_string(),
            register_date: "Register Date".to_string(),

            group_team_member: "Group Team Member".to_string(),
            add_member: "Add Member".to_string(),
            name: "Name".to_string(),
            group: "Group".to_string(),
            role: "Role".to_string(),
            project: "Project".to_string(),

            common_project: "Common Project".to_string(),
            add_project: "Create Project".to_string(),
            item: "Item".to_string(),
            panel: "Panel".to_string(),
            period: "Period".to_string(),
            status: "Status".to_string(),
        },
        Language::Ko => GroupDetailTranslate {
            organization_management: "조직관리".to_string(),
            group_management: "그룹 관리".to_string(),
            see_detail: "자세히 보기".to_string(),
            register_date: "등록된 날짜".to_string(),

            group_team_member: "그룹 팀원".to_string(),
            add_member: "팀원 추가하기".to_string(),
            name: "이름".to_string(),
            group: "그룹".to_string(),
            role: "역할".to_string(),
            project: "프로젝트".to_string(),

            common_project: "공통 프로젝트".to_string(),
            add_project: "프로젝트 추가하기".to_string(),
            item: "항목".to_string(),
            panel: "패널".to_string(),
            period: "기간".to_string(),
            status: "상태".to_string(),
        },
    }
}
