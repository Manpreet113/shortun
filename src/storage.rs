use async_trait::async_trait;
use crate::error::AppError;

#[async_trait]
pub trait Storage: Send + Sync + 'static {
    async fn shorten(&self, url: &str) -> Result<String, AppError>;

    async fn get_url(&self, id: &str) -> Result<Option<String>, AppError>;
}
