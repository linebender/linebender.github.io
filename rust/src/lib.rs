use xilem_web::{
    elements::html::{button, div, p},
    interfaces::{Element as _, HtmlDivElement},
    App,
};

struct AppState {
    name: String,
    clicks: u32,
}

fn app_logic(state: &mut AppState) -> impl HtmlDivElement<AppState> + use<> {
    let clicks = state.clicks;
    div((
        p(format!("argument passed in: {}", state.name)),
        button(format!("clicked {clicks} times")).on_click(|state: &mut AppState, _event| state.clicks += 1),
        (state.clicks >= 5).then_some(p("Huzzah, clicked at least 5 times")),
    ))
}

#[wasm_bindgen]
pub fn start(arg: &str) {
    let app = AppState {
        name: arg.to_owned(),
        clicks: 0,
    };
    let root = xilem_web::get_element_by_id("wasm-demo-container");
    App::new(root, app, app_logic).run();
}

use wasm_bindgen::prelude::*;
