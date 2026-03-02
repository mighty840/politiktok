# RAG Pipeline

PolitikTok implements a **Retrieval-Augmented Generation** (RAG) pipeline that grounds LLM responses in actual document content. This is used by the Policy Chatbot (F02) and Knowledge Base (F25) modules.

## Pipeline Overview

```
1. Document Ingestion
   Text -> Chunking -> Embedding -> Qdrant Storage + PostgreSQL metadata

2. Query Processing
   Question -> Embedding -> Vector Search -> Context Assembly -> LLM Generation
```

## Document Ingestion

### Chunking

Documents are split into overlapping chunks using a word-based sliding window algorithm implemented in `src/infrastructure/embedding.rs`:

```rust
pub fn chunk_text(text: &str, chunk_size: usize, overlap: usize) -> Vec<TextChunk>
```

Default parameters:

| Parameter | Value | Purpose |
|-----------|-------|---------|
| `CHUNK_SIZE` | 300 words | Size of each chunk |
| `CHUNK_OVERLAP` | 50 words | Overlap between consecutive chunks |

The overlap ensures that information spanning a chunk boundary is captured in at least one chunk. Each `TextChunk` includes:

- `text`: The chunk content
- `index`: Sequential chunk number
- `metadata`: Start and end word positions

### Content Deduplication

Before ingesting, a SHA-256 hash of the full document content is computed:

```rust
pub fn content_hash(content: &str) -> String
```

If a document with the same hash already exists in the target collection, the ingestion is skipped and the existing document ID is returned.

### Embedding

Chunks are embedded in batches using the `EmbeddingClient`:

```rust
let embedder = EmbeddingClient::new(base_url, model);
let embeddings = embedder.embed_batch(&chunk_texts).await?;
```

The client calls the `/embeddings` endpoint of an OpenAI-compatible API. The default model is `nomic-embed-text` running on Ollama, which produces 1536-dimensional vectors.

### Vector Storage

Embedded chunks are stored in Qdrant as points with JSON payloads:

```json
{
    "id": "doc-uuid_chunk_0",
    "vector": [0.123, -0.456, ...],
    "payload": {
        "document_id": "doc-uuid",
        "title": "Healthcare Policy Platform",
        "chunk_index": 0,
        "chunk_text": "Our healthcare policy focuses on...",
        "collection": "policy_documents"
    }
}
```

Collections are created on-demand with cosine distance:

```rust
vs.ensure_collection(&collection, vector_size).await?;
vs.upsert(&collection, points).await?;
```

### Metadata Storage

Document metadata is recorded in the PostgreSQL `documents` table:

- `id`, `title`, `source_path`
- `content_hash` (for deduplication)
- `collection_name` (Qdrant collection)
- `chunk_count`
- `tags`, `status`, `ingested_at`

## Query Processing

### Question Embedding

The user's question is embedded using the same model as the documents:

```rust
let query_embedding = embedder.embed_text(&question).await?;
```

### Vector Search

The query vector is searched against the relevant Qdrant collection:

```rust
let results = vs.search(
    &collection,
    query_embedding,
    TOP_K,                    // 5 results
    Some(SCORE_THRESHOLD),    // minimum 0.3 cosine similarity
).await?;
```

Each result includes:

- `id`: Point identifier
- `score`: Cosine similarity score (0.0 to 1.0)
- `payload`: The chunk text and metadata

### Context Assembly

Retrieved chunks are formatted into a context string with source attribution:

```
[Source: Healthcare Policy Platform (relevance: 0.87)]
Our healthcare policy focuses on expanding access to affordable care...

---

[Source: Tax Reform Proposal (relevance: 0.72)]
The proposed tax reform aims to simplify the tax code while...
```

If no chunks meet the score threshold, the context indicates that no relevant documents were found.

### LLM Generation

The assembled context is injected into the user message alongside the original question:

```
Context from policy documents:

[retrieved chunks]

---

Question: What is the candidate's position on healthcare?
```

The system prompt constrains the LLM to:

- Use ONLY the provided context
- Cite source document titles
- Acknowledge when information is insufficient
- Maintain a factual, non-partisan tone

## Collections

| Collection | Module | Content |
|------------|--------|---------|
| `policy_documents` | Policy Chatbot (F02) | Public-facing policy documents, platform positions |
| `knowledge_base` | Knowledge Base (F25) | Internal campaign documents, procedures, talking points |

Additional collections can be created by specifying a custom collection name during ingestion.

## Qdrant Client

The `VectorStoreClient` in `src/infrastructure/vector_store.rs` provides four operations:

| Operation | Method | Purpose |
|-----------|--------|---------|
| Create collection | `ensure_collection()` | Creates a collection if it does not exist |
| Upsert points | `upsert()` | Inserts or updates vectors with payloads |
| Search | `search()` | Finds similar vectors with optional score threshold |
| Delete | `delete()` | Removes points by ID |

All operations use Qdrant's HTTP API directly via `reqwest`, without a dedicated SDK.

## Document Lifecycle

1. **Ingest**: Staff uploads a document -> chunked, embedded, stored in Qdrant + PostgreSQL
2. **Active**: Document chunks are included in search results
3. **Delete**: Staff removes a document -> Qdrant points deleted, PostgreSQL status set to `deleted`

Deleted documents are soft-deleted in PostgreSQL (status changes from `active` to `deleted`) and their vectors are removed from Qdrant.

## Performance Considerations

- **Batch embedding**: Documents are embedded in a single batch API call rather than one chunk at a time.
- **Score threshold**: A minimum similarity score of 0.3 filters out irrelevant results, reducing noise in the LLM context.
- **Top-K limit**: Only the 5 most relevant chunks are retrieved, keeping the prompt size manageable.
- **Chunking overlap**: 50-word overlap prevents information loss at chunk boundaries without excessive duplication.
