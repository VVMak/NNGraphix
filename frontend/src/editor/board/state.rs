pub mod arrow_creation;
pub mod basic;
pub mod dragging_blocks;
pub mod internal;
pub mod predrag;
pub mod rectangle_selection;

pub mod states;

use std::mem;

use yew::KeyboardEvent;

use super::block;
use crate::{editor::types::BoardCoords, utils::viewable::Viewable};

#[derive(PartialEq, Clone)]
pub enum State {
    Basic(basic::State),
    PredragBlocks(predrag::State),
    DraggingBlocks(dragging_blocks::State),
    RectangleSelection(rectangle_selection::State),
    ArrowCreation(arrow_creation::StateStages),
}

impl Default for State {
    fn default() -> Self {
        State::Basic(basic::State::default())
    }
}

impl State {
    fn transition(&mut self, new_state: Self, should_render: bool) -> bool {
        self.set_new_state(new_state);
        should_render
    }
    pub fn set_new_state(&mut self, new_state: Self) -> &mut Self {
        *self = new_state;
        log::debug!("New board state: {}", self);
        self
    }
    pub fn handle_mouse_move(
        &mut self,
        old_cursor_pos: BoardCoords,
        new_cursor_pos: BoardCoords,
    ) -> bool {
        let current = mem::take(self);
        match current {
            State::PredragBlocks(s) => {
                let new_state = s.drag_blocks().to_states_enum();
                self.transition(new_state, false)
            }
            State::DraggingBlocks(s) => {
                let new_state = s
                    .move_selected(new_cursor_pos - old_cursor_pos)
                    .to_states_enum();
                self.transition(new_state, true)
            }
            State::RectangleSelection(s) => {
                let new_state = s.move_end(new_cursor_pos).to_states_enum();
                self.transition(new_state, true)
            }
            other => self.transition(other, false),
        }
    }
    pub fn handle_left_mouse_button_release(&mut self) -> bool {
        let current = mem::take(self);
        match current {
            State::ArrowCreation(stage) => match stage {
                arrow_creation::StateStages::Start(_) => {
                    log::info!("Release left click");
                    self.transition(State::ArrowCreation(stage), false)
                }
                arrow_creation::StateStages::Finish(s) => {
                    let new_state = s.commit().to_states_enum();
                    self.transition(new_state, true)
                }
                arrow_creation::StateStages::Preview(_) => {
                    self.transition(State::ArrowCreation(stage), false)
                }
            },
            State::Basic(_) => false,
            State::PredragBlocks(s) => {
                let new_state = s.deselect().to_states_enum();
                self.transition(new_state, true)
            }
            State::DraggingBlocks(s) => {
                let new_state = s.stop().to_states_enum();
                self.transition(new_state, true)
            }
            State::RectangleSelection(s) => {
                let new_state = s.finish().to_states_enum();
                self.transition(new_state, true)
            }
        }
    }
    pub fn handle_left_mouse_button_press(&mut self, cursor_pos: BoardCoords) -> bool {
        let current = mem::take(self);
        match current {
            State::Basic(mut s) => {
                s.clear_selection();
                let new_state = s.start_rectangle_selection(cursor_pos).to_states_enum();
                self.set_new_state(new_state);
            }
            other => {
                *self = other;
                log::warn!("board left click on state {}", self);
            }
        };
        false
    }
    pub fn handle_board_event(&mut self, block_event: block::Event) -> bool {
        let current = mem::take(self);
        match block_event {
            block::Event::MouseDown(e, id) => match current {
                State::ArrowCreation(stages) => match stages {
                    arrow_creation::StateStages::Preview(s) => {
                        let new_state = s.finish().to_states_enum();
                        self.transition(new_state, true)
                    }
                    arrow_creation::StateStages::Finish(_) => {
                        log::warn!("block click on finish arrow creation");
                        self.transition(State::ArrowCreation(stages), false)
                    }
                    arrow_creation::StateStages::Start(_) => {
                        self.transition(State::ArrowCreation(stages), false)
                    }
                },
                State::Basic(s) => {
                    let new_state = s
                        .hold_block(
                            id,
                            match e.ctrl_key() {
                                false => predrag::SelectionModifier::None,
                                true => predrag::SelectionModifier::Add,
                            },
                        )
                        .to_states_enum();
                    self.transition(new_state, true)
                }
                other => self.transition(other, false),
            },
            block::Event::MouseOver(id) => match current {
                State::ArrowCreation(arrow_creation::StateStages::Start(s)) => {
                    let new_state = s.preview(id).to_states_enum();
                    self.transition(new_state, true)
                }
                other => self.transition(other, false),
            },
            block::Event::MouseLeave => match current {
                State::ArrowCreation(arrow_creation::StateStages::Preview(s)) => {
                    let new_state = s.cancel_preview().to_states_enum();
                    self.transition(new_state, true)
                }
                other => self.transition(other, false),
            },
        }
    }
    pub fn handle_key_down(&mut self, event: KeyboardEvent, cursor_pos: BoardCoords) -> bool {
        let current = mem::take(self);
        match current {
            State::Basic(mut s) => match event.key().as_str() {
                "a" => {
                    let result = s.try_create_arrow();
                    match result {
                        Ok(state) => self.transition(state.to_states_enum(), true),
                        Err(state) => self.transition(state.to_states_enum(), false),
                    }
                }
                "n" => {
                    s.create_block(cursor_pos);
                    self.transition(State::Basic(s), true)
                }
                "Delete" => {
                    s.remove_selected_blocks();
                    self.transition(State::Basic(s), true)
                }
                "Escape" => {
                    s.clear_selection();
                    self.transition(State::Basic(s), true)
                }
                _ => self.transition(State::Basic(s), false),
            },
            State::ArrowCreation(s) => match event.key().as_str() {
                "Escape" => {
                    match s {
                        arrow_creation::StateStages::Start(s) => {
                            let new_state = s.cancel().to_states_enum();
                            self.transition(new_state, false)
                        }
                        arrow_creation::StateStages::Preview(s) => {
                            let new_state = s.cancel_creation().to_states_enum();
                            self.transition(new_state, true)
                        }
                        arrow_creation::StateStages::Finish(s) => {
                            let new_state = s.cancel().to_states_enum();
                            self.transition(new_state, true)
                        }
                    };
                    false
                }
                _ => self.transition(State::ArrowCreation(s), false),
            },
            other => self.transition(other, false),
        }
    }
}

impl Viewable<yew::Html> for State {
    type Callback = yew::Callback<crate::editor::board::Event>;

    fn view(&self, callback: Self::Callback) -> yew::Html {
        match self {
            State::Basic(state) => state.view(callback),
            State::PredragBlocks(state) => state.view(callback),
            State::DraggingBlocks(state) => state.view(callback),
            State::RectangleSelection(state) => state.view(callback),
            State::ArrowCreation(state) => state.view(callback),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Basic(s) => s.fmt(f),
            State::PredragBlocks(s) => s.fmt(f),
            State::DraggingBlocks(s) => s.fmt(f),
            State::RectangleSelection(s) => s.fmt(f),
            State::ArrowCreation(stages) => stages.fmt(f),
        }
    }
}
