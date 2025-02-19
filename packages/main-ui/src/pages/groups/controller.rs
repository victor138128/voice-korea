use dioxus::prelude::*;

use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    prelude::{CreateGroupRequest, TeamMemberRequest},
    GroupV2, GroupV2CreateRequest, GroupV2Query, GroupV2Summary, QueryResponse,
};

use crate::service::{
    group_api::GroupApi, login_service::LoginService, popup_service::PopupService,
};

use super::{
    i18n::GroupTranslate,
    page::{CreateGroupModal, RemoveGroupModal, UpdateGroupNameModal},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupMemberSummary {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<&models::prelude::GroupMemberResponse> for GroupMemberSummary {
    fn from(response: &models::prelude::GroupMemberResponse) -> Self {
        GroupMemberSummary {
            id: response.id.clone(),
            name: response.user_name.clone(),
            email: response.user_email.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupSummary {
    pub group_id: String,
    pub group_name: String,
    pub member_count: u64,
    pub member_list: Vec<GroupMemberSummary>,
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub popup_service: Signal<PopupService>,
    pub group_resource: Resource<QueryResponse<GroupV2Summary>>,
    pub org_id: Signal<i64>,
}

impl Controller {
    pub fn init(_lang: Language, popup_service: PopupService) -> Result<Self, RenderError> {
        let login_service: LoginService = use_context();
        let page = use_signal(|| 1);
        let size = 10;
        let org_id = login_service.get_selected_org().unwrap_or_default().id;

        let group_resource = use_server_future(move || {
            let page = page();
            async move {
                let client = GroupV2::get_client(&crate::config::get().api_url);
                let query = GroupV2Query::new(size).with_page(page);
                client.query(org_id, query).await.unwrap_or_default()
            }
        })?;

        let ctrl = Self {
            group_resource,
            popup_service: use_signal(|| popup_service),
            org_id: use_signal(|| org_id),
        };

        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub fn get_org_id(&self) -> i64 {
        (self.org_id)()
    }

    pub fn get_groups(&self) -> Vec<GroupV2Summary> {
        self.group_resource.with(|v| {
            if let Some(v) = v {
                v.items.clone()
            } else {
                vec![]
            }
        })
    }

    pub async fn remove_group_member(&mut self, _group_id: i64, _member_id: i64) {
        let _api: GroupApi = use_context();
        let _group_resource = self.group_resource;

        // match api.remove_team_member(group_id, member_id).await {
        //     Ok(_) => {
        //         group_resource.restart();
        //     }
        //     Err(e) => {
        //         tracing::error!("failed to remove team member: {e}");
        //     }
        // };
    }

    pub async fn create_group(&mut self, req: CreateGroupRequest) {
        let api: GroupApi = use_context();
        match api.create_group(req).await {
            Ok(_) => self.group_resource.restart(),
            Err(e) => {
                tracing::error!("failed to create group: {e}");
            }
        }
    }

    pub async fn invite_team_member(
        &mut self,
        group_id: String,
        email: String,
        name: Option<String>,
    ) {
        let api: GroupApi = use_context();
        match api
            .add_team_member(
                group_id,
                TeamMemberRequest {
                    email,
                    name,
                    group: None,
                    role: None,
                },
            )
            .await
        {
            Ok(_) => {
                self.group_resource.restart();
            }
            Err(e) => {
                tracing::error!("failed to invite team member: {e}");
            }
        };
    }

    pub async fn remove_group(&mut self, group_id: String) {
        let api: GroupApi = use_context();
        match api.remove_group(group_id).await {
            Ok(_) => self.group_resource.restart(),
            Err(e) => {
                tracing::error!("failed to remove group: {e}");
            }
        };
    }

    pub async fn update_group_name(
        &mut self,
        api: &GroupApi,
        group_id: String,
        group_name: String,
    ) {
        match api.update_group_name(group_id, group_name).await {
            Ok(_) => self.group_resource.restart(),
            Err(e) => {
                tracing::error!("failed to update group name: {e}");
            }
        };
    }

    pub async fn open_update_group_name_modal(
        &self,
        lang: Language,
        mut clicked_group_id: Signal<String>,
        mut clicked_group_name: Signal<String>,
    ) {
        let client = GroupV2::get_client(&crate::config::get().api_url);
        let mut popup_service = (self.popup_service)().clone();
        let translates: GroupTranslate = translate(&lang);

        let mut group_resource = self.group_resource;
        let groups = self.get_groups();
        let group: Vec<GroupV2Summary> = groups
            .iter()
            .filter(|v| v.id == clicked_group_id().parse::<i64>().unwrap())
            .map(|v| v.clone())
            .collect();
        let group = group.first().unwrap().clone();

        popup_service
            .open(rsx! {
                UpdateGroupNameModal {
                    lang,
                    onclose: move |_e: MouseEvent| {
                        clicked_group_id.set("".to_string());
                        clicked_group_name.set("".to_string());
                        popup_service.close();
                    },
                    initialize_group_name: clicked_group_name(),
                    update_group_name: {
                        let client = client.clone();
                        move |group_name: String| {
                            let client = client.clone();
                            async move {
                                match client.update(group.org_id, group.id, group_name.clone()).await {
                                    Ok(_) => group_resource.restart(),
                                    Err(e) => {
                                        tracing::error!("failed to update group name: {e}");
                                    }
                                };
                                clicked_group_id.set("".to_string());
                                clicked_group_name.set("".to_string());
                                popup_service.close();
                            }
                        }
                    },
                }
            })
            .with_id("update_group")
            .with_title(translates.update_group_name);
    }

    pub async fn open_remove_group_modal(
        &self,
        lang: Language,
        mut clicked_group_id: Signal<String>,
        mut clicked_group_name: Signal<String>,
    ) {
        let _client = GroupV2::get_client(&crate::config::get().api_url);
        let mut popup_service = (self.popup_service)().clone();
        let translates: GroupTranslate = translate(&lang);

        let _group_resource = self.group_resource;
        let groups = self.get_groups();
        let group: Vec<GroupV2Summary> = groups
            .iter()
            .filter(|v| v.id == clicked_group_id().parse::<i64>().unwrap())
            .map(|v| v.clone())
            .collect();
        let _group = group.first().unwrap().clone();

        //TODO: implement remove groups
        popup_service
            .open(rsx! {
                RemoveGroupModal {
                    lang,
                    onclose: move |_e: MouseEvent| {
                        clicked_group_id.set("".to_string());
                        clicked_group_name.set("".to_string());
                        popup_service.close();
                    },
                    remove_group: move |_e: Event<MouseData>| {},
                }
            })
            .with_id("remove_group")
            .with_title(translates.remove_group);

        // popup_service
        //     .open(rsx! {
        //         RemoveGroupModal {
        //             lang,
        //             onclose: move |_e: MouseEvent| {
        //                 clicked_group_id.set("".to_string());
        //                 clicked_group_name.set("".to_string());
        //                 popup_service.close();
        //             },
        //             remove_group: move |_e: Event<MouseData>| {
        //                 let client = client.clone();
        //                 async move {
        //                     match client.remove_group_member(clicked_group_id()).await {
        //                         Ok(_) => group_resource.restart(),
        //                         Err(e) => {
        //                             tracing::error!("failed to remove group: {e}");
        //                         }
        //                     };
        //                     clicked_group_id.set("".to_string());
        //                     clicked_group_name.set("".to_string());
        //                     popup_service.close();
        //                 }
        //             },
        //         }
        //     })
        //     .with_id("remove_group")
        //     .with_title(translates.remove_group);
    }

    pub async fn open_create_group_modal(
        &self,
        lang: Language,
        mut clicked_group_id: Signal<String>,
        mut clicked_group_name: Signal<String>,
    ) {
        let client = GroupV2::get_client(&crate::config::get().api_url);
        let mut popup_service = (self.popup_service)().clone();
        let translates: GroupTranslate = translate(&lang);

        let mut group_resource = self.group_resource;
        let org_id = self.get_org_id();

        popup_service
            .open(rsx! {
                CreateGroupModal {
                    lang,
                    members: vec![], //FIXME: fix to connect member api
                    oncreate: {
                        let client = client.clone();
                        move |req: GroupV2CreateRequest| {
                            let client = client.clone();
                            async move {
                                let _ = match client.create(org_id, req.name, req.users).await {
                                    Ok(_) => group_resource.restart(),
                                    Err(e) => {
                                        tracing::error!("failed to create group: {e}");
                                    }
                                };
                                popup_service.close();
                            }
                        }
                    },
                    onclose: move |_e: MouseEvent| {
                        clicked_group_id.set("".to_string());
                        clicked_group_name.set("".to_string());
                        popup_service.close();
                    },
                }
            })
            .with_id("create_group")
            .with_title(translates.create_group);
    }
}
