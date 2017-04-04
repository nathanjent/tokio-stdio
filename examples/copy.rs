extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_stdio;

use tokio_io::AsyncRead;
use futures::Future;
use tokio_stdio::stdio::Stdio;

pub fn main() {
    let (read, write) = Stdio::new(1, 1).split();

    tokio_io::io::copy(read, write).wait().unwrap();
}
