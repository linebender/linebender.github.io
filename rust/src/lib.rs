use xilem_web::{
    elements::html::{button, div, input, p},
    interfaces::{Element as _, HtmlDivElement, HtmlInputElement as _},
    App,
};

use web_sys::HtmlInputElement;

struct AppState {
    name: String,
    clicks: u32,
    distance: f64,
}

fn app_logic(state: &mut AppState) -> impl HtmlDivElement<AppState> + use<> {
    let clicks = state.clicks;
    div((
        input(())
            .type_("range")
            .attr("max", 200)
            .attr("step", "any")
            .on_input(|state: &mut AppState, event| {
                let input_element = event
                    .target()
                    .and_then(|target| target.dyn_into::<HtmlInputElement>().ok())
                    .unwrap();
                state.distance = input_element.value().parse().unwrap();
            }),
        p(format!("argument passed in: {}, distance = {}", state.name, state.distance)),
        button(format!("clicked {clicks} times"))
            .on_click(|state: &mut AppState, _event| state.clicks += 1),
        (state.clicks >= 5).then_some(p("Huzzah, clicked at least 5 times")),
    ))
}

#[wasm_bindgen]
pub fn start(arg: &str) {
    let app = AppState {
        name: arg.to_owned(),
        clicks: 0,
        distance: 50.,
    };
    let root = xilem_web::get_element_by_id("wasm-demo-container");
    App::new(root, app, app_logic).run();
}

use wasm_bindgen::prelude::*;
