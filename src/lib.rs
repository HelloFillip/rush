
#![deny(missing_docs,
        trivial_casts,
        unstable_features,
        unused_import_braces)]

//! Server and client SSH library, based on *ring* for its crypto, and
//! tokio/futures for its network management.

#[macro_use]
extern crate tokio_core;
extern crate tokio_io;
#[macro_use]
extern crate futures;

#[derive(Clone, Copy)]
enum Status {
    Ok,
    Disconnect,
}
