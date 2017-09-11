use std::path::Path;
use std::io;
use std::convert;


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
