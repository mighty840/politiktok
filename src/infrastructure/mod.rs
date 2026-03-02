// Server functions accessible from both client and server
pub mod auth_check;

// Server-only modules
#[cfg(feature = "server")]
pub mod auth;
#[cfg(feature = "server")]
pub mod config;
#[cfg(feature = "server")]
pub mod db;
#[cfg(feature = "server")]
pub mod embedding;
#[cfg(feature = "server")]
pub mod error;
#[cfg(feature = "server")]
pub mod llm;
#[cfg(feature = "server")]
pub mod middleware;
#[cfg(feature = "server")]
mod server_start;
#[cfg(feature = "server")]
pub mod state;
#[cfg(feature = "server")]
pub mod vector_store;

#[cfg(feature = "server")]
pub use auth::*;
#[cfg(feature = "server")]
pub use config::*;
#[cfg(feature = "server")]
pub use db::*;
#[cfg(feature = "server")]
pub use embedding::*;
#[cfg(feature = "server")]
pub use error::*;
#[cfg(feature = "server")]
pub use llm::*;
#[cfg(feature = "server")]
pub use server_start::*;
#[cfg(feature = "server")]
pub use state::*;
#[cfg(feature = "server")]
pub use vector_store::*;
