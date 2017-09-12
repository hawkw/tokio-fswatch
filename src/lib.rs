extern crate futures;
extern crate tokio_core;

use futures::Stream;
use tokio_core::reactor;

use std::convert::{self, AsRef};
use std::io;
use std::path::Path;

pub trait Watch<'event>: Stream<Item = Event<'event>, Error = Error> + Sized {
    type Builder: Builder<'event, Watch = Self>;

    /// Return a builder for constructing a new watch.
    fn builder() -> Self::Builder;

    /// Construct a new watch on the specified set of paths
    fn new<I, P>(paths: I, handle: &reactor::Handle) -> Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        Self::builder().add_paths(paths).build(handle)
    }
}

pub trait Builder<'event> {
    type Watch: Watch<'event>;

    fn build(&self, handle: &reactor::Handle) -> Self::Watch;

    fn add_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self;

    fn add_paths<I, P>(&mut self, paths: I) -> &mut Self
    where
        I: IntoIterator<Item = P>,
        P: AsRef<Path>,
    {
        paths.into_iter().fold(self, Self::add_path)
    }

    fn filter_event(&mut self, kind: EventKind) -> &mut Self;

    fn filter_events<I>(&mut self, kinds: I) -> &mut Self
    where
        I: IntoIterator<Item = EventKind>,
    {
        kinds.into_iter().fold(self, Self::filter_event)
    }
}

#[derive(Debug)]
pub struct Event<'a> {
    kind: EventKind,
    path: &'a Path,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EventKind {
    Created,
    Deleted,
    AttrsModified,
    WriteOpen,
    WriteClosed,
    Renamed,
    Rescan,
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl convert::From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Io(error)
    }
}
