use std::path::PathBuf;
use std::io;
use std::convert;

#[derive(Debug)]
pub struct Event {
    path: Option<PathBuf>,
    kind: EventKind,
}

#[derive(Debug, Copy, Clone)]
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
