use std::cell::RefCell;
use std::collections::BTreeMap;

mod iter;
mod vertex;
mod vertex_entry;

pub use vertex::*;
pub use vertex_entry::*;

#[derive(Debug)]
pub struct Graph<VertexId, VertexData> {
    vertices: BTreeMap<VertexId, RefCell<Vertex<VertexId, VertexData>>>,
}

impl<VertexId, VertexData> Default for Graph<VertexId, VertexData> {
    fn default() -> Self {
        Self {
            vertices: BTreeMap::default(),
        }
    }
}

impl<VertexId: Ord + Clone + std::fmt::Debug, VertexData> Graph<VertexId, VertexData> {
    pub fn entry(&self, id: VertexId) -> Option<OccupiedVertexEntry<'_, VertexId, VertexData>> {
        match self.vertices.contains_key(&id) {
            true => Some(OccupiedVertexEntry::new(id, &self.vertices)),
            false => None,
        }
    }
    pub fn entry_mut(&mut self, id: VertexId) -> VertexEntry<'_, VertexId, VertexData> {
        VertexEntry::new(id, &mut self.vertices)
    }
    pub fn new_vertex(&mut self, id: VertexId, data: VertexData) -> OccupiedVertexEntry<'_, VertexId, VertexData> {
        self.entry_mut(id).occupy(data).unwrap()
    }
    pub fn remove_vertex(&mut self, id: VertexId) {
        let mut vertex = self.vertices.remove(&id).unwrap().into_inner();

        vertex.incoming.remove(&id);
        vertex.outgoing.remove(&id);
        vertex.incoming.into_iter().for_each(|id| {
            self.vertices
                .get_mut(&id)
                .unwrap()
                .borrow_mut()
                .outgoing
                .remove(&vertex.id);
        });
        vertex.outgoing.into_iter().for_each(|id| {
            self.vertices
                .get_mut(&id)
                .unwrap()
                .borrow_mut()
                .incoming
                .remove(&vertex.id);
        });
    }

    #[allow(unused)]
    pub fn add_edge(&mut self, e: (VertexId, VertexId)) {
        self.entry_mut(e.0).unwrap().add_outgoing(e.1);
    }

    pub fn iter_vertices(&self) -> impl Iterator<Item = OccupiedVertexEntry<'_, VertexId, VertexData>> {
        iter::VerticesIter::new(self.vertices.iter().map(|(id, _)| id.clone()), &self.vertices)
    }

    #[allow(unused)]
    pub fn iter_mut_vertices(
        &mut self,
    ) -> impl Iterator<Item = OccupiedVertexEntry<'_, VertexId, VertexData>> {
        iter::VerticesIterMut::new(self.vertices.keys().map(|id| id.clone()), &self.vertices)
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (VertexId, VertexId)> + '_ {
        self.vertices.iter().flat_map(|(id, vertex)| {
            vertex
                .borrow()
                .outgoing
                .iter()
                .map(move |other_id| (id.clone(), other_id.clone()))
                .collect::<Vec<_>>()
        })
    }
}

impl<VertexId: Ord + Clone, VertexData: Clone> Clone for Graph<VertexId, VertexData> {
    fn clone(&self) -> Self {
        Self {
            vertices: self
                .vertices
                .iter()
                .map(|(k, v)| (k.clone(), RefCell::new(v.borrow().clone())))
                .collect::<BTreeMap<_, _>>(),
        }
    }
}

impl<VertexId: Ord, VertexData: PartialEq> PartialEq for Graph<VertexId, VertexData> {
    fn eq(&self, other: &Self) -> bool {
        self.vertices == other.vertices
    }
}
