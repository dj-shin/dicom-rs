//! DICOM association module
//!
//! This module contains utilities for establishing associations
//! between DICOM nodes via TCP/IP.
//!
//! As an association requester, often as a service class user (SCU),
//! a new association can be started
//! via the [`ClientAssociationOptions`][1] type.
//! The minimum required properties are the accepted abstract syntaxes
//! and the TCP socket address to the target node.
//! 
//! As an association acceptor,
//! usually taking the role of a service class provider (SCP),
//! a newly created [TCP stream][2] can be passed to
//! a previously prepared [`ServerAssociationOptions`][3].
//!
//! [1]: crate::association::client::ClientAssociationOptions
//! [2]: std::net::TcpStream
//! [3]: crate::association::server::ServerAssociationOptions
pub mod server;
pub mod client;

pub(crate) mod pdata;

pub use server::{ServerAssociation, ServerAssociationOptions};
pub use client::{ClientAssociation, ClientAssociationOptions};
pub use pdata::PDataWriter;

/// A service class user or a provider.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum ServiceClassRole {
    /// Service Class User
    Scu,
    /// Service Class Provider
    Scp,
}
