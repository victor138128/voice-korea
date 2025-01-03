use crate::utils::context::Language;

pub struct MemberTranslate {
    pub organization_management: String,
    pub team_member_management: String,

    pub total: String,
    pub manager: String,
    pub public_opinion_manager: String,
    pub analyst: String,
    pub repeater: String,
    pub lecturer: String,

    pub add_team_member: String,
    pub name: String,
    pub group: String,
    pub role: String,
    pub project: String,
}

pub fn translate(lang: Language) -> MemberTranslate {
    match lang {
        Language::En => MemberTranslate {
            organization_management: "Organization Management".to_string(),
            team_member_management: "Team Member Management".to_string(),

            total: "Total".to_string(),
            manager: "Manager".to_string(),
            public_opinion_manager: "Public Opinion Manager".to_string(),
            analyst: "Analyst".to_string(),
            repeater: "Repeater".to_string(),
            lecturer: "Lecturer".to_string(),

            add_team_member: "Add Team Member".to_string(),
            name: "Name".to_string(),
            group: "Group".to_string(),
            role: "Role".to_string(),
            project: "Project".to_string(),
        },
        Language::Ko => MemberTranslate {
            organization_management: "조직관리".to_string(),
            team_member_management: "팀원 관리".to_string(),

            total: "전체".to_string(),
            manager: "관리자".to_string(),
            public_opinion_manager: "공론 관리자".to_string(),
            analyst: "분석가".to_string(),
            repeater: "중계자".to_string(),
            lecturer: "강연자".to_string(),

            add_team_member: "팀원 추가하기".to_string(),
            name: "이름".to_string(),
            group: "그룹".to_string(),
            role: "역할".to_string(),
            project: "프로젝트".to_string(),
        },
    }
}
