pub use super::basic::*;
pub use super::vertex::*;

use std::cell::{RefCell, Ref, RefMut};

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub enum VertexEntry<'a, Data> {
  Occupied(OccupiedVertexEntry<'a, Data>),
  Vacant(VacantVertexEntry<'a, Data>),
}

impl<'a, Data> VertexEntry<'a, Data> {
  pub fn new(vertex_id: VertexId, vertices: &'a mut BTreeMap<VertexId, RefCell<Vertex<Data>>>) -> Self {
    if vertices.contains_key(&vertex_id) {
      Self::Occupied(OccupiedVertexEntry{vertex_id, vertices})
    } else {
      Self::Vacant(VacantVertexEntry{vertex_id, vertices})
    }
  }

  #[allow(unused)]
  pub fn or_insert(self, default: Data) -> OccupiedVertexEntry<'a, Data> {
    match self {
      Self::Occupied(entry) => entry,
      Self::Vacant(entry) => entry.occupy(default),
    }
  }

  pub fn unwrap(self) -> OccupiedVertexEntry<'a, Data> {
    match self {
      Self::Occupied(entry) => entry,
      Self::Vacant(entry) => panic!("No vertex {} in graph", entry.id()),
    }
  }

  pub fn occupy(self, data: Data) -> Option<OccupiedVertexEntry<'a, Data>> {
    match self {
      Self::Occupied(entry) => {
        log::warn!("Vertex {} is already occupied", entry.id());
        None
      },
      Self::Vacant(entry) => Some(entry.occupy(data)),
    }
  }
}

pub struct OccupiedVertexEntry<'a, Data> {
  vertex_id: VertexId,
  vertices: &'a BTreeMap<VertexId, RefCell<Vertex<Data>>>,
}

impl<'a, Data> OccupiedVertexEntry<'a, Data> {
  pub(super) fn new(vertex_id: VertexId, vertices: &'a BTreeMap<VertexId, RefCell<Vertex<Data>>>) -> Self {
    Self { vertex_id, vertices }
  }

  pub fn id(&self) -> VertexId { self.vertex_id }
  fn vertex(&self) -> Ref<'_, Vertex<Data>> { self.vertices[&self.vertex_id].borrow() }
  fn vertex_mut(&mut self) -> RefMut<'_, Vertex<Data>> { self.vertices[&self.vertex_id].borrow_mut() }
  pub fn data(&self) -> Ref<'_, Data> {
    Ref::map(self.vertex(), |v| &v.data)
  }
  pub fn data_mut(&mut self) -> RefMut<'_, Data> {
    RefMut::map(self.vertex_mut(), |v| &mut v.data)
  }

  #[allow(unused)]
  pub fn add_incoming(&mut self, id: VertexId) {
    self.vertex_mut().incoming.insert(id);
    self.vertices[&id].borrow_mut().outgoing.insert(self.vertex_id);
  }
  pub fn add_outgoing(&mut self, id: VertexId) {
    self.vertex_mut().outgoing.insert(id);
    self.vertices[&id].borrow_mut().incoming.insert(self.vertex_id);
  }
}

impl<'a, Data: std::fmt::Debug> std::fmt::Debug for OccupiedVertexEntry<'a, Data> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "OccupiedVertexEntry(id: {:?}, data: {:?})", self.vertex_id, self.data())
  }
}

pub struct VacantVertexEntry<'a, Data> {
  vertex_id: VertexId,
  vertices: &'a mut BTreeMap<VertexId, RefCell<Vertex<Data>>>,  
}

impl<'a, Data> VacantVertexEntry<'a, Data> {
  pub fn id(&self) -> VertexId { self.vertex_id }
  pub fn occupy(self, data: Data) -> OccupiedVertexEntry<'a, Data> {
    self.vertices.insert(self.vertex_id, RefCell::new(Vertex::new(self.vertex_id, data)));
    OccupiedVertexEntry{vertex_id: self.vertex_id, vertices: self.vertices}
  }
}

impl<'a, Data> std::fmt::Debug for VacantVertexEntry<'a, Data> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "VacantVertexEntry(id: {:?})", self.vertex_id)
  }
}


impl<'a, Data> PartialEq for OccupiedVertexEntry<'a, Data> {
  fn eq(&self, other: &Self) -> bool {
    self.vertex_id == other.vertex_id
  }
}
impl<'a, Data> Eq for OccupiedVertexEntry<'a, Data> {}

impl<'a, Data> PartialEq for VacantVertexEntry<'a, Data> {
  fn eq(&self, other: &Self) -> bool {
    self.vertex_id == other.vertex_id
  }
}
impl<'a, Data> Eq for VacantVertexEntry<'a, Data> {}
