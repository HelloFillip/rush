
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

#[derive(Debug)]
/// Errors.
pub enum Error {
    /// The key file could not be parsed.
    CouldNotReadKey,

    /// Unspecified problem with the beginning of key exchange.
    KexInit,

    /// No common key exchange algorithm.
    NoCommonKexAlgo,

    /// No common signature algorithm.
    NoCommonKeyAlgo,

    /// No common cipher.
    NoCommonCipher,

    /// Invalid SSH version string.
    Version,

    /// Error during key exchange.
    Kex,

    /// Invalid packet authentication code.
    PacketAuth,

    /// The protocol is in an inconsistent state.
    Inconsistent,

    /// Index out of bounds.
    IndexOutOfBounds,

    /// UTF-8 decoding error (most probably ASCII error).
    Utf8(std::str::Utf8Error),

    /// Unknown server key.
    UnknownKey,

    /// Message received/sent on unopened channel.
    WrongChannel,

    /// I/O error.
    IO(std::io::Error),

    /// Disconnected
    Disconnect,

    /// No home directory found when trying to learn new host key.
    NoHomeDir,

    /// Remote key changed, this could mean a man-in-the-middle attack
    /// is being performed on the connection.
    KeyChanged(usize),

    /// Connection closed by the remote side.
    HUP,

    /// Error from the cryptography layer.
    OpenSSL(openssl::error::Error),

    /// Error from the cryptography layer.
    OpenSSLStack(openssl::error::ErrorStack),

    /// Unit error (sodiumoxide might return that).
    Unit,

    /// Connection timeout.
    ConnectionTimeout,

    /// Missing authentication method.
    NoAuthMethod,
}