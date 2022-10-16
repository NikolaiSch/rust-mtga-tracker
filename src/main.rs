pub mod log_parser;
pub mod message_types;
use dioxus::{desktop, prelude::*};
use dioxus_elements::*;

struct AppProps {
    pub log: log_parser::Log
}

fn main() {
    let mut logs =
    log_parser::Log::new("/Users/vii/Documents/rust-mtg-viewer/dev_files/Player.log");
    logs.init();

    desktop::launch_with_props(app, AppProps {log: logs}, |c| c.with_window(|w| w.with_title("My App")));
}

fn app(cx: Scope<AppProps>) -> Element {
    let inv = cx.props.log.clone().profileData.unwrap().InventoryInfo;

    cx.render(rsx! {
        div {
            {stat_box(cx, String::from("Common"), inv.WildCardCommons)}
            {stat_box(cx, String::from("Uncommon"), inv.WildCardUnCommons)}
            {stat_box(cx, String::from("Rare"), inv.WildCardRares)}
            {stat_box(cx, String::from("Mythic"), inv.WildCardMythics)}
        }
    })
}

fn stat_box(cx: Scope<AppProps>, result_type: String, result: u32) -> Element {
    cx.render(rsx!(
            p {
                h2 {
                    "{result_type}"
                }
            div {
                    "{result}"
                }
            }
    ))
}





