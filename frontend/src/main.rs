use yew::prelude::*;
use log::info;

#[derive(PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Clone)]
struct GBlock {
    center: Point,
    dragged: bool,
    chosen: bool,
}

#[derive(Clone)]
struct GBoard {
    blocks: std::vec::Vec<GBlock>,
}

#[derive(PartialEq, Properties, Clone)]
struct Coordinates {
    x: i32,
    y: i32,
    dragged: bool,
    chosen: bool
}

#[function_component]
fn Board() -> Html {
    let board: UseStateHandle<GBoard> = use_state(|| GBoard { blocks: vec![] });
    let onkeypress = {
        let board = board.clone();
        Callback::from(move |event: KeyboardEvent| -> () {
            info!("Pressed {}", event.key());
            if event.key() == "n" {
                let mut blocks = board.blocks.clone();
                for b in &mut blocks {
                    b.dragged = false;
                    b.chosen = false;
                }
                blocks.push(GBlock { center: Point{x: event.page_x(), y: event.page_y()}, dragged: true, chosen: true } );
                board.set(GBoard { blocks });
            }
        })
    };
    html!{
        <div onkeypress={onkeypress}>
            <svg width="1920" height="1080">
                {board.blocks.iter().map(|block| {
                    html!{<Block x={block.center.x} y={block.center.y} dragged={block.dragged} chosen={block.chosen}/>}
                }).collect::<Html>()}
            </svg>
        </div>
    }
}

#[function_component]
fn Block(coords: &Coordinates) -> Html {
    let coordinates: UseStateHandle<Coordinates> = use_state(|| coords.clone());
    let onmousemove: Callback<MouseEvent> = {
        let coordinates: UseStateHandle<Coordinates> = coordinates.clone();
        Callback::from(move |event: MouseEvent| -> () {
            if coordinates.dragged == true {
                coordinates.set(Coordinates {
                    x: event.client_x(), 
                    y: event.client_y(),
                    dragged: coordinates.dragged,
                    chosen: coordinates.chosen
                });
            }
        })
    };

    let onmousedown: Callback<MouseEvent> = {
        let coordinates: UseStateHandle<Coordinates> = coordinates.clone();
        Callback::from(move |_: MouseEvent| -> (){
            if coordinates.chosen == true {
                coordinates.set(Coordinates {
                    x: coordinates.x,
                    y: coordinates.y,
                    dragged: true,
                    chosen: coordinates.chosen
                })
            }
        })
    };

    let onmouseup: Callback<MouseEvent> = {
        let coordinates: UseStateHandle<Coordinates> = coordinates.clone();
        Callback::from(move |_: MouseEvent| -> (){
            if coordinates.chosen == true {
                coordinates.set(Coordinates {
                    x: coordinates.x,
                    y: coordinates.y,
                    dragged: false,
                    chosen: coordinates.chosen
                })
            }
        })
    };
    
    let onmouseover: Callback<MouseEvent> = {
        let coordinates: UseStateHandle<Coordinates> = coordinates.clone();
        Callback::from(move |_: MouseEvent| -> (){
            coordinates.set(Coordinates {
                x: coordinates.x,
                y: coordinates.y,
                dragged: coordinates.dragged,
                chosen: true
            })
        })
    };

    let onmouseout: Callback<MouseEvent> = {
        let coordinates: UseStateHandle<Coordinates> = coordinates.clone();
        Callback::from(move |_: MouseEvent| -> (){
            coordinates.set(Coordinates {
                x: coordinates.x,
                y: coordinates.y,
                dragged: coordinates.dragged,
                chosen: false
            })
        })
    };

    html! {
        <g
        class="block"
        onmousemove={onmousemove}
        onmousedown={onmousedown}
        onmouseup={onmouseup}
        onmouseover={onmouseover}
        onmouseout={onmouseout}>
        <rect x={(coordinates.x - 75).to_string()} y={(coordinates.y - 75).to_string()} rx="20" ry="20" width="150" height="150"
        style="fill:red;stroke:black;stroke-width:5;opacity:0.5"/>
        </g>
    }
}

// #[function_component(App)]
// fn app() -> Html {
//     html! {
//         <>
//         <svg width="1920" height="1080">
//         <Block />
//         <Block />
//         </svg>
//         </>
//     }
// }


#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <Board />
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}