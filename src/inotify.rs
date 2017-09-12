extern crate inotify;

use std::convert;

use inotify::wrapper::{INotify, Watch as INWatch, Event as INotifyEvent};
use tokio_core::reactor::PollEvented;
use mio::sys::unix::Io;

use ::Event;

pub struct INotifyWatch {
    inotify: INotify,
    io: PollEvented<Io>,
}

impl convert::Into<Event> for INotifyEvent {

}
