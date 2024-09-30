use dioxus::prelude::*;

use crate::prelude::*;
use crate::presentations::create::CreatePage;
use crate::presentations::dashboard::DashboardPage;
use crate::presentations::find_email::FindEmailPage;
use crate::presentations::login::LoginPage;
use crate::presentations::reset_password::ResetPasswordPage;
use crate::presentations::write_question::WriteQuestionPage;
use crate::presentations::write_title::WriteTitlePage;
use crate::utils::context::{default_lang, Language};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/dashboard")]
            DashboardPage { lang: Language },
            #[route("/write-title")]
            WriteTitlePage { lang: Language },
            #[route("/write-question")]
            WriteQuestionPage { lang: Language },
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
