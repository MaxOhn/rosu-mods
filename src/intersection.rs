use std::{
    cmp::{min, Ordering},
    collections::{btree_map::Iter as TreeIter, btree_set::Iter as SetIter},
};

use crate::{
    generated_mods::{GameMod, GameModIntermode},
    order::GameModOrder,
};

pub(super) enum IntersectionInner<I, A> {
    Stitch(Stitch<I>),
    Answer(Option<A>),
}

impl<I, A> IntersectionInner<I, A> {
    pub(super) const fn new_stitch(a: I, b: I) -> Self {
        Self::Stitch(Stitch { a, b })
    }
}

pub(super) struct Stitch<I> {
    a: I,
    b: I,
}

/// Iterator over [`GameMod`] references that appear in both given [`GameMods`].
///
/// Created by [`GameMods::intersection`].
///
/// [`GameMods`]: crate::GameMods
/// [`GameMods::intersection`]: crate::GameMods::intersection
// https://github.com/rust-lang/rust/blob/c1d3610ac1ddd1cd605479274047fd0a3f37d220/library/alloc/src/collections/btree/set.rs#L256
pub struct GameModsIntersection<'m> {
    pub(super) inner: IntersectionInner<TreeIter<'m, GameModOrder, GameMod>, &'m GameMod>,
}

impl<'m> Iterator for GameModsIntersection<'m> {
    type Item = &'m GameMod;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            IntersectionInner::Stitch(Stitch { a, b }) => {
                let mut a_next = a.next()?;
                let mut b_next = b.next()?;

                loop {
                    match a_next.0.cmp(b_next.0) {
                        Ordering::Less => a_next = a.next()?,
                        Ordering::Greater => b_next = b.next()?,
                        Ordering::Equal => return Some(a_next.1),
                    }
                }
            }
            IntersectionInner::Answer(answer) => answer.take(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner {
            IntersectionInner::Stitch(Stitch { a, b }) => (0, Some(min(a.len(), b.len()))),
            IntersectionInner::Answer(None) => (0, Some(0)),
            IntersectionInner::Answer(Some(_)) => (1, Some(1)),
        }
    }

    fn min(mut self) -> Option<Self::Item> {
        self.next()
    }
}

/// Iterator over [`GameModIntermode`]s that appear in both given [`GameModsIntermode`].
///
/// Created by [`GameModsIntermode::intersection`].
///
/// [`GameModsIntermode`]: crate::GameModsIntermode
/// [`GameModsIntermode::intersection`]: crate::GameModsIntermode::intersection
pub struct GameModsIntermodeIntersection<'m> {
    pub(super) inner: IntersectionInner<SetIter<'m, GameModIntermode>, GameModIntermode>,
}

impl Iterator for GameModsIntermodeIntersection<'_> {
    type Item = GameModIntermode;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            IntersectionInner::Stitch(Stitch { a, b }) => {
                let mut a_next = a.next()?;
                let mut b_next = b.next()?;

                loop {
                    match a_next.cmp(b_next) {
                        Ordering::Less => a_next = a.next()?,
                        Ordering::Greater => b_next = b.next()?,
                        Ordering::Equal => return Some(*a_next),
                    }
                }
            }
            IntersectionInner::Answer(answer) => answer.take(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match &self.inner {
            IntersectionInner::Stitch(Stitch { a, b }) => (0, Some(min(a.len(), b.len()))),
            IntersectionInner::Answer(None) => (0, Some(0)),
            IntersectionInner::Answer(Some(_)) => (1, Some(1)),
        }
    }

    fn min(mut self) -> Option<Self::Item> {
        self.next()
    }
}
