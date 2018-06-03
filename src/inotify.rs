use std::{
    convert,
    collections::VecDeque,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use futures::{Stream, Poll};
use inotify::{Inotify, Event as InotifyEvent, WatchMask};
use tokio::reactor::PollEvented2 as PollEvented;
use mio::{
    unix::EventedF
    Ready,
};

use ::*;

pub struct InotifyWatch<'event> {
    inotify: Inotify,
    fd: PollEvented<EventedFd<'event>>,

    debounce: VecDeque<Event<'event>>,

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

    fn build(&self, handle: &reactor::Handle) -> Result<Self::Watch, Error> {
        //TODO: do we want to allow the user to pass flags to inotify_init1()?
        let inotify = Inotify::init()?;
        let mask =
            self.filter.iter()
                .map(WatchMask::from)
                .collect::<WatchMask>()
                ;

        for &path in self.paths {
            inotify.add_watch(path, mask)?;
        }
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
    type Item = Event<&'event Path>;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if !self.debounce.is_empty() {
            if self.debounce.len() == 1 {
                // Reregister interest if there are no more queued events.
                self.fd.clear_read_ready(Ready::readable())?;
            }

            return Ok(Async::Ready(self.debounce.pop_front()));
        };

        if let Async::NotReady = self.fd.poll_read_ready()? {
            self.fd.clear_read_ready(Ready::readable())?;
            return Ok(Async::NotReady);
        }

        let mut events = self.inotify.read_events()?;
        if let Some(ev) = events.next() {
            self.debounce.extend(events);

            if self.debounce.is_empty() {
                // Reregister interest if there are no more queued events.
                self.fd.clear_read_ready(Ready::readable())?;
            }

            return Ok(Async::Ready(Some(ev)));
        } else {
            self.fd.clear_read_ready(Ready::readable())?;
            Ok(Async::NotReady)
        }
    }
}

impl convert::Into<WatchMask> for EventKind {
    fn into(kind: EventKind) -> WatchMask {
        // TODO: handle file/dir logic --- e.g., output DELETE for dirs,
        //       DELETE_SELF for files?
        use inotify::watch_mask::*;
        use ::EventKind::*;
        match kind {
            Created => CREATE,
            Deleted => DELETE_SELF,
            AttrsModified => ATTRIB,
            WriteOpen => OPEN,
            WriteClosed => CLOSE_WRITE,
            Renamed => MOVE_SELF,
            Rescan => unimplemented!()
        }
    }
}
