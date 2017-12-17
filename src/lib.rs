
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

/// Run one step of the protocol. It factors out common code
/// between client::Data and server::Data.
trait AtomicPoll<E> {
    fn atomic_poll(&mut self) -> futures::Poll<Status, E>;
}

/// Handlers future types must implement `FromFinished` trait to 
/// provide reasonable default implementations (i.e. rejecting 
/// all requests).
pub trait FromFinished<T, E>: futures::Future<Item = T, Error = E> {
    /// Turns type `T` into `Self`, a future yielding `T`.
    fn finished(t: T) -> Self;
}