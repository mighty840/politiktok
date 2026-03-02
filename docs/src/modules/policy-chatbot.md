# F02 -- Policy Chatbot

An interactive RAG-powered chatbot that explains policy positions to citizens in plain language, grounding every response in actual policy documents stored in a Qdrant vector store.

## Key Features

- **Document ingestion**: Upload policy documents that are automatically chunked, embedded, and stored in Qdrant for semantic retrieval.
- **RAG-based Q&A**: Every answer is generated from retrieved document chunks, with source citations.
- **Session management**: Persistent chat sessions per user, allowing conversation continuity.
- **Source transparency**: Each response includes relevance scores and source document references.
- **Content deduplication**: SHA-256 hashing prevents duplicate document ingestion.
- **Document lifecycle**: Soft-delete documents and remove associated vectors from Qdrant.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `ingest_document` | `policy-chatbot/ingest-document` | Chunk, embed, and store a policy document |
| `list_documents` | `policy-chatbot/list-documents` | List all documents in a collection |
| `delete_document` | `policy-chatbot/delete-document` | Soft-delete a document and remove vectors |
| `chat` | `policy-chatbot/chat` | Ask a question and get a RAG-grounded answer |
| `create_chat_session` | `policy-chatbot/create-session` | Start a new chat session |
| `list_chat_sessions` | `policy-chatbot/list-sessions` | List the current user's chat sessions |
| `get_chat_messages` | `policy-chatbot/get-messages` | Retrieve all messages in a session |

## RAG Configuration

| Parameter | Value |
|-----------|-------|
| Collection name | `policy_documents` |
| Vector dimension | 1536 (auto-detected) |
| Chunk size | 300 words |
| Chunk overlap | 50 words |
| Top-K results | 5 |
| Score threshold | 0.3 |
| Temperature | 0.3 |
| Max tokens | 1024 |

## System Prompt

The chatbot operates under a strict system prompt that requires it to:

- Answer only from the provided context
- Honestly state when information is insufficient
- Reference source document titles
- Maintain a concise, factual, non-partisan tone

## UI Components

- **Chat interface** (`/policy-chat`): Real-time chat with the policy chatbot, showing messages with source citations and relevance scores.
- **Session switcher**: Left panel listing previous sessions for quick access.
- **Document management**: Admin interface for ingesting and managing policy documents (accessible via the Admin Panel).

## Database Tables

- `documents` -- document metadata (title, hash, collection, chunk count, status)
- `chat_sessions` -- per-user sessions with type and timestamps
- `chat_messages` -- individual messages with role, content, and source references (jsonb)
