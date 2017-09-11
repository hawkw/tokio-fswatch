extern crate futures;
extern crate tokio_core;

use std::path::Path;
use std::io;
use std::convert::{self, AsRef};

use tokio_core::reactor;
use futures::stream::Stream;

pub trait Watch<'a>: Sized + Stream<Item = Event<'a>, Error = Error> {
    /// Create a new `Watch` with the given `Handle` to a Tokio `Reactor`.
    fn new(handle: &reactor::Handle) -> Result<Self, Error>;

    /// Attempt to establish a watch on a new path.
    fn add_path<P: AsRef<Path>>(&self, path: P) -> Result<Self, Error>;

    fn remove_path<P: AsRef<Path>>(&self, path: P) -> Result<(), Error>;

    fn close(self) -> Result<(), Error>;
}


#[derive(Debug)]
pub enum Event<'a> {
    Created(&'a Path),
    Deleted(&'a Path),
    AttrsModified(&'a Path),
    WriteOpen(&'a Path),
    WriteClosed(&'a Path),
    Renamed(&'a Path, &'a Path),
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

#[cfg(target_os = "linux")]
pub mod inotify;
