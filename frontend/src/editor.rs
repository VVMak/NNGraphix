mod board;
mod cursor;
mod event;
mod types;
mod viewbox;

use crate::{editor::types::AppCoords, utils::viewable::Viewable};
use event::Event;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props;

#[derive(Default)]
pub struct Editor {
    viewbox: viewbox::State,
    board: board::State,
    cursor: cursor::Cursor,
}

impl Editor {
    #[allow(unused)]
    const GRID_SIZE: f64 = 80.0;

    #[allow(unused)]
    fn grid_html(x: f64, y: f64, width: f64, height: f64) -> yew::Html {
        yew::html! {
        <>
        <defs>
            <pattern id="smallGrid" width={(Self::GRID_SIZE / 10.0).to_string()} height={(Self::GRID_SIZE / 10.0).to_string()} patternUnits="userSpaceOnUse">
            <path d={format!("M {small_grid_size} 0 L 0 0 0 {small_grid_size}", small_grid_size=Self::GRID_SIZE / 10.0)} fill="none" stroke="gray" stroke-width="0.5"/>
            </pattern>
            <pattern id="grid" width={Self::GRID_SIZE.to_string()} height={Self::GRID_SIZE.to_string()} patternUnits="userSpaceOnUse">
            <rect width={Self::GRID_SIZE.to_string()} height={Self::GRID_SIZE.to_string()} fill="url(#smallGrid)"/>
            <path d={format!("M {grid_size} 0 L 0 0 0 {grid_size}", grid_size=Self::GRID_SIZE)} fill="none" stroke="gray" stroke-width="1"/>
            </pattern>
            </defs>
            <rect width={width.to_string()} height={height.to_string()} x={x.to_string()} y={y.to_string()} fill="url(#grid)" />
        </>
        }
    }
}

impl Component for Editor {
    type Message = Event;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        return Self::default();
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousemove = ctx.link().callback(|e: MouseEvent| Event::CursorMove {
            new_pos: AppCoords::new(e.client_x() as f64, e.client_y() as f64),
        });
        let onkeydown = ctx.link().callback(Event::KeyDown);
        let onmouseup = ctx.link().callback(Event::MouseUp);
        let onmousedown = ctx.link().callback(Event::MouseDown);
        let onwheel = ctx.link().callback(Event::MouseWheel);

        let vt = self.viewbox.make_viewbox_tuple();
        html! {
            <div tabindex="0" {onkeydown} {onmousemove} {onmousedown} {onmouseup} {onwheel}>
                <svg
                width = "100%"
                height = "100%"
                viewBox={self.viewbox.make_viewbox_str()}
                xmlns="http://www.w3.org/2000/svg">
                    {Self::grid_html(vt.0, vt.1, vt.2, vt.3)}
                    {self.board.view(ctx.link().callback(Event::BoardEvent))}
                </svg>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Event::CursorMove { new_pos } => {
                let old_pos = self.cursor.update(new_pos);
                let old_board_pos = self.viewbox.to_board_coords(old_pos);
                let new_board_pos = self.viewbox.to_board_coords(new_pos);
                self.viewbox.handle_mouse_move(old_pos, new_pos)
                    || self.board.handle_mouse_move(old_board_pos, new_board_pos)
            }
            Event::MouseUp(e) => match e.button() {
                0 => self.board.handle_left_mouse_button_release(), // left button click
                1 => self.viewbox.handle_middle_mouse_button_release(), // middle button click
                _ => false,
            },
            Event::MouseDown(e) => {
                let cursor_pos = self.viewbox.to_board_coords(self.cursor.get());
                match e.button() {
                    0 => self.board.handle_left_mouse_button_press(cursor_pos), // left button click
                    1 => self.viewbox.handle_middle_mouse_button_press(), // middle button click
                    _ => false,
                }
            }
            Event::BoardEvent(board::Event::BlockEvent(block_event)) => {
                self.board.handle_board_event(block_event)
            }
            Event::KeyDown(event) => {
                let cursor_pos = self.viewbox.to_board_coords(self.cursor.get());
                self.board.handle_key_down(event, cursor_pos)
            }
            Event::MouseWheel(event) => self
                .viewbox
                .handle_mouse_wheel(self.cursor.get(), event.delta_y()),
        }
    }
}
