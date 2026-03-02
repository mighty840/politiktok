use crate::infrastructure::Error;

/// Search result from vector similarity search.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VectorSearchResult {
    pub id: String,
    pub score: f32,
    pub payload: serde_json::Value,
}

/// Qdrant vector store client.
#[derive(Debug, Clone)]
pub struct VectorStoreClient {
    http: reqwest::Client,
    base_url: String,
}

impl VectorStoreClient {
    /// Create a new Qdrant client.
    pub fn new(base_url: &str) -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self {
            http,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    /// Create a collection if it doesn't exist.
    pub async fn ensure_collection(
        &self,
        name: &str,
        vector_size: u64,
    ) -> Result<(), Error> {
        // Check if collection exists
        let resp = self
            .http
            .get(format!("{}/collections/{name}", self.base_url))
            .send()
            .await
            .map_err(|e| Error::VectorStoreError(format!("Failed to check collection: {e}")))?;

        if resp.status().is_success() {
            return Ok(());
        }

        // Create collection
        let body = serde_json::json!({
            "vectors": {
                "size": vector_size,
                "distance": "Cosine"
            }
        });

        let resp = self
            .http
            .put(format!("{}/collections/{name}", self.base_url))
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::VectorStoreError(format!("Failed to create collection: {e}")))?;

        if !resp.status().is_success() {
            let body_text = resp.text().await.unwrap_or_default();
            return Err(Error::VectorStoreError(format!(
                "Failed to create collection: {body_text}"
            )));
        }

        tracing::info!("Created Qdrant collection: {name}");
        Ok(())
    }

    /// Upsert points into a collection.
    pub async fn upsert(
        &self,
        collection: &str,
        points: Vec<VectorPoint>,
    ) -> Result<(), Error> {
        let points_json: Vec<serde_json::Value> = points
            .into_iter()
            .map(|p| {
                serde_json::json!({
                    "id": p.id,
                    "vector": p.vector,
                    "payload": p.payload,
                })
            })
            .collect();

        let body = serde_json::json!({ "points": points_json });

        let resp = self
            .http
            .put(format!(
                "{}/collections/{collection}/points",
                self.base_url
            ))
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::VectorStoreError(format!("Upsert failed: {e}")))?;

        if !resp.status().is_success() {
            let body_text = resp.text().await.unwrap_or_default();
            return Err(Error::VectorStoreError(format!(
                "Upsert failed: {body_text}"
            )));
        }

        Ok(())
    }

    /// Search for similar vectors.
    pub async fn search(
        &self,
        collection: &str,
        query_vector: Vec<f32>,
        top_k: usize,
        score_threshold: Option<f32>,
    ) -> Result<Vec<VectorSearchResult>, Error> {
        let mut body = serde_json::json!({
            "vector": query_vector,
            "limit": top_k,
            "with_payload": true,
        });

        if let Some(threshold) = score_threshold {
            body["score_threshold"] = serde_json::json!(threshold);
        }

        let resp = self
            .http
            .post(format!(
                "{}/collections/{collection}/points/search",
                self.base_url
            ))
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::VectorStoreError(format!("Search failed: {e}")))?;

        if !resp.status().is_success() {
            let body_text = resp.text().await.unwrap_or_default();
            return Err(Error::VectorStoreError(format!(
                "Search failed: {body_text}"
            )));
        }

        let json: serde_json::Value = resp.json().await.map_err(|e| {
            Error::VectorStoreError(format!("Invalid search response: {e}"))
        })?;

        let results = json["result"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|r| VectorSearchResult {
                id: r["id"].as_str().unwrap_or_default().to_string(),
                score: r["score"].as_f64().unwrap_or_default() as f32,
                payload: r["payload"].clone(),
            })
            .collect();

        Ok(results)
    }

    /// Delete points by IDs from a collection.
    pub async fn delete(
        &self,
        collection: &str,
        ids: &[String],
    ) -> Result<(), Error> {
        let body = serde_json::json!({
            "points": ids,
        });

        let resp = self
            .http
            .post(format!(
                "{}/collections/{collection}/points/delete",
                self.base_url
            ))
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::VectorStoreError(format!("Delete failed: {e}")))?;

        if !resp.status().is_success() {
            let body_text = resp.text().await.unwrap_or_default();
            return Err(Error::VectorStoreError(format!(
                "Delete failed: {body_text}"
            )));
        }

        Ok(())
    }
}

/// A point to upsert into the vector store.
#[derive(Debug, Clone)]
pub struct VectorPoint {
    pub id: String,
    pub vector: Vec<f32>,
    pub payload: serde_json::Value,
}
