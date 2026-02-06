use crate::tools;
use std::collections::BTreeMap;
use std::cell::RefCell;

mod basic;
mod iter;
mod vertex;
mod vertex_entry;

pub use basic::*;
pub use vertex::*;
pub use vertex_entry::*;

#[derive(Debug)]
pub struct Graph<VertexData> {
    id_gen: tools::IdGen,
    vertices: BTreeMap<VertexId, RefCell<Vertex<VertexData>>>,
}

impl<VertexData> Default for Graph<VertexData> {
    fn default() -> Self {
        Self {
            id_gen: tools::IdGen::default(),
            vertices: BTreeMap::default(),
        }
    }
}

impl<VertexData> Graph<VertexData> {
    pub fn entry(&self, id: VertexId) -> Option<OccupiedVertexEntry<VertexData>> {
        match self.vertices.contains_key(&id) {
            true => Some(OccupiedVertexEntry::new(id, &self.vertices)),
            false => None,
        }
    }
    pub fn entry_mut(&mut self, id: VertexId) -> VertexEntry<VertexData> {
        VertexEntry::new(id, &mut self.vertices)
    }
    pub fn new_vertex(&mut self, data: VertexData) -> OccupiedVertexEntry<VertexData> {
        let id = self.id_gen.next().unwrap();
        self.entry_mut(id).occupy(data).unwrap()
    }
    pub fn remove_vertex(&mut self, id: VertexId) {
        let mut vertex = self.vertices.remove(&id).unwrap().into_inner();
    
        vertex.incoming.remove(&id);
        vertex.outgoing.remove(&id);
        vertex.incoming.into_iter().for_each(|id| {self.vertices.get_mut(&id).unwrap().borrow_mut().outgoing.remove(&vertex.id);});
        vertex.outgoing.into_iter().for_each(|id| {self.vertices.get_mut(&id).unwrap().borrow_mut().incoming.remove(&vertex.id);});
    }

    #[allow(unused)]
    pub fn add_edge(&mut self, e: Edge) {
        self.entry_mut(e.0).unwrap().add_outgoing(e.1);
    }


    pub fn iter_vertices(&self) -> impl Iterator<Item = OccupiedVertexEntry<VertexData>> {
        iter::VerticesIter::new(self.vertices.iter().map(|(id, _)| *id), &self.vertices)
    }

    #[allow(unused)]
    pub fn iter_mut_vertices(&mut self) -> impl Iterator<Item = OccupiedVertexEntry<VertexData>> {
        iter::VerticesIterMut::new(self.vertices.keys().map(|id| *id), &self.vertices)
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = Edge> + use<'_, VertexData> {
        self.vertices
            .iter()
            .flat_map(|(id, vertex)|
                vertex.borrow()
                      .outgoing
                      .iter()
                      .map(move |other_id| (*id, *other_id))
                      .collect::<Vec<_>>()
            )
    }
}

impl<VertexData: Clone> Clone for Graph<VertexData> { 
    fn clone(&self) -> Self {
        Self { id_gen: self.id_gen.clone(), vertices: self.vertices.iter().map(|(k, v)| (k.clone(), RefCell::new(v.borrow().clone()))).collect::<BTreeMap<VertexId, RefCell<Vertex<VertexData>>>>() }
    }
}

impl<VertexData> PartialEq for Graph<VertexData> {
    fn eq(&self, other: &Self) -> bool {
        self.vertices == other.vertices
    }
}