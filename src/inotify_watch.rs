use std::convert;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use futures::{Stream, Poll};
use inotify::{Inotify, Event as InotifyEvent};
use tokio_core::reactor::PollEvented;
use mio::unix::EventedFd;

use ::*;

pub struct InotifyWatch<'event> {
    inotify: Inotify,
    fd: PollEvented<EventedFd<'event>>,

    // TODO: would be nice to not have to allocate here...
    debounce: Vec<Event<'event>>,

    _event_lifetime: PhantomData<Event<'event>>,
}

#[derive(Debug, Default)]
pub struct InotifyBuilder<'a> {
    paths: Vec<PathBuf>,
    filter: Vec<EventKind>,
    _lifetime: PhantomData<InotifyWatch<'a>>,
}

impl<'event> Builder<'event> for InotifyBuilder<'event> {
    type Watch = InotifyWatch<'event>;

    fn build(&self, handle: &reactor::Handle) -> Self::Watch {
        unimplemented!()
    }

    fn add_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.paths.push(path.as_ref().to_owned());
        self
    }

    fn filter_event(&mut self, kind: EventKind) -> &mut Self {
        self.filter.push(kind);
        self
    }

}

impl<'event> Watch<'event> for InotifyWatch<'event> {
    type Builder = InotifyBuilder<'event>;
    fn builder() -> Self::Builder {
        InotifyBuilder::default()
    }
}

impl<'event> Stream for InotifyWatch<'event> {
    type Item = Event<'event>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.fd.poll_read().map(|_| {
            // poll_read() returned Ready(()) -> map over & change result type
            unimplemented!()
        });
        unimplemented!()
    }
}
