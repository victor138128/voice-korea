use dioxus::prelude::*;

use crate::prelude::*;
use crate::presentations::create::CreatePage;
use crate::presentations::dashboard::DashboardPage;
use crate::presentations::login::LoginPage;
use crate::utils::context::{default_lang, Language};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/home")]
            DashboardPage { lang: Language },
        #[end_layout]
        
        #[route("/")]
        LoginPage { lang: Language },

        #[route("/create")]
        CreatePage { lang: Language },

    #[end_nest]

    #[redirect("/", || Route::LoginPage { lang: default_lang() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
