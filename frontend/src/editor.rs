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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Event::CursorMove { new_pos } => {
                let old_value = self.cursor.update(new_pos);
                let delta =
                    self.viewbox.to_board_coords(new_pos) - self.viewbox.to_board_coords(old_value);
                match &mut self.viewbox {
                    viewbox::State::Dragged(s) => {
                        s.move_viewbox(-delta);
                        true
                    }
                    viewbox::State::Basic(_) => match &mut self.board {
                        board::State::PredragBlocks(s) => {
                            let new_s = s.clone().drag_blocks().to_states_enum();
                            self.board.set_new_state(new_s);
                            ctx.link().send_message(Event::CursorMove { new_pos });
                            false
                        }
                        board::State::DraggingBlocks(s) => {
                            let new_s = s.clone().move_selected(delta).to_states_enum();
                            self.board.set_new_state(new_s);
                            true
                        }
                        board::State::RectangleSelection(s) => {
                            let new_s = s
                                .clone()
                                .move_end(self.viewbox.to_board_coords(new_pos))
                                .to_states_enum();
                            self.board.set_new_state(new_s);
                            true
                        }
                        _ => false,
                    },
                }
            }
            Event::MouseUp(e) => match e.button() {
                0 => {
                    // left button click
                    match &mut self.board {
                        board::State::ArrowCreation(stage) => match stage {
                            board::state::arrow_creation::StateStages::Start(_) => {
                                log::info!("Release left click");
                                false
                            }
                            board::state::arrow_creation::StateStages::Finish(s) => {
                                let new_s = s.clone().commit().to_states_enum();
                                self.board.set_new_state(new_s);
                                true
                            }
                            _ => false,
                        },
                        board::State::Basic(_) => false,
                        board::State::PredragBlocks(s) => {
                            let new_s = s.clone().deselect().to_states_enum();
                            self.board.set_new_state(new_s);
                            true
                        }
                        board::State::DraggingBlocks(s) => {
                            let new_s = s.clone().stop().to_states_enum();
                            self.board.set_new_state(new_s);
                            true
                        }
                        board::State::RectangleSelection(s) => {
                            let new_s = s.clone().finish().to_states_enum();
                            self.board.set_new_state(new_s);
                            true
                        }
                    }
                }
                1 => {
                    // middle button click
                    match &mut self.viewbox {
                        viewbox::State::Basic(_) => {
                            log::warn!("basic state on middle click mouse release");
                            false
                        }
                        viewbox::State::Dragged(s) => {
                            let new_s = s.clone().drop().to_states_enum();
                            self.viewbox.set_new_state(new_s);
                            false
                        }
                    }
                }
                _ => false,
            },
            Event::MouseDown(e) => match e.button() {
                0 => {
                    // left button click
                    match &mut self.board {
                        board::State::ArrowCreation(_) => {
                            log::debug!("ignore board left click on arrow creation");
                        }
                        board::State::Basic(s) => {
                            let new_s = s
                                .clear_selection()
                                .clone()
                                .start_rectangle_selection(
                                    self.viewbox.to_board_coords(self.cursor.get()),
                                )
                                .to_states_enum();
                            self.board.set_new_state(new_s);
                        }
                        _ => {
                            log::warn!("board left click on state {}", self.board);
                        }
                    };
                    false
                }
                1 => {
                    // middle button click
                    match &mut self.viewbox {
                        viewbox::State::Basic(s) => {
                            let new_s = s.clone().drag().to_states_enum();
                            self.viewbox.set_new_state(new_s);
                        }
                        viewbox::State::Dragged(_) => {
                            log::warn!("dragged state on middle click mouse hold");
                        }
                    };
                    false
                }
                _ => false,
            },
            Event::BoardEvent(board::Event::BlockEvent(block_event)) => match block_event {
                board::block::Event::MouseDown(e, id) => match &mut self.board {
                    board::State::ArrowCreation(stages) => match stages {
                        board::state::arrow_creation::StateStages::Preview(s) => {
                            let new_s = s.clone().finish().to_states_enum();
                            self.board.set_new_state(new_s);
                            true
                        }
                        board::state::arrow_creation::StateStages::Finish(_) => {
                            log::warn!("block click on finish arrow creation");
                            false
                        }
                        _ => false,
                    },
                    board::State::Basic(s) => {
                        let new_s = s
                            .clone()
                            .hold_block(
                                id,
                                match e.ctrl_key() {
                                    false => board::state::predrag::SelectionModifier::None,
                                    true => board::state::predrag::SelectionModifier::Add,
                                },
                            )
                            .to_states_enum();
                        self.board.set_new_state(new_s);
                        true
                    }
                    _ => false,
                },
                board::block::Event::MouseOver(id) => match &mut self.board {
                    board::State::ArrowCreation(
                        board::state::arrow_creation::StateStages::Start(s),
                    ) => {
                        let new_s = s.clone().preview(id).to_states_enum();
                        self.board.set_new_state(new_s);
                        true
                    }
                    _ => false,
                },
                board::block::Event::MouseLeave => match &mut self.board {
                    board::State::ArrowCreation(
                        board::state::arrow_creation::StateStages::Preview(s),
                    ) => {
                        let new_s = s.clone().cancel_preview().to_states_enum();
                        self.board.set_new_state(new_s);
                        true
                    }
                    _ => false,
                },
            },
            Event::KeyDown(event) => match &mut self.board {
                board::state::State::Basic(s) => match event.key().as_str() {
                    "a" => {
                        let result = s.clone().try_create_arrow();
                        match result {
                            Ok(state) => {
                                self.board.set_new_state(state.to_states_enum());
                                true
                            }
                            Err(state) => {
                                self.board.set_new_state(state.to_states_enum());
                                false
                            }
                        }
                    }
                    "n" => {
                        s.create_block(self.viewbox.to_board_coords(self.cursor.get()));
                        true
                    }
                    "Delete" => {
                        s.remove_selected_blocks();
                        true
                    }
                    "Escape" => {
                        s.clear_selection();
                        true
                    }
                    _ => false,
                },
                board::State::ArrowCreation(s) => match event.key().as_str() {
                    "Escape" => {
                        match s {
                            board::state::arrow_creation::StateStages::Start(s) => {
                                let new_s = s.clone().cancel().to_states_enum();
                                self.board.set_new_state(new_s);
                                false
                            }
                            board::state::arrow_creation::StateStages::Preview(s) => {
                                let new_s = s.clone().cancel_creation().to_states_enum();
                                self.board.set_new_state(new_s);
                                true
                            }
                            board::state::arrow_creation::StateStages::Finish(s) => {
                                let new_s = s.clone().cancel().to_states_enum();
                                self.board.set_new_state(new_s);
                                true
                            }
                        };
                        false
                    }
                    _ => false,
                },
                _ => false,
            },
            Event::MouseWheel(event) => {
                self.viewbox.scale(self.cursor.get(), event.delta_y());
                true
            }
        }
    }
}
