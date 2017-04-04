extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_stdio;
extern crate tokio_service;
extern crate bytes;

use std::{io, str};
use bytes::BytesMut;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder};
use futures::{Sink, Stream, Future};
use futures::future::{self, BoxFuture, FutureResult, result, err};
use tokio_stdio::stdio::Stdio;
use tokio_service::{NewService, Service};
use tokio_core::reactor::Core;

pub fn main() {
    let status = match serve(move || Ok(EchoService)) {
        Ok(_) => 0,
        Err(e) => {
            println!("{:?}", e);
            1
        }
    };
    ::std::process::exit(status);
}

fn serve<S>(s: S) -> io::Result<()>
    where S: NewService<Request = String, Response = String, Error = io::Error> + 'static
{
    let mut core = Core::new()?;
    let handle = core.handle();

    let stdio = Stdio::new(1, 1);

    let server = future::lazy(move || -> FutureResult<(), io::Error> {
        let (writer, reader) = stdio.framed(LineCodec).split();
        if let Ok(service) = s.new_service() {
            let responses = reader.and_then(move |req| {
                println!("read");
                service.call(req)
            });
            let server = writer.send_all(responses)
                .then(|_| {
                    println!("write");
                    Ok(())
                });
            handle.spawn(server);
            result::<(), io::Error>(Ok(()))
        } else {
            err::<(), io::Error>(io::Error::new(io::ErrorKind::Other, "Service failed"))
        }
    });

    core.run(server)
}

struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        println!("decode");
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}

impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        println!("encode");
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

struct EchoService;

impl Service for EchoService {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<String, io::Error>;

    fn call(&self, input: String) -> Self::Future {
        future::ok(input).boxed()
    }
}
