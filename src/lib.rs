use std::path::PathBuf;

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
}
