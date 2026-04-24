use async_trait::async_trait;
use miette::Result;

pub mod add;
pub mod apply;
#[cfg(feature = "npm-auth")]
pub mod login;
#[cfg(feature = "npm-auth")]
pub mod logout;
#[cfg(feature = "diagnostics")]
pub mod ping;
pub mod reapply;
pub mod remove;
pub mod view;

#[async_trait]
pub trait OroCommand {
    async fn execute(self) -> Result<()>;
}
