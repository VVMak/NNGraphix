pub use super::vertex::*;

use std::cell::{Ref, RefCell, RefMut};

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub enum VertexEntry<'a, VertexId: Ord + Clone + std::fmt::Debug, Data> {
    Occupied(OccupiedVertexEntry<'a, VertexId, Data>),
    Vacant(VacantVertexEntry<'a, VertexId, Data>),
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data> VertexEntry<'a, VertexId, Data> {
    pub fn new(
        vertex_id: VertexId,
        vertices: &'a mut BTreeMap<VertexId, RefCell<Vertex<VertexId, Data>>>,
    ) -> Self {
        if vertices.contains_key(&vertex_id) {
            Self::Occupied(OccupiedVertexEntry {
                vertex_id,
                vertices,
            })
        } else {
            Self::Vacant(VacantVertexEntry {
                vertex_id,
                vertices,
            })
        }
    }

    #[allow(unused)]
    pub fn or_insert(self, default: Data) -> OccupiedVertexEntry<'a, VertexId, Data> {
        match self {
            Self::Occupied(entry) => entry,
            Self::Vacant(entry) => entry.occupy(default),
        }
    }

    pub fn unwrap(self) -> OccupiedVertexEntry<'a, VertexId, Data> {
        match self {
            Self::Occupied(entry) => entry,
            Self::Vacant(entry) => panic!("No vertex {:?} in graph", entry.id()),
        }
    }

    pub fn occupy(self, data: Data) -> Option<OccupiedVertexEntry<'a, VertexId, Data>> {
        match self {
            Self::Occupied(entry) => {
                log::warn!("Vertex {:?} is already occupied", entry.id());
                None
            }
            Self::Vacant(entry) => Some(entry.occupy(data)),
        }
    }
}

pub struct OccupiedVertexEntry<'a, VertexId: Ord + Clone, Data> {
    vertex_id: VertexId,
    vertices: &'a BTreeMap<VertexId, RefCell<Vertex<VertexId, Data>>>,
}

impl<'a, VertexId: Ord + Clone, Data> OccupiedVertexEntry<'a, VertexId, Data> {
    pub(super) fn new(
        vertex_id: VertexId,
        vertices: &'a BTreeMap<VertexId, RefCell<Vertex<VertexId, Data>>>,
    ) -> Self {
        Self {
            vertex_id,
            vertices,
        }
    }

    pub fn id(&self) -> VertexId {
        self.vertex_id.clone()
    }
    fn vertex(&self) -> Ref<'_, Vertex<VertexId, Data>> {
        self.vertices[&self.vertex_id].borrow()
    }
    fn vertex_mut(&mut self) -> RefMut<'_, Vertex<VertexId, Data>> {
        self.vertices[&self.vertex_id].borrow_mut()
    }
    pub fn data(&self) -> Ref<'_, Data> {
        Ref::map(self.vertex(), |v| &v.data)
    }
    pub fn data_mut(&mut self) -> RefMut<'_, Data> {
        RefMut::map(self.vertex_mut(), |v| &mut v.data)
    }

    #[allow(unused)]
    pub fn add_incoming(&mut self, id: VertexId) {
        self.vertex_mut().incoming.insert(id.clone());
        self.vertices[&id]
            .borrow_mut()
            .outgoing
            .insert(self.vertex_id.clone());
    }
    pub fn add_outgoing(&mut self, id: VertexId) {
        self.vertex_mut().outgoing.insert(id.clone());
        self.vertices[&id]
            .borrow_mut()
            .incoming
            .insert(self.vertex_id.clone());
    }
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data: std::fmt::Debug> std::fmt::Debug
    for OccupiedVertexEntry<'a, VertexId, Data>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OccupiedVertexEntry(id: {:?}, data: {:?})",
            self.vertex_id,
            self.data()
        )
    }
}

pub struct VacantVertexEntry<'a, VertexId: Ord + Clone + std::fmt::Debug, Data> {
    vertex_id: VertexId,
    vertices: &'a mut BTreeMap<VertexId, RefCell<Vertex<VertexId, Data>>>,
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data> VacantVertexEntry<'a, VertexId, Data> {
    pub fn id(&self) -> VertexId {
        self.vertex_id.clone()
    }
    pub fn occupy(self, data: Data) -> OccupiedVertexEntry<'a, VertexId, Data> {
        self.vertices.insert(
            self.vertex_id.clone(),
            RefCell::new(Vertex::new(self.vertex_id.clone(), data)),
        );
        OccupiedVertexEntry {
            vertex_id: self.vertex_id,
            vertices: self.vertices,
        }
    }
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data: std::fmt::Debug> std::fmt::Debug
    for VacantVertexEntry<'a, VertexId, Data>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VacantVertexEntry(id: {:?})", self.vertex_id)
    }
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data: PartialEq> PartialEq
    for OccupiedVertexEntry<'a, VertexId, Data>
{
    fn eq(&self, other: &Self) -> bool {
        self.vertex_id == other.vertex_id
    }
}
impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data: Eq> Eq
    for OccupiedVertexEntry<'a, VertexId, Data>
{
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data: PartialEq> PartialEq
    for VacantVertexEntry<'a, VertexId, Data>
{
    fn eq(&self, other: &Self) -> bool {
        self.vertex_id == other.vertex_id
    }
}
impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data: Eq> Eq
    for VacantVertexEntry<'a, VertexId, Data>
{
}
