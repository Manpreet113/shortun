use async_trait::async_trait;

#[async_trait]
pub trait Storage: Send + Sync + 'static {
    // Takes a long URL, returns the ID (e.g., "100")
    async fn shorten(&self, url: &str) -> String;

    // Takes an ID, returns the long URL if it exists
    async fn get_url(&self, id: &str) -> Option<String>;
}
