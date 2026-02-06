use std::cell::RefCell;
use std::collections::BTreeMap;

use std::iter::FusedIterator;

use super::basic::*;
use super::vertex_entry::*;

pub struct VerticesIterMut<'a, Data, Iter: Iterator<Item = VertexId>> {
  pub(super) ids: Iter,
  pub(super) graph: &'a BTreeMap<VertexId, RefCell<Vertex<Data>>>,
}

impl<'a, Data, Iter: Iterator<Item = VertexId>> VerticesIterMut<'a, Data, Iter>
{
  pub (super) fn new (ids: Iter, graph: &'a BTreeMap<VertexId, RefCell<Vertex<Data>>>) -> Self {
    Self { ids, graph }
  }
}

impl<'a, Data, Iter: Iterator<Item = VertexId>> Iterator for VerticesIterMut<'a, Data, Iter> {
  type Item = OccupiedVertexEntry<'a, Data>;

  fn next(&mut self) -> Option<Self::Item> {
    self.ids.next().map(|id| OccupiedVertexEntry::new(id, self.graph))
  }
}

impl<'a, Data, Iter: DoubleEndedIterator<Item = VertexId>> DoubleEndedIterator for VerticesIterMut<'a, Data, Iter> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.ids.next_back().map(|id| OccupiedVertexEntry::new(id, self.graph))
  }
}

impl<'a, Data, Iter: ExactSizeIterator<Item = VertexId>> ExactSizeIterator for VerticesIterMut<'a, Data, Iter> {
  fn len(&self) -> usize {
    self.ids.len()
  }
}

impl<'a, Data, Iter: FusedIterator<Item = VertexId>> FusedIterator for VerticesIterMut<'a, Data, Iter> {}


pub struct VerticesIter<'a, Data, Iter: Iterator<Item = VertexId>> {
  pub(super) ids: Iter,
  pub(super) graph: &'a BTreeMap<VertexId, RefCell<Vertex<Data>>>,
}

impl<'a, Data, Iter: Iterator<Item = VertexId>> VerticesIter<'a, Data, Iter> {
  pub (super) fn new (ids: Iter, graph: &'a BTreeMap<VertexId, RefCell<Vertex<Data>>>) -> Self {
    Self { ids, graph }
  }
}

impl<'a, Data, Iter: Iterator<Item = VertexId>> Iterator for VerticesIter<'a, Data, Iter> {
  type Item = OccupiedVertexEntry<'a, Data>;

  fn next(&mut self) -> Option<Self::Item> {
    self.ids.next().map(|id| OccupiedVertexEntry::new(id, self.graph))
  }
}

impl<'a, Data, Iter: DoubleEndedIterator<Item = VertexId>> DoubleEndedIterator for VerticesIter<'a, Data, Iter> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.ids.next_back().map(|id| OccupiedVertexEntry::new(id, self.graph))
  }
}

impl<'a, Data, Iter: ExactSizeIterator<Item = VertexId>> ExactSizeIterator for VerticesIter<'a, Data, Iter> {
  fn len(&self) -> usize {
    self.ids.len()
  }
}

impl<'a, Data, Iter: FusedIterator<Item = VertexId>> FusedIterator for VerticesIter<'a, Data, Iter> {}
