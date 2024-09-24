use dioxus::prelude::*;

use crate::prelude::*;
use crate::presentations::root::Root;
use crate::utils::context::{default_lang, Language};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/")]
            Root { lang: Language },
        #[end_layout]

    #[end_nest]

    #[redirect("/", || Route::Root { lang: default_lang() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
