use parking_lot::MappedRwLockReadGuard;
use prometheus_client::{
    encoding::text::{Encode, EncodeMetric, Encoder},
    metrics::{
        family::{Family as InnerFamily, MetricConstructor},
        MetricType, TypedMetric,
    },
};
use serde::Serialize;
use std::hash::Hash;
use std::{fmt, io};

/// A wrapper around [`prometheus_client::metrics::family::Family`] which
/// encodes its labels with [`Serialize`] instead of [`Encode`].
#[derive(Debug)]
pub struct Family<S, M, C = fn() -> M> {
    inner: InnerFamily<Bridge<S>, M, C>,
}

impl<S, M, C> Family<S, M, C>
where
    S: Clone + Eq + Hash,
{
    pub fn new_with_constructor(constructor: C) -> Self {
        Self {
            inner: InnerFamily::new_with_constructor(constructor),
        }
    }
}

impl<S, M> Default for Family<S, M>
where
    S: Clone + Eq + Hash,
    M: Default,
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<S, M, C> Family<S, M, C>
where
    S: Clone + Eq + Hash,
    C: MetricConstructor<M>,
{
    pub fn get_or_create(&self, label_set: &S) -> MappedRwLockReadGuard<M> {
        self.inner.get_or_create(Bridge::from_ref(label_set))
    }
}

impl<S, M, C> EncodeMetric for Family<S, M, C>
where
    S: Clone + Eq + Hash + Serialize,
    M: EncodeMetric + TypedMetric,
    C: MetricConstructor<M>,
{
    fn encode(&self, encoder: Encoder) -> io::Result<()> {
        self.inner.encode(encoder)
    }

    fn metric_type(&self) -> MetricType {
        M::TYPE
    }
}

impl<S, M, C> TypedMetric for Family<S, M, C>
where
    M: TypedMetric,
{
    const TYPE: MetricType = <M as TypedMetric>::TYPE;
}

impl<S, M, C> Clone for Family<S, M, C>
where
    C: Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
#[repr(transparent)]
struct Bridge<S>(S);

impl<S> Bridge<S> {
    fn from_ref(label_set: &S) -> &Self {
        // SAFETY: `Self` is a transparent newtype wrapper.
        unsafe { &*(label_set as *const S as *const Bridge<S>) }
    }
}

impl<S> Encode for Bridge<S>
where
    S: Serialize,
{
    fn encode(&self, writer: &mut dyn io::Write) -> Result<(), std::io::Error> {
        crate::to_writer(writer, &self.0)?;

        Ok(())
    }
}

impl<S> fmt::Debug for Bridge<S>
where
    S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
