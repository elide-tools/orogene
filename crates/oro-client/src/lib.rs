//! A general-use client for interacting with NPM registry APIs.

mod api;
mod auth_middleware;
mod client;
mod credentials;
mod error;
mod notify;

pub use api::login;
pub use api::packument;
pub use auth_middleware::nerf_dart;
pub use client::{OroClient, OroClientBuilder};
pub use error::OroClientError;

#[cfg(test)]
pub(crate) mod test_util {
    use std::sync::Once;

    /// Test-only: install a rustls provider since the library is built with `rustls-no-provider`.
    pub fn init_crypto() {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        });
    }
}
