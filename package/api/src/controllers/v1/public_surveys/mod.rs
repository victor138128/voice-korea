use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::{get, post},
        Json, Router,
    },
    log::root,
};
use slog::o;

use crate::{
    common::CommonQueryResponse, middleware::auth::authorization_middleware, utils::error::ApiError,
};

use models::prelude::*;

#[derive(Clone, Debug)]
pub struct PublicSurveyControllerV1 {
    log: slog::Logger,
}

#[derive(Debug, serde::Deserialize)]
pub struct Pagination {
    pub _size: Option<i32>,
    pub _bookmark: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchParams {
    pub _keyword: String,
}

impl PublicSurveyControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "PublicSurveyControllerV1"));
        let ctrl = PublicSurveyControllerV1 { log };

        //TODO: implement metadata uri
        Router::new()
            .route(
                "/organizations/:organization_id",
                post(Self::upsert_survey).get(Self::list_surveys),
            )
            .route(
                "/organizations/:organization_id/search/surveys",
                get(Self::search_survey),
            )
            .route(
                "/organizations/:organization_id/surveys/:survey_id",
                post(Self::act_survey).get(Self::get_survey),
            )
            .with_state(ctrl)
            .layer(middleware::from_fn(authorization_middleware))
    }

    pub async fn get_survey(
        State(ctrl): State<PublicSurveyControllerV1>,
        Path((organization_id, survey_id)): Path<(String, String)>,
    ) -> Result<Json<PublicSurveyResponse>, ApiError> {
        let log = ctrl.log.new(o!("api" => "get_survey"));
        slog::debug!(log, "get_survey: {:?} {:?}", organization_id, survey_id);

        Ok(Json(PublicSurveyResponse {
            id: "1".to_string(),
            statistics: PublicSurveyStatistics {
                total_members: 1720,
                response_members: 1454,
                participants_rate: 98,
                time_taken: "00:02:00".to_string(),
                remained_time: "20일".to_string(),
                start_date: 1759244400,
                end_date: 1764601200,
            },
            response_participant_rate_totals: PublicSurveyResponseParticipantRateTotals {
                panels: vec![
                    PublicSurveyResponsePanelInfo {
                        id: "1".to_string(),
                        name: "패널1".to_string(),
                        members: 700,
                        percents: 35,
                    },
                    PublicSurveyResponsePanelInfo {
                        id: "2".to_string(),
                        name: "패널2".to_string(),
                        members: 700,
                        percents: 35,
                    },
                    PublicSurveyResponsePanelInfo {
                        id: "3".to_string(),
                        name: "패널3".to_string(),
                        members: 700,
                        percents: 35,
                    },
                    PublicSurveyResponsePanelInfo {
                        id: "4".to_string(),
                        name: "패널4".to_string(),
                        members: 700,
                        percents: 35,
                    },
                ],
            },
            response_participant_rates: vec![
                PublicSurveyResponseParticipantRates {
                    question_id: "1".to_string(),
                    question_name: "질문1".to_string(),
                    panels: vec![
                        PublicSurveyResponsePanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "4".to_string(),
                            name: "패널4".to_string(),
                            members: 700,
                            percents: 35,
                        },
                    ],
                },
                PublicSurveyResponseParticipantRates {
                    question_id: "2".to_string(),
                    question_name: "질문2".to_string(),
                    panels: vec![
                        PublicSurveyResponsePanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "4".to_string(),
                            name: "패널4".to_string(),
                            members: 700,
                            percents: 35,
                        },
                    ],
                },
                PublicSurveyResponseParticipantRates {
                    question_id: "3".to_string(),
                    question_name: "질문3".to_string(),
                    panels: vec![
                        PublicSurveyResponsePanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                            members: 700,
                            percents: 35,
                        },
                        PublicSurveyResponsePanelInfo {
                            id: "4".to_string(),
                            name: "패널4".to_string(),
                            members: 700,
                            percents: 35,
                        },
                    ],
                },
            ],
            single_choice_statistics: SingleChoiceStatistics {
                totals: vec![
                    SingleChoiceInfo {
                        answer_name: "5시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                    SingleChoiceInfo {
                        answer_name: "3시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                    SingleChoiceInfo {
                        answer_name: "2시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                ],
                panels: vec![SingleChoicePanelInfo {
                    panel_id: "1".to_string(),
                    panel_name: "패널1".to_string(),
                    statistics: vec![
                        SingleChoiceInfo {
                            answer_name: "5시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                        SingleChoiceInfo {
                            answer_name: "3시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                        SingleChoiceInfo {
                            answer_name: "2시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                    ],
                }],
            },
            multiple_choice_statistics: MultipleChoiceStatistics {
                totals: vec![
                    MultipleChoiceInfo {
                        answer_name: "5시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                    MultipleChoiceInfo {
                        answer_name: "5시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                    MultipleChoiceInfo {
                        answer_name: "5시간 이상".to_string(),
                        response_count: 727,
                        response_rate: 35.08,
                    },
                ],
                panels: vec![MultipleChoicePanelInfo {
                    panel_id: "1".to_string(),
                    panel_name: "패널1".to_string(),
                    statistics: vec![
                        MultipleChoiceInfo {
                            answer_name: "5시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                        MultipleChoiceInfo {
                            answer_name: "5시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                        MultipleChoiceInfo {
                            answer_name: "5시간 이상".to_string(),
                            response_count: 727,
                            response_rate: 35.08,
                        },
                    ],
                }],
            },
            text_statistics: TextStatistics {
                totals: TextInfo {
                    most_used_keyword: vec!["Element".to_string(), "Element".to_string()],
                    include_keyword_answer: vec![
                        "저도 'Element'에 동의합니다.".to_string(),
                        "저도 'Element'에 동의합니다.".to_string(),
                        "저도 'Element'에 동의합니다.".to_string(),
                    ],
                },
                panels: vec![TextPanelInfo {
                    panel_id: "1".to_string(),
                    panel_name: "패널1".to_string(),
                    statistics: TextInfo {
                        most_used_keyword: vec!["Element".to_string(), "Element".to_string()],
                        include_keyword_answer: vec![
                            "저도 'Element'에 동의합니다.".to_string(),
                            "저도 'Element'에 동의합니다.".to_string(),
                            "저도 'Element'에 동의합니다.".to_string(),
                        ],
                    },
                }],
            },
            optional_statistics: OptionalStatistics {
                totals: OptionalInfo {
                    responses: vec![250, 0, 0, 0, 1000, 0, 0, 0, 0],
                    response_rates: vec![0.25, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0],
                },
                panels: vec![OptionalPanelInfo {
                    panel_id: "1".to_string(),
                    panel_name: "패널1".to_string(),
                    statistics: OptionalInfo {
                        responses: vec![250, 0, 0, 0, 1000, 0, 0, 0, 0],
                        response_rates: vec![0.25, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.0],
                    },
                }],
            },
        }))
    }

    pub async fn act_survey(
        State(ctrl): State<PublicSurveyControllerV1>,
        Path((organization_id, survey_id)): Path<(String, String)>,
        Json(body): Json<SurveyActionRequest>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "act_survey"));
        slog::debug!(log, "act_survey: {:?} {:?}", organization_id, survey_id);

        match body {
            SurveyActionRequest::Delete => {
                ctrl.remove_survey(&organization_id, &survey_id).await?;
            }
        }

        Ok(())
    }

    pub async fn upsert_survey(
        State(ctrl): State<PublicSurveyControllerV1>,
        Path(organization_id): Path<String>,
        Json(body): Json<UpsertPublicSurveyRequest>,
    ) -> Result<Json<UpsertPublicSurveyRequest>, ApiError> {
        let log = ctrl.log.new(o!("api" => "upsert_survey"));
        slog::debug!(log, "upsert_survey {:?} {:?}", organization_id, body);
        Ok(Json(UpsertPublicSurveyRequest::default()))
    }

    pub async fn search_survey(
        State(ctrl): State<PublicSurveyControllerV1>,
        Path(organization_id): Path<String>,
        Query(params): Query<SearchParams>,
    ) -> Result<Json<CommonQueryResponse<PublicSurveySummary>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "list_surveys"));
        slog::debug!(log, "list_surveys: {:?} {:?}", organization_id, params);
        Ok(Json(CommonQueryResponse {
            items: vec![
                PublicSurveySummary {
                    id: "1".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: SurveyFieldType::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
                PublicSurveySummary {
                    id: "2".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: SurveyFieldType::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
                PublicSurveySummary {
                    id: "3".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: SurveyFieldType::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
            ],
            bookmark: None,
        }))
    }

    pub async fn list_surveys(
        Path(organization_id): Path<String>,
        State(ctrl): State<PublicSurveyControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<PublicSurveySummary>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "list_surveys"));
        slog::debug!(log, "list_surveys: {:?} {:?}", organization_id, pagination);
        Ok(Json(CommonQueryResponse {
            items: vec![
                PublicSurveySummary {
                    id: "1".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: SurveyFieldType::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
                PublicSurveySummary {
                    id: "2".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: SurveyFieldType::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
                PublicSurveySummary {
                    id: "3".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: SurveyFieldType::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
                PublicSurveySummary {
                    id: "4".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: SurveyFieldType::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
                PublicSurveySummary {
                    id: "5".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: SurveyFieldType::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
                PublicSurveySummary {
                    id: "6".to_string(),
                    survey_type: SurveyType::Survey,
                    survey_field_type: SurveyFieldType::Economy,
                    title: "조사주제".to_string(),
                    total_response: 60,
                    survey_response: 40,
                    panels: vec![],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: PublicSurveyStatus::Finish,
                },
            ],
            bookmark: None,
        }))
    }
}

impl PublicSurveyControllerV1 {
    pub async fn remove_survey(
        &self,
        organization_id: &str,
        survey_id: &str,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove_survey"));
        slog::debug!(log, "remove_survey {:?} {:?}", organization_id, survey_id);
        Ok(())
    }
}
