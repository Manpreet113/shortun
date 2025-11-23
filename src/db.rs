use crate::storage::Storage;
use crate::base62;
use async_trait::async_trait;
use sqlx::{ Pool, Postgres, Row};
use crate::error::AppError;

#[derive(Clone)]
pub struct PostgresStorage{
    pub pool: Pool<Postgres>,
}

#[async_trait]
impl Storage for PostgresStorage {
    async fn shorten(&self, url: &str) -> Result<String, AppError> {
        let row = sqlx::query("INSERT INTO urls (url) VALUES ($1) RETURNING id")
            .bind(url)
            .fetch_one(&self.pool)
            .await?;

        let id: i64 = row.try_get("id")?;
        
        Ok(base62::encode(id as u64))
    }

    async fn get_url(&self, id: &str) -> Result<Option<String>, AppError> {
        let id_num = match base62::decode(id) {
            Ok(n) => n,
            Err(_) => return Ok(None),
        };

        let row = sqlx::query("UPDATE urls SET clicks = clicks + 1 WHERE id = $1 RETURNING url")
            .bind(id_num as i64)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => Ok(Some(r.try_get("url")?)),
            None => Ok(None),
        }
    }
    
    async fn get_stats(&self, id: &str) -> Result<Option<i64>, AppError> {
        let id_num = match base62::decode(id) {
            Ok(n) => n,
            Err(_) => return Ok(None),
        };

        let row = sqlx::query("SELECT clicks FROM urls WHERE id = $1")
            .bind(id_num as i64)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(r) => Ok(Some(r.try_get("clicks")?)),
            None => Ok(None),
        }
    }
}