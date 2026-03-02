# LLM Integration

PolitikTok integrates with large language models through an **OpenAI-compatible API** client. By default, this points to a local Ollama instance, but any compatible endpoint can be used.

## LLM Client

The `LlmClient` in `src/infrastructure/llm.rs` handles all text generation requests. It is configured with:

- **Base URL**: The API endpoint (e.g., `http://localhost:11434/v1`)
- **Default model**: The model name (e.g., `llama3.1:8b`)
- **Timeout**: Per-request timeout in seconds (default: 120)
- **Max retries**: Number of retry attempts with exponential backoff (default: 3)

### Non-Streaming Generation

Most modules use synchronous generation via the `/chat/completions` endpoint:

```rust
let llm = LlmClient::new(base_url, model, timeout_secs, max_retries);

let response = llm.generate(
    &messages,        // Vec<LlmMessage> with role + content
    None,             // Optional model override
    Some(0.7),        // Temperature
    Some(512),        // Max tokens
).await?;

// response.content      -- generated text
// response.prompt_tokens -- input token count
// response.completion_tokens -- output token count
```

### Streaming Generation

For real-time output (e.g., chat interfaces), the client supports SSE streaming:

```rust
let stream = llm.generate_streaming(
    &messages,
    None,
    Some(0.7),
    Some(1024),
).await?;

// stream yields Result<String, Error> for each content delta
```

The streaming implementation:

1. Sends a request with `"stream": true`
2. Reads the response as a byte stream
3. Parses SSE `data:` lines, extracting `choices[0].delta.content`
4. Yields each content chunk as a `String`
5. Terminates on `data: [DONE]`

## Message Format

All LLM interactions use the chat completions format:

```rust
pub struct LlmMessage {
    pub role: String,    // "system", "user", or "assistant"
    pub content: String,
}
```

Each module constructs its own system prompt tailored to its domain. For example:

- **Volunteer Matching (F01)**: "You are a campaign volunteer coordinator..."
- **Sentiment Monitor (F03)**: "You are a political sentiment classifier..."
- **Opposition Research (F05)**: "You are a political opposition research analyst..."

## Retry Strategy

The client uses exponential backoff for failed requests:

| Attempt | Delay |
|---------|-------|
| 1 (initial) | 0ms |
| 2 (retry 1) | 500ms |
| 3 (retry 2) | 1000ms |
| 4 (retry 3) | 2000ms |

Retries cover both network errors and non-success HTTP status codes. The last error is returned if all attempts fail.

## Usage Logging

Every LLM call is logged to the `llm_usage_log` table in PostgreSQL:

```sql
INSERT INTO llm_usage_log (id, module_id, model, prompt_tokens, completion_tokens, latency_ms)
VALUES ($1, $2, $3, $4, $5, $6)
```

This enables the Admin Panel (F26) to track:

- Token consumption per module
- Latency trends over time
- Model usage distribution
- Cost estimation (when using paid APIs)

## Temperature Guidelines

Different modules use different temperature settings based on their requirements:

| Temperature | Use Case | Modules |
|------------|----------|---------|
| 0.1 | Structured JSON output, classification | Sentiment Monitor (F03), Compliance (F23) |
| 0.3 | Factual Q&A, grounded responses | Policy Chatbot (F02), Knowledge Base (F25) |
| 0.5 | Analytical content, briefings | Opposition Research (F05), Candidate Briefings (F12) |
| 0.7 | Creative content, personalized messages | Volunteer Matching (F01), Campaign Copy (F04), Canvassing (F06) |

## Switching LLM Providers

Because the client uses the OpenAI-compatible API format, switching providers requires only changing environment variables:

```bash
# Local Ollama (default)
LLM_BASE_URL=http://localhost:11434/v1
LLM_MODEL=llama3.1:8b

# vLLM
LLM_BASE_URL=http://vllm-server:8000/v1
LLM_MODEL=meta-llama/Llama-3.1-8B-Instruct

# OpenAI
LLM_BASE_URL=https://api.openai.com/v1
LLM_MODEL=gpt-4o-mini

# llama.cpp server
LLM_BASE_URL=http://localhost:8080/v1
LLM_MODEL=default
```

No code changes are needed. The same applies to the embedding endpoint.

## Prompt Engineering Patterns

Modules follow consistent patterns for prompt construction:

1. **System prompt**: Defines the AI's role, constraints, and output format. Stored as a `const &str` in each module.
2. **Context injection**: Relevant data from the database is formatted into the user message (e.g., volunteer profiles, policy documents, call transcripts).
3. **Output format specification**: Modules that need structured data specify JSON schemas in the system prompt and parse the response with `serde_json`.
4. **Guard rails**: System prompts include instructions to stay grounded in provided context and avoid fabrication.
