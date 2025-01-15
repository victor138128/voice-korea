use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(Props, Clone, PartialEq)]
pub struct InputOpinionProps {
    lang: Language,
}

#[component]
pub fn InputOpinion(props: InputOpinionProps) -> Element {
    let _props = props;
    rsx! { "implement this ui" }
}
