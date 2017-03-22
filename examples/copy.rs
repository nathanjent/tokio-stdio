extern crate futures;
extern crate tokio_core;
extern crate tokio_stdio;
extern crate tokio_io;

use futures::Future;
use tokio_stdio::stdio::Stdio;
use tokio_core::reactor::Core;
use tokio_io::{AsyncRead, AsyncWrite};

pub fn main() {
    let (read, write) = Stdio::new(1, 1).split();

    tokio_io::io::copy(read, write).wait().unwrap();
}
