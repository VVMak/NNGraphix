use yew::prelude::*;

mod editor;
mod utils;

#[function_component(App)]
fn app() -> Html {
    html! {
        <editor::Editor />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
