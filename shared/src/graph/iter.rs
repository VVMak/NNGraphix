use std::cell::RefCell;
use std::collections::BTreeMap;

use std::iter::FusedIterator;

use super::vertex_entry::*;

pub struct VerticesIterMut<
    'a,
    VertexId: Ord + Clone + std::fmt::Debug,
    Data,
    Iter: Iterator<Item = VertexId>,
> {
    pub(super) ids: Iter,
    pub(super) graph: &'a BTreeMap<VertexId, RefCell<Vertex<VertexId, Data>>>,
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data, Iter: Iterator<Item = VertexId>>
    VerticesIterMut<'a, VertexId, Data, Iter>
{
    pub(super) fn new(
        ids: Iter,
        graph: &'a BTreeMap<VertexId, RefCell<Vertex<VertexId, Data>>>,
    ) -> Self {
        Self { ids, graph }
    }
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data, Iter: Iterator<Item = VertexId>> Iterator
    for VerticesIterMut<'a, VertexId, Data, Iter>
{
    type Item = OccupiedVertexEntry<'a, VertexId, Data>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ids
            .next()
            .map(|id| OccupiedVertexEntry::new(id, self.graph))
    }
}

impl<
        'a,
        VertexId: Ord + Clone + std::fmt::Debug,
        Data,
        Iter: DoubleEndedIterator<Item = VertexId>,
    > DoubleEndedIterator for VerticesIterMut<'a, VertexId, Data, Iter>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.ids
            .next_back()
            .map(|id| OccupiedVertexEntry::new(id, self.graph))
    }
}

impl<
        'a,
        VertexId: Ord + Clone + std::fmt::Debug,
        Data,
        Iter: ExactSizeIterator<Item = VertexId>,
    > ExactSizeIterator for VerticesIterMut<'a, VertexId, Data, Iter>
{
    fn len(&self) -> usize {
        self.ids.len()
    }
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data, Iter: FusedIterator<Item = VertexId>>
    FusedIterator for VerticesIterMut<'a, VertexId, Data, Iter>
{
}

pub struct VerticesIter<
    'a,
    VertexId: Ord + Clone + std::fmt::Debug,
    Data,
    Iter: Iterator<Item = VertexId>,
> {
    pub(super) ids: Iter,
    pub(super) graph: &'a BTreeMap<VertexId, RefCell<Vertex<VertexId, Data>>>,
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data, Iter: Iterator<Item = VertexId>>
    VerticesIter<'a, VertexId, Data, Iter>
{
    pub(super) fn new(
        ids: Iter,
        graph: &'a BTreeMap<VertexId, RefCell<Vertex<VertexId, Data>>>,
    ) -> Self {
        Self { ids, graph }
    }
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data, Iter: Iterator<Item = VertexId>> Iterator
    for VerticesIter<'a, VertexId, Data, Iter>
{
    type Item = OccupiedVertexEntry<'a, VertexId, Data>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ids
            .next()
            .map(|id| OccupiedVertexEntry::new(id, self.graph))
    }
}

impl<
        'a,
        VertexId: Ord + Clone + std::fmt::Debug,
        Data,
        Iter: DoubleEndedIterator<Item = VertexId>,
    > DoubleEndedIterator for VerticesIter<'a, VertexId, Data, Iter>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.ids
            .next_back()
            .map(|id| OccupiedVertexEntry::new(id, self.graph))
    }
}

impl<
        'a,
        VertexId: Ord + Clone + std::fmt::Debug,
        Data,
        Iter: ExactSizeIterator<Item = VertexId>,
    > ExactSizeIterator for VerticesIter<'a, VertexId, Data, Iter>
{
    fn len(&self) -> usize {
        self.ids.len()
    }
}

impl<'a, VertexId: Ord + Clone + std::fmt::Debug, Data, Iter: FusedIterator<Item = VertexId>>
    FusedIterator for VerticesIter<'a, VertexId, Data, Iter>
{
}
