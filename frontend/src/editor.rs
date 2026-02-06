mod board;
mod cursor;
mod event;
mod viewbox;

use event::Event;
use glam::DVec2;
use yew::prelude::*;
use crate::tools::viewable::Viewable;

#[derive(PartialEq, Properties)]
pub struct Props;

#[derive(Default)]
pub struct Editor {
    viewbox: viewbox::State,
    board: board::State,
    cursor: cursor::Cursor,
}

impl Component for Editor {
    type Message = Event;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        return Self::default()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onmousemove = ctx.link().callback(|e: MouseEvent| {
            Event::CursorMove {
                new_value: DVec2 {
                    x: e.client_x() as f64,
                    y: e.client_y() as f64,
                }
            }
        });
        let onkeydown = ctx.link().callback(Event::KeyDown);
        let onmouseup = ctx.link().callback(Event::MouseUp);
        let onmousedown = ctx.link().callback(Event::MouseDown);
        let onwheel = ctx.link().callback(Event::MouseWheel);

        html! {
            <div tabindex="0" {onkeydown} {onmousemove} {onmousedown} {onmouseup} {onwheel}>
                <svg
                width = "100%"
                height = "100%"
                viewBox={self.viewbox.make_viewbox_str()}
                xmlns="http://www.w3.org/2000/svg">
                    {self.board.view(ctx.link().callback(Event::BoardEvent))}
                </svg>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Event::CursorMove{new_value} => {
                let delta = self.cursor.update(new_value);
                match &mut self.viewbox {
                    viewbox::State::Dragged(s) => {
                        s.move_box(-delta);
                        true
                    }
                    viewbox::State::Basic(_) => {
                        match &mut self.board {
                            board::State::PredragBlocks(s) => {
                                let new_s = s.clone().drag_blocks().to_states_enum();
                                self.board.set_new_state(new_s);
                                ctx.link().send_message(Event::CursorMove { new_value });
                                false
                            }
                            board::State::DraggingBlocks(s) => {
                                let new_s = s.clone().move_selected(delta).to_states_enum();
                                self.board.set_new_state(new_s);
                                true
                            }
                            board::State::RectangleSelection(s) => {
                                let new_s = s.clone().move_end(self.viewbox.to_board_coords( new_value)).to_states_enum();
                                self.board.set_new_state(new_s);
                                true
                            }
                            _ => false,
                        }
                    }
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
                        }
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
            }
            Event::MouseDown(e) => match e.button() {
                0 => {
                    // left button click
                    match &mut self.board {
                        board::State::ArrowCreation(_) => {
                            log::debug!("ignore board left click on arrow creation");
                        }
                        board::State::Basic(s) => {
                            let new_s = s.clear_selection().clone().start_rectangle_selection(self.viewbox.to_board_coords(self.cursor.get())).to_states_enum();
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
                        viewbox::State::Dragged(_) => { log::warn!("dragged state on middle click mouse hold"); }
                    };
                    false
                }
                _ => false,
            }
            Event::BoardEvent(board::Event::BlockEvent(board::block::Event::MouseDown(e, id))) => match e.button() {
                // left button click
                0 => match &mut self.board {
                    board::State::ArrowCreation(stages) => match stages {
                        board::state::arrow_creation::StateStages::Start(s) => {
                            let new_s = s.clone().finish(id).to_states_enum();
                            self.board.set_new_state(new_s);
                            true
                        }
                        board::state::arrow_creation::StateStages::Finish(_) => {
                            log::warn!("block click on finish arrow creation");
                            false
                        }
                    }
                    board::State::Basic(s) => {
                        let new_s = s.clone().hold_block(id, match e.ctrl_key() {
                            false => board::state::predrag::SelectionModifier::None,
                            true => board::state::predrag::SelectionModifier::Add,
                        }).to_states_enum();
                        self.board.set_new_state(new_s);
                        true
                    }
                    _ => false,
                },
                1 => {
                    // middle button click
                    match &mut self.viewbox {
                        viewbox::State::Basic(s) => {
                            let new_s = s.clone().drag().to_states_enum();
                            self.viewbox.set_new_state(new_s);
                        }
                        viewbox::State::Dragged(_) => { log::warn!("dragged state on middle click mouse hold"); }
                    };
                    false
                }
                _ => false,
            }
            Event::KeyDown(event) => {
                match &mut self.board {
                board::state::State::Basic(s) => {
                    match event.key().as_str() {
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
                        s.create_block(self.cursor.get());
                        true
                    }
                    "Delete" => {
                        s.remove_selected_blocks();
                        true
                    },
                    "Escape" => {
                        s.clear_selection();
                        true
                    }
                    _ => false,
                    }
                },
                board::State::ArrowCreation(s) => match event.key().as_str() {
                    "Escape" => {
                        match s {
                            board::state::arrow_creation::StateStages::Start(s) => {
                                let new_s = s.clone().cancel().to_states_enum();
                                self.board.set_new_state(new_s);
                            }
                            board::state::arrow_creation::StateStages::Finish(s) => {
                                let new_s = s.clone().cancel().to_states_enum();
                                self.board.set_new_state(new_s);
                            }
                        };
                        false
                    }
                    _ => false,
                },
                _ => false,
            }},
            Event::MouseWheel(event) => {self.viewbox.scale(self.cursor.get(), event.delta_y()); true}
        }
    }
}
