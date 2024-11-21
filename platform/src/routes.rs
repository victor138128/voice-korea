use dioxus::prelude::*;

use crate::pages::create::CreatePage;
use crate::pages::dashboard::DashboardPage;
use crate::pages::find_email::FindEmailPage;
use crate::pages::id::select_response::response_type::SelectResponseDetailPage;
use crate::pages::id::select_response::SelectResponsePage;
use crate::pages::id::survey_summary::SurveySummaryPage;
use crate::pages::id::write_question::WriteQuestionPage;
use crate::pages::id::write_title::WriteTitlePage;
use crate::pages::login::LoginPage;
use crate::pages::reset_password::ResetPasswordPage;
use crate::prelude::*;
use crate::utils::context::{default_lang, Language};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/dashboard")]
            DashboardPage { lang: Language },
            #[route("/id/:survey_id/write-title")]
            WriteTitlePage { lang: Language, survey_id: String },
            #[route("/id/:survey_id/write-question")]
            WriteQuestionPage { lang: Language, survey_id: String },
            #[route("/id/:survey_id/select-response")]
            SelectResponsePage { lang: Language, survey_id: String },
            #[route("/:title/select-response/type/:select_type")]
            SelectResponseDetailPage { lang: Language, title: String, select_type: String },
            #[route("/:title/survey-summary")]
            SurveySummaryPage { lang: Language, title: String },
        #[end_layout]

        #[route("/")]
        LoginPage { lang: Language },
        #[route("/create")]
        CreatePage { lang: Language },
        #[route("/find-email")]
        FindEmailPage { lang: Language },
        #[route("/reset-password")]
        ResetPasswordPage { lang: Language },
    #[end_nest]

    #[redirect("/", || Route::LoginPage { lang: default_lang() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
