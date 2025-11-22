use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::RwLock;
use async_trait::async_trait;

#[async_trait]
pub trait Storage: Send + Sync + 'static {
    // Takes a long URL, returns the ID (e.g., "100")
    async fn shorten(&self, url: &str) -> String;

    // Takes an ID, returns the long URL if it exists
    async fn get_url(&self, id: &str) -> Option<String>;
}

#[derive(Clone)]
pub struct MemoryStorage {
    counter: Arc<AtomicU64>,
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            counter: Arc::new(AtomicU64::new(0)),
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
// Derive macro because standard Rust traits struggle
// with returning Futures (async) dynamically.
#[async_trait]
impl Storage for MemoryStorage {
    async fn shorten(&self, url: &str) -> String {
        let id = self.counter.fetch_add(1, Ordering::Relaxed); // Add 1 to counter to get id and Ordering relaxed cause who needs strictness for id gen

        // TODO: Convert 'id' (u64) to Base62 (String)

        let id_string = id.to_string();

        let mut map = self.data.write().await;

        map.insert(id_string.clone(), url.to_string());

        id_string
    }

    async fn get_url(&self, id: &str) -> Option<String>{
        let map = self.data.read().await;
        map.get(id).cloned()
    }
}
