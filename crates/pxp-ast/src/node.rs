use std::fmt::Debug;

use pxp_span::{Span, Spanned};

#[derive(Debug)]
pub struct Node<'a, T: Debug + Spanned + Nodeable> {
    inner: &'a T,
}

impl<'a, T: Debug + Spanned + Nodeable> Node<'a, T> {
    pub fn new(inner: &'a T) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &'a T {
        self.inner
    }
}

impl<'a, T: Debug + Spanned + Nodeable> Spanned for Node<'a, T> {
    fn span(&self) -> Span {
        self.inner.span()
    }
}

pub trait Nodeable {
    fn children(&self) -> Vec<&dyn Nodeable>;
}