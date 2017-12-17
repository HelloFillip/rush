
#![deny(missing_docs,
        trivial_casts,
        unstable_features,
        unused_import_braces)]

//! Server and client SSH library, based on *ring* for its crypto, and
//! tokio/futures for its network management.