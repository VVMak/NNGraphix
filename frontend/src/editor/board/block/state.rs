use crate::editor::{board::graph::OccupiedVertexEntry, types::BoardCoords};

type BlockId = crate::editor::board::block::Id;

const BLOCK_SIZE: BoardCoords = BoardCoords::new_const(150.0, 150.0);

#[derive(PartialEq, Debug)]
pub struct State<'a>(pub OccupiedVertexEntry<'a, super::vertex_data::VertexData>);

#[derive(PartialEq)]
pub struct StateDump {
    id: BlockId,
    vertex_data: super::vertex_data::VertexData,
}

impl<'a> State<'a> {
    pub fn from(entry: OccupiedVertexEntry<'a, super::vertex_data::VertexData>) -> Self {
        Self(entry)
    }

    #[allow(unused)]
    pub fn entry(&self) -> &OccupiedVertexEntry<'a, super::vertex_data::VertexData> {
        &self.0
    }

    pub fn entry_mut(&mut self) -> &mut OccupiedVertexEntry<'a, super::vertex_data::VertexData> {
        &mut self.0
    }

    pub fn dump(self) -> StateDump {
        StateDump {
            id: self.0.id(),
            vertex_data: self.0.data().clone(),
        }
    }
}
pub trait StateInterface {
    fn move_block(&mut self, delta: BoardCoords);
    fn set_selected(&mut self, selected: bool);
    #[allow(unused)]
    fn toggle_selected(&mut self) -> bool;

    fn id(&self) -> BlockId;
    fn selected(&self) -> bool;
    fn size(&self) -> BoardCoords;

    fn top_left(&self) -> BoardCoords;
    fn bottom_right(&self) -> BoardCoords;
    fn center(&self) -> BoardCoords;
}

trait StateImpl {
    fn id(&self) -> BlockId;
    fn vertex_data(&self) -> impl std::ops::Deref<Target = super::vertex_data::VertexData>;
    fn vertex_data_mut(
        &mut self,
    ) -> impl std::ops::DerefMut<Target = super::vertex_data::VertexData>;
}

impl<T: StateImpl> StateInterface for T {
    fn move_block(&mut self, delta: BoardCoords) {
        self.vertex_data_mut().center += delta;
    }
    fn set_selected(&mut self, selected: bool) {
        self.vertex_data_mut().selected = selected;
    }
    fn toggle_selected(&mut self) -> bool {
        self.vertex_data_mut().selected = !self.selected();
        self.selected()
    }

    fn id(&self) -> BlockId {
        self.id()
    }
    fn selected(&self) -> bool {
        self.vertex_data().selected
    }
    fn size(&self) -> BoardCoords {
        BLOCK_SIZE
    }

    fn top_left(&self) -> BoardCoords {
        self.vertex_data().center - self.size() / 2.
    }
    fn bottom_right(&self) -> BoardCoords {
        self.vertex_data().center + self.size() / 2.
    }
    fn center(&self) -> BoardCoords {
        self.vertex_data().center
    }
}

impl<'a> StateImpl for State<'a> {
    fn id(&self) -> BlockId {
        self.0.id()
    }

    fn vertex_data(&self) -> impl std::ops::Deref<Target = super::vertex_data::VertexData> {
        self.0.data()
    }

    fn vertex_data_mut(
        &mut self,
    ) -> impl std::ops::DerefMut<Target = super::vertex_data::VertexData> {
        self.0.data_mut()
    }
}

impl StateImpl for StateDump {
    fn id(&self) -> BlockId {
        self.id
    }

    fn vertex_data(&self) -> impl std::ops::Deref<Target = super::vertex_data::VertexData> {
        &self.vertex_data
    }

    fn vertex_data_mut(
        &mut self,
    ) -> impl std::ops::DerefMut<Target = super::vertex_data::VertexData> {
        &mut self.vertex_data
    }
}

impl Clone for StateDump {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            vertex_data: self.vertex_data.clone(),
        }
    }
}
