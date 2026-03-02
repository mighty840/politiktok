# F25 -- Knowledge Base

Provides a searchable internal knowledge base with AI-powered Q&A for campaign staff to quickly find policies, procedures, and talking points using RAG (Retrieval-Augmented Generation).

## Key Features

- **Document ingestion**: Upload internal campaign documents that are chunked, embedded, and stored for semantic search.
- **RAG-powered Q&A**: Ask natural language questions and receive answers grounded in ingested documents.
- **Source citations**: Every answer includes references to the source documents and relevance scores.
- **Session management**: Persistent Q&A sessions for ongoing research conversations.
- **Content deduplication**: Prevents re-ingesting documents that already exist in the knowledge base.
- **Document lifecycle**: Manage document status (active, deleted) with vector cleanup.

## Server Functions

| Function | Endpoint | Description |
|----------|----------|-------------|
| `kb_ingest_document` | `knowledge-base/ingest` | Ingest a document into the knowledge base |
| `kb_list_documents` | `knowledge-base/list-documents` | List all knowledge base documents |
| `kb_delete_document` | `knowledge-base/delete-document` | Soft-delete a document |
| `kb_query` | `knowledge-base/query` | Ask a question against the knowledge base |
| `kb_create_session` | `knowledge-base/create-session` | Create a new Q&A session |
| `kb_list_sessions` | `knowledge-base/list-sessions` | List Q&A sessions |
| `kb_get_messages` | `knowledge-base/get-messages` | Get messages for a session |

## RAG Configuration

| Parameter | Value |
|-----------|-------|
| Collection name | `knowledge_base` |
| Vector dimension | 1536 (auto-detected) |
| Chunk size | 300 words |
| Chunk overlap | 50 words |
| Top-K results | 5 |
| Score threshold | 0.3 |

## Differences from Policy Chatbot (F02)

While the Knowledge Base shares the same RAG infrastructure as the Policy Chatbot, they serve different purposes:

| Aspect | Policy Chatbot (F02) | Knowledge Base (F25) |
|--------|---------------------|---------------------|
| Audience | Citizens and public | Internal campaign staff |
| Content | Public policy documents | Internal procedures, talking points, memos |
| Collection | `policy_documents` | `knowledge_base` |
| Tone | Public-facing, non-partisan | Internal, operational |

## UI Components

- **Knowledge base page** (`/knowledge-base`): Q&A interface with document management.
- **Chat panel**: Ask questions and receive answers with source citations.
- **Document manager**: Upload, browse, and delete knowledge base documents.
- **Session history**: Switch between Q&A sessions.
- **Source viewer**: Expand source citations to see the relevant document chunks.

## Database Tables

- `documents` -- shared with F02, filtered by `collection_name = 'knowledge_base'`
- `chat_sessions` -- shared with F02, filtered by `session_type = 'knowledge_base'`
- `chat_messages` -- shared with F02, messages linked to knowledge base sessions
