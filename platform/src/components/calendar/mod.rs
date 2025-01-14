#![allow(non_snake_case)]
use chrono::{Datelike, TimeZone, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing;
mod controller;
use controller::Controller;
#[component]
pub fn Calendar(timestamp: Option<u64>, update_date: EventHandler<i64>) -> Element {
    let mut ctrl = Controller::init()?;

    use_effect(use_reactive(&timestamp, move |timestamp| {
        if timestamp.is_some() {
            let datetime = Utc.timestamp_opt(timestamp.unwrap() as i64, 0).unwrap();

            ctrl.year.set(datetime.year());
            ctrl.month.set(datetime.month());
            ctrl.day.set(datetime.day());

            ctrl.selected_year.set(datetime.year());
            ctrl.selected_month.set(datetime.month());
            ctrl.selected_day.set(datetime.day());
        } else {
            let today = chrono::Local::now();
            ctrl.year.set(today.year());
            ctrl.month.set(today.month());
            ctrl.day.set(today.day());

            ctrl.selected_year.set(0);
            ctrl.selected_month.set(0);
            ctrl.selected_day.set(0);
        }
    }));

    rsx! {
        div { class: "flex flex-col w-[500px] items-center justify-center p-[20px] gap-[20px] bg-white border-[1px] borber-[#f5f5f5] rounded-[16px]",
            div { class: "flex flex-col items-start justify-start gap-[8px]",
                div { class: "flex flex-row items-center justify-between w-full gap-[8px] p-[8px]",
                    div {
                        calendar_title { year: ctrl.get_year(), month: ctrl.get_month() }
                    }
                    div { class: "flex flex-row gap-[20px]",
                        left_arrow {
                            onclick: move |_| {
                                ctrl.handle_previous_month();
                            },
                        }
                        right_arrow {
                            onclick: move |_| {
                                ctrl.handle_next_month();
                            },
                        }
                    }
                }
                div { class: "flex flex-col items-start justify-start gap-[8px]",
                    weekday_labels {}
                    calendar_day {
                        days: ctrl.get_days(),
                        year: ctrl.get_year(),
                        month: ctrl.get_month(),
                        selected_year: ctrl.get_selected_year(),
                        selected_month: ctrl.get_selected_month(),
                        selected_day: ctrl.get_selected_day(),
                        today: (ctrl.today)(),
                        onselect: move |(year, month, day, weekday)| {
                            tracing::debug!("day: {:?} {:?} {:?} {:?}", year, month, day, weekday);
                            let date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();
                            let naive_datetime = date.and_hms_opt(0, 0, 0).unwrap();
                            let datetime = Utc.from_utc_datetime(&naive_datetime);
                            let timestamp = datetime.timestamp();
                            update_date.call(timestamp);
                        },
                    }
                }
            }
        }
    }
}
#[component]
pub fn calendar_day(
    days: Vec<Vec<u32>>,
    year: i32,
    month: u32,
    selected_year: i32,
    selected_month: u32,
    selected_day: u32,
    today: (i32, u32, u32),
    onselect: EventHandler<(i32, u32, u32, usize)>,
) -> Element {
    let mut i = 0;
    let pl = days[0].len();
    let mut rows = vec![Vec::<Element>::new(); 6];
    tracing::debug!("days: {:?}", days);
    for (n, days) in days.iter().enumerate() {
        for (ind, day) in days.iter().enumerate() {
            rows[i / 7]
                .push(
                    if n == 1 {
                        let day = day.clone();
                        rsx! {
                            calendar_number {
                                text: "{day}",
                                class: if day == selected_day && month == selected_month && year == selected_year { "bg-[#2a60d3] rounded-[100px] text-white font-semibold text-[16px] cursor-pointer"
                                    .to_string() } else { "bg-white text-[#222222] font-semibold text-[16px] cursor-pointer".to_string() },
                                onclick: move |_| {
                                    onselect((year, month, day, (pl + ind) % 7));
                                },
                            }
                        }
                    } else {
                        rsx! {
                            calendar_number {
                                text: "{day}",
                                class: "bg-white text-[#bfc8d9] text-[16px] font-semibold",
                            }
                        }
                    },
                );
            i += 1;
        }
    }
    rsx! {
        div { class: "flex flex-col items-start justify-start gap-[12px]",
            for row in rows {
                div { class: "flex flex-row items-center justify-center gap-[15px]",
                    for day in row {
                        {day}
                    }
                }
            }
        }
    }
}
#[component]
pub fn calendar_number(
    text: String,
    class: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        div {
            class: format!(
                "flex flex-row w-[48px] h-[48px] p-[8px] gap-[8px] justify-center items-center {}",
                match class {
                    Some(class) => class,
                    None => "bg-white text-[#c4c4c4]".to_string(),
                },
            ),
            onclick: move |e| {
                if let Some(onclick) = onclick {
                    onclick(e);
                }
            },
            "{text}"
        }
    }
}
#[component]
pub fn weekday_labels() -> Element {
    let labels = vec!["S", "M", "T", "W", "T", "F", "S"];
    rsx! {
        div { class: "flex flex-row items-center justify-center gap-[15px]",
            for label in labels {
                div {
                    class: format!(
                        "w-[48px] h-[48px] p-[8px] gap-[15px] text-center font-semibold text-[16px] text-[#2a60d3]",
                    ),
                    "{label}"
                }
            }
        }
    }
}
#[component]
pub fn calendar_title(year: i32, month: u32) -> Element {
    rsx! {
        div { class: "flex-grow-1 h-full flex items-center justify-center gap-[8px]",
            div { class: "w-full text-center text-[#2a60d3] font-semibold text-[16px]",
                "{month}월 {year}년"
            }
        }
    }
}
#[component]
pub fn right_arrow(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "p-[4px] gap-[8px] cursor-pointer", onclick,
            div { class: "w-[24px] h-[24px] p-auto flex items-center justify-center",
                svg {
                    width: "8",
                    height: "14",
                    view_box: "0 0 8 14",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        d: "M1 1L7 7L1 13",
                        stroke: "#2a60d3",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                    }
                }
            }
        }
    }
}
#[component]
pub fn left_arrow(onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "p-[4px] gap-[8px] cursor-pointer", onclick,
            div { class: "w-[24px] h-[24px] p-auto flex items-center justify-center",
                svg {
                    width: "8",
                    height: "14",
                    view_box: "0 0 8 14",
                    fill: "none",
                    xmlns: "http://www.w3.org/2000/svg",
                    path {
                        d: "M7 1L1 7L7 13",
                        stroke: "#2a60d3",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                    }
                }
            }
        }
    }
}
