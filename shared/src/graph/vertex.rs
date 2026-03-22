pub use std::collections::BTreeSet;
use std::fmt;

pub struct Vertex<VertexId, Data> {
  pub id: VertexId,
  pub data: Data,

  pub incoming: BTreeSet<VertexId>,
  pub outgoing: BTreeSet<VertexId>,
}

impl<VertexId, Data> Vertex<VertexId, Data> {
  pub fn new(id: VertexId, data: Data) -> Self {
    Self { id, data, incoming: BTreeSet::new(), outgoing: BTreeSet::new() }
  }
}

impl<VertexId: std::fmt::Debug, Data> fmt::Debug for Vertex<VertexId, Data> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Vertex(id: {:?}, incoming: {:?}, outgoing: {:?})", self.id, self.incoming, self.outgoing)
  }
}

impl<VertexId: PartialEq, Data> PartialEq for Vertex<VertexId, Data> {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
  }
}

impl<VertexId: Eq, Data> Eq for Vertex<VertexId, Data> {}

impl<VertexId: Clone, Data: Clone> Clone for Vertex<VertexId, Data> {
  fn clone(&self) -> Self {
      Self { id: self.id.clone(), data: self.data.clone(), incoming: self.incoming.clone(), outgoing: self.outgoing.clone() }
  }
}