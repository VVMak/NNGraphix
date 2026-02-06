pub use std::collections::BTreeSet;
use std::fmt;
pub use super::basic::*;

pub struct Vertex<Data> {
  pub id: VertexId,
  pub data: Data,

  pub incoming: BTreeSet<VertexId>,
  pub outgoing: BTreeSet<VertexId>,
}

impl<Data> Vertex<Data> {
  pub fn new(id: VertexId, data: Data) -> Self {
    Self { id, data, incoming: BTreeSet::new(), outgoing: BTreeSet::new() }
  }
}

impl<Data> fmt::Debug for Vertex<Data> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Vertex(id: {:?}, incoming: {:?}, outgoing: {:?})", self.id, self.incoming, self.outgoing)
  }
}

impl<Data> PartialEq for Vertex<Data> {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl<Data> Eq for Vertex<Data> {}

impl<Data: Clone> Clone for Vertex<Data> {
  fn clone(&self) -> Self {
      Self { id: self.id.clone(), data: self.data.clone(), incoming: self.incoming.clone(), outgoing: self.outgoing.clone() }
  }
}