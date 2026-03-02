use std::time::Duration;

use crate::infrastructure::Error;

/// A single LLM chat message.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LlmMessage {
    pub role: String,
    pub content: String,
}

/// Response from an LLM generation call.
#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub content: String,
    pub prompt_tokens: Option<i32>,
    pub completion_tokens: Option<i32>,
}

/// OpenAI-compatible LLM client.
#[derive(Debug, Clone)]
pub struct LlmClient {
    http: reqwest::Client,
    base_url: String,
    default_model: String,
    max_retries: u32,
}

impl LlmClient {
    /// Create a new LLM client from config.
    pub fn new(base_url: &str, model: &str, timeout_secs: u64, max_retries: u32) -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .unwrap_or_default();

        Self {
            http,
            base_url: base_url.trim_end_matches('/').to_string(),
            default_model: model.to_string(),
            max_retries,
        }
    }

    /// Generate a completion (non-streaming).
    pub async fn generate(
        &self,
        messages: &[LlmMessage],
        model: Option<&str>,
        temperature: Option<f32>,
        max_tokens: Option<i32>,
    ) -> Result<LlmResponse, Error> {
        let model = model.unwrap_or(&self.default_model);
        let mut body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": false,
        });

        if let Some(temp) = temperature {
            body["temperature"] = serde_json::json!(temp);
        }
        if let Some(max) = max_tokens {
            body["max_tokens"] = serde_json::json!(max);
        }

        let mut last_err = None;
        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                let backoff = Duration::from_millis(500 * 2u64.pow(attempt - 1));
                tokio::time::sleep(backoff).await;
            }

            match self
                .http
                .post(format!("{}/chat/completions", self.base_url))
                .json(&body)
                .send()
                .await
            {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let json: serde_json::Value = resp
                            .json()
                            .await
                            .map_err(|e| Error::LlmError(format!("Invalid response: {e}")))?;

                        let content = json["choices"][0]["message"]["content"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();

                        let prompt_tokens =
                            json["usage"]["prompt_tokens"].as_i64().map(|v| v as i32);
                        let completion_tokens = json["usage"]["completion_tokens"]
                            .as_i64()
                            .map(|v| v as i32);

                        return Ok(LlmResponse {
                            content,
                            prompt_tokens,
                            completion_tokens,
                        });
                    }
                    let status = resp.status();
                    let body_text = resp.text().await.unwrap_or_default();
                    last_err = Some(Error::LlmError(format!(
                        "LLM returned {status}: {body_text}"
                    )));
                }
                Err(e) => {
                    last_err = Some(Error::LlmError(format!("Request failed: {e}")));
                }
            }
        }

        Err(last_err.unwrap_or_else(|| Error::LlmError("Unknown error".into())))
    }

    /// Generate a streaming completion, returning a stream of content deltas.
    pub async fn generate_streaming(
        &self,
        messages: &[LlmMessage],
        model: Option<&str>,
        temperature: Option<f32>,
        max_tokens: Option<i32>,
    ) -> Result<impl futures::Stream<Item = Result<String, Error>>, Error> {
        let model = model.unwrap_or(&self.default_model).to_string();
        let mut body = serde_json::json!({
            "model": model,
            "messages": messages,
            "stream": true,
        });

        if let Some(temp) = temperature {
            body["temperature"] = serde_json::json!(temp);
        }
        if let Some(max) = max_tokens {
            body["max_tokens"] = serde_json::json!(max);
        }

        let resp = self
            .http
            .post(format!("{}/chat/completions", self.base_url))
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::LlmError(format!("Stream request failed: {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body_text = resp.text().await.unwrap_or_default();
            return Err(Error::LlmError(format!(
                "LLM returned {status}: {body_text}"
            )));
        }

        let stream = async_stream::try_stream! {
            use futures::StreamExt;
            let mut byte_stream = resp.bytes_stream();

            let mut buffer = String::new();
            while let Some(chunk) = byte_stream.next().await {
                let chunk = chunk.map_err(|e| Error::LlmError(format!("Stream error: {e}")))?;
                buffer.push_str(&String::from_utf8_lossy(&chunk));

                // Process SSE lines
                while let Some(line_end) = buffer.find('\n') {
                    let line = buffer[..line_end].trim().to_string();
                    buffer = buffer[line_end + 1..].to_string();

                    if line.starts_with("data: ") {
                        let data = &line[6..];
                        if data == "[DONE]" {
                            return;
                        }
                        if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                            if let Some(delta) = json["choices"][0]["delta"]["content"].as_str() {
                                if !delta.is_empty() {
                                    yield delta.to_string();
                                }
                            }
                        }
                    }
                }
            }
        };

        Ok(stream)
    }
}

/// Log LLM usage to the database.
pub async fn log_llm_usage(
    pool: &sqlx::PgPool,
    module_id: &str,
    model: &str,
    prompt_tokens: Option<i32>,
    completion_tokens: Option<i32>,
    latency_ms: i32,
) -> Result<(), Error> {
    sqlx::query(
        "INSERT INTO llm_usage_log (id, module_id, model, prompt_tokens, completion_tokens, latency_ms)
         VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(uuid::Uuid::new_v4())
    .bind(module_id)
    .bind(model)
    .bind(prompt_tokens)
    .bind(completion_tokens)
    .bind(latency_ms)
    .execute(pool)
    .await?;

    Ok(())
}
