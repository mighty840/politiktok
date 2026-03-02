use crate::infrastructure::Error;

/// Embedding client for generating vector embeddings via OpenAI-compatible API.
#[derive(Debug, Clone)]
pub struct EmbeddingClient {
    http: reqwest::Client,
    base_url: String,
    model: String,
}

/// A chunk of text with metadata for document ingestion.
#[derive(Debug, Clone)]
pub struct TextChunk {
    pub text: String,
    pub index: usize,
    pub metadata: serde_json::Value,
}

impl EmbeddingClient {
    /// Create a new embedding client.
    pub fn new(base_url: &str, model: &str) -> Self {
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap_or_default();

        Self {
            http,
            base_url: base_url.trim_end_matches('/').to_string(),
            model: model.to_string(),
        }
    }

    /// Embed a single text string.
    pub async fn embed_text(&self, text: &str) -> Result<Vec<f32>, Error> {
        let results = self.embed_batch(&[text.to_string()]).await?;
        results
            .into_iter()
            .next()
            .ok_or_else(|| Error::EmbeddingError("Empty embedding response".into()))
    }

    /// Embed a batch of texts.
    pub async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, Error> {
        let body = serde_json::json!({
            "model": self.model,
            "input": texts,
        });

        let resp = self
            .http
            .post(format!("{}/embeddings", self.base_url))
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::EmbeddingError(format!("Request failed: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body_text = resp.text().await.unwrap_or_default();
            return Err(Error::EmbeddingError(format!(
                "Embedding API returned {status}: {body_text}"
            )));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| Error::EmbeddingError(format!("Invalid response: {e}")))?;

        let data = json["data"]
            .as_array()
            .ok_or_else(|| Error::EmbeddingError("Missing data in response".into()))?;

        let mut embeddings = Vec::with_capacity(data.len());
        for item in data {
            let embedding = item["embedding"]
                .as_array()
                .ok_or_else(|| Error::EmbeddingError("Missing embedding array".into()))?
                .iter()
                .filter_map(|v| v.as_f64().map(|f| f as f32))
                .collect();
            embeddings.push(embedding);
        }

        Ok(embeddings)
    }
}

/// Split text into overlapping chunks for embedding.
pub fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<TextChunk> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return Vec::new();
    }

    let step = if chunk_size > overlap {
        chunk_size - overlap
    } else {
        1
    };

    let mut chunks = Vec::new();
    let mut start = 0;
    let mut index = 0;

    while start < words.len() {
        let end = (start + chunk_size).min(words.len());
        let chunk_text = words[start..end].join(" ");

        chunks.push(TextChunk {
            text: chunk_text,
            index,
            metadata: serde_json::json!({
                "start_word": start,
                "end_word": end,
            }),
        });

        start += step;
        index += 1;
    }

    chunks
}

/// Compute SHA-256 hash of content for deduplication.
pub fn content_hash(content: &str) -> String {
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(content.as_bytes());
    format!("{hash:x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_text() {
        let text = "one two three four five six seven eight nine ten";
        let chunks = chunk_text(text, 4, 1);
        assert!(!chunks.is_empty());
        assert_eq!(chunks[0].text, "one two three four");
        assert_eq!(chunks[0].index, 0);
    }

    #[test]
    fn test_chunk_text_empty() {
        let chunks = chunk_text("", 4, 1);
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_content_hash_deterministic() {
        let hash1 = content_hash("hello world");
        let hash2 = content_hash("hello world");
        assert_eq!(hash1, hash2);
    }
}
