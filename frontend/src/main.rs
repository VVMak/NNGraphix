use yew::prelude::*;

mod board;
mod tools;

#[function_component(App)]
fn app() -> Html {
    html! {
        <board::Board />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
