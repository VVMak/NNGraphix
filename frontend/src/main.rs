use yew::prelude::*;

#[derive(PartialEq, Properties)]
struct Coordinates {
    x: i32,
    y: i32,
    dragged: bool,
    chosen: bool
}

#[function_component]
fn Block() -> Html {

    let coordinates: UseStateHandle<Coordinates> = use_state(|| Coordinates {x: 0, y: 0, dragged: false, chosen: false});
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
        Callback::from(move |event: MouseEvent| -> (){
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
        Callback::from(move |event: MouseEvent| -> (){
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
        Callback::from(move |event: MouseEvent| -> (){
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
        Callback::from(move |event: MouseEvent| -> (){
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

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <svg width="1920" height="1080">
        <Block />
        <Block />
        </svg>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}