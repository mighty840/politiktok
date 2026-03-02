-- PolitikTok initial migration
-- Creates all core tables for the platform.

-- ============================================================
-- Extensions
-- ============================================================

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================
-- Users
-- ============================================================

CREATE TABLE users (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email       TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL,
    role        TEXT NOT NULL DEFAULT 'staff'
                    CHECK (role IN ('admin', 'staff', 'volunteer', 'readonly')),
    status      TEXT NOT NULL DEFAULT 'active'
                    CHECK (status IN ('active', 'inactive', 'suspended')),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_active TIMESTAMPTZ
);

CREATE INDEX idx_users_email ON users (email);
CREATE INDEX idx_users_role  ON users (role);

-- ============================================================
-- Volunteers
-- ============================================================

CREATE TABLE volunteers (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name          TEXT NOT NULL,
    email         TEXT NOT NULL UNIQUE,
    phone         TEXT,
    skills        TEXT[] NOT NULL DEFAULT '{}',
    availability  JSONB NOT NULL DEFAULT '{}',
    location      JSONB,
    tags          TEXT[] NOT NULL DEFAULT '{}',
    bio           TEXT,
    status        TEXT NOT NULL DEFAULT 'active'
                      CHECK (status IN ('active', 'inactive', 'on_leave')),
    churn_score   DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_active   TIMESTAMPTZ
);

CREATE INDEX idx_volunteers_email  ON volunteers (email);
CREATE INDEX idx_volunteers_skills ON volunteers USING gin (skills);
CREATE INDEX idx_volunteers_status ON volunteers (status);

-- ============================================================
-- Tasks
-- ============================================================

CREATE TABLE tasks (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title           TEXT NOT NULL,
    description     TEXT NOT NULL DEFAULT '',
    required_skills TEXT[] NOT NULL DEFAULT '{}',
    location        JSONB,
    date_start      TIMESTAMPTZ,
    date_end        TIMESTAMPTZ,
    max_volunteers  INTEGER NOT NULL DEFAULT 1,
    status          TEXT NOT NULL DEFAULT 'open'
                        CHECK (status IN ('open', 'in_progress', 'completed', 'cancelled')),
    created_by      UUID REFERENCES users (id) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_tasks_status     ON tasks (status);
CREATE INDEX idx_tasks_date_start ON tasks (date_start);

-- ============================================================
-- Assignments (volunteer <-> task)
-- ============================================================

CREATE TABLE assignments (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    task_id      UUID NOT NULL REFERENCES tasks (id) ON DELETE CASCADE,
    volunteer_id UUID NOT NULL REFERENCES volunteers (id) ON DELETE CASCADE,
    assigned_by  UUID REFERENCES users (id) ON DELETE SET NULL,
    assigned_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    status       TEXT NOT NULL DEFAULT 'assigned'
                     CHECK (status IN ('assigned', 'accepted', 'declined', 'completed')),
    UNIQUE (task_id, volunteer_id)
);

CREATE INDEX idx_assignments_task      ON assignments (task_id);
CREATE INDEX idx_assignments_volunteer ON assignments (volunteer_id);

-- ============================================================
-- Donors
-- ============================================================

CREATE TABLE donors (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    encrypted_name    TEXT,
    encrypted_email   TEXT,
    donation_history  JSONB NOT NULL DEFAULT '[]',
    engagement_score  DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    last_contact      TIMESTAMPTZ,
    tags              TEXT[] NOT NULL DEFAULT '{}',
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ============================================================
-- Documents (knowledge base)
-- ============================================================

CREATE TABLE documents (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title           TEXT NOT NULL,
    source_path     TEXT,
    content_hash    TEXT,
    collection_name TEXT NOT NULL DEFAULT 'default',
    chunk_count     INTEGER NOT NULL DEFAULT 0,
    tags            TEXT[] NOT NULL DEFAULT '{}',
    ingested_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    status          TEXT NOT NULL DEFAULT 'pending'
                        CHECK (status IN ('pending', 'processing', 'indexed', 'failed', 'active', 'deleted'))
);

CREATE INDEX idx_documents_collection ON documents (collection_name);
CREATE INDEX idx_documents_status     ON documents (status);

-- ============================================================
-- Social posts (sentiment analysis)
-- ============================================================

CREATE TABLE social_posts (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_platform     TEXT NOT NULL,
    external_id         TEXT,
    text                TEXT NOT NULL,
    author_hash         TEXT,
    posted_at           TIMESTAMPTZ,
    fetched_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    sentiment           TEXT CHECK (sentiment IN ('positive', 'negative', 'neutral', 'mixed')),
    sentiment_score     DOUBLE PRECISION,
    topics              TEXT[] NOT NULL DEFAULT '{}',
    location            JSONB,
    coordination_flags  JSONB
);

CREATE INDEX idx_social_posts_platform  ON social_posts (source_platform);
CREATE INDEX idx_social_posts_sentiment ON social_posts (sentiment);
CREATE INDEX idx_social_posts_topics    ON social_posts USING gin (topics);
CREATE INDEX idx_social_posts_posted_at ON social_posts (posted_at);

-- ============================================================
-- Candidates
-- ============================================================

CREATE TABLE candidates (
    id                UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name              TEXT NOT NULL,
    role              TEXT,
    district          TEXT,
    bio               TEXT,
    policy_positions  JSONB NOT NULL DEFAULT '{}',
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ============================================================
-- Chat sessions
-- ============================================================

CREATE TABLE chat_sessions (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id      TEXT,
    session_type TEXT NOT NULL DEFAULT 'policy'
                     CHECK (session_type IN ('policy', 'policy_chatbot', 'coaching', 'empathy', 'general')),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    last_active  TIMESTAMPTZ,
    metadata     JSONB NOT NULL DEFAULT '{}'
);

CREATE INDEX idx_chat_sessions_user ON chat_sessions (user_id);
CREATE INDEX idx_chat_sessions_type ON chat_sessions (session_type);

-- ============================================================
-- Chat messages
-- ============================================================

CREATE TABLE chat_messages (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id UUID NOT NULL REFERENCES chat_sessions (id) ON DELETE CASCADE,
    role       TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
    content    TEXT NOT NULL,
    sources    JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_chat_messages_session ON chat_messages (session_id);

-- ============================================================
-- Audit log
-- ============================================================

CREATE TABLE audit_log (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id    UUID REFERENCES users (id) ON DELETE SET NULL,
    action     TEXT NOT NULL,
    resource   TEXT NOT NULL,
    detail     JSONB,
    ip_address TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_audit_log_user    ON audit_log (user_id);
CREATE INDEX idx_audit_log_action  ON audit_log (action);
CREATE INDEX idx_audit_log_created ON audit_log (created_at);

-- ============================================================
-- Module configuration
-- ============================================================

CREATE TABLE module_config (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    module_name TEXT NOT NULL UNIQUE,
    enabled     BOOLEAN NOT NULL DEFAULT true,
    config      JSONB NOT NULL DEFAULT '{}',
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_by  UUID REFERENCES users (id) ON DELETE SET NULL
);

-- ============================================================
-- Prompt templates
-- ============================================================

CREATE TABLE prompt_templates (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name        TEXT NOT NULL UNIQUE,
    module      TEXT NOT NULL,
    template    TEXT NOT NULL,
    variables   TEXT[] NOT NULL DEFAULT '{}',
    version     INTEGER NOT NULL DEFAULT 1,
    is_active   BOOLEAN NOT NULL DEFAULT true,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_prompt_templates_module ON prompt_templates (module);

-- ============================================================
-- LLM usage log
-- ============================================================

CREATE TABLE llm_usage_log (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id        UUID REFERENCES users (id) ON DELETE SET NULL,
    module         TEXT NOT NULL,
    model_name     TEXT NOT NULL,
    prompt_tokens  INTEGER NOT NULL DEFAULT 0,
    output_tokens  INTEGER NOT NULL DEFAULT 0,
    latency_ms     INTEGER,
    error          TEXT,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_llm_usage_module  ON llm_usage_log (module);
CREATE INDEX idx_llm_usage_created ON llm_usage_log (created_at);

-- ============================================================
-- Feedback
-- ============================================================

CREATE TABLE feedback (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id      UUID REFERENCES users (id) ON DELETE SET NULL,
    module       TEXT NOT NULL,
    reference_id TEXT,
    rating       INTEGER CHECK (rating BETWEEN 1 AND 5),
    comment      TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_feedback_module ON feedback (module);

-- ============================================================
-- Commitments (accountability tracker)
-- ============================================================

CREATE TABLE commitments (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    candidate_id   UUID REFERENCES candidates (id) ON DELETE CASCADE,
    title          TEXT NOT NULL,
    description    TEXT NOT NULL DEFAULT '',
    category       TEXT,
    source_url     TEXT,
    date_made      DATE,
    deadline       DATE,
    status         TEXT NOT NULL DEFAULT 'pending'
                       CHECK (status IN ('pending', 'in_progress', 'fulfilled', 'broken', 'modified')),
    confidence     DOUBLE PRECISION NOT NULL DEFAULT 0.5,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_commitments_candidate ON commitments (candidate_id);
CREATE INDEX idx_commitments_status    ON commitments (status);

-- ============================================================
-- Commitment evidence
-- ============================================================

CREATE TABLE commitment_evidence (
    id            UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    commitment_id UUID NOT NULL REFERENCES commitments (id) ON DELETE CASCADE,
    source_url    TEXT NOT NULL,
    summary       TEXT NOT NULL,
    supports      BOOLEAN NOT NULL DEFAULT true,
    fetched_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_commitment_evidence_commitment ON commitment_evidence (commitment_id);

-- ============================================================
-- Regulatory sources
-- ============================================================

CREATE TABLE regulatory_sources (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name         TEXT NOT NULL,
    url          TEXT NOT NULL,
    jurisdiction TEXT,
    source_type  TEXT NOT NULL DEFAULT 'government'
                     CHECK (source_type IN ('government', 'ngo', 'news', 'academic')),
    enabled      BOOLEAN NOT NULL DEFAULT true,
    last_checked TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ============================================================
-- Regulatory updates
-- ============================================================

CREATE TABLE regulatory_updates (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_id   UUID NOT NULL REFERENCES regulatory_sources (id) ON DELETE CASCADE,
    title       TEXT NOT NULL,
    summary     TEXT NOT NULL,
    url         TEXT,
    published   TIMESTAMPTZ,
    impact_tags TEXT[] NOT NULL DEFAULT '{}',
    read        BOOLEAN NOT NULL DEFAULT false,
    fetched_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_regulatory_updates_source ON regulatory_updates (source_id);
CREATE INDEX idx_regulatory_updates_read   ON regulatory_updates (read);

-- ============================================================
-- Data sources (admin-managed external data integrations)
-- ============================================================

CREATE TABLE data_sources (
    id             UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name           TEXT NOT NULL UNIQUE,
    source_type    TEXT NOT NULL,
    connection_url TEXT,
    config         JSONB NOT NULL DEFAULT '{}',
    enabled        BOOLEAN NOT NULL DEFAULT true,
    last_sync      TIMESTAMPTZ,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- ============================================================
-- Seed default module configuration
-- ============================================================

INSERT INTO module_config (module_name, enabled, config) VALUES
    ('volunteer_matching', true,  '{"default_radius_km": 50}'),
    ('policy_chatbot',     true,  '{"max_context_chunks": 5}'),
    ('sentiment_monitor',  true,  '{"poll_interval_secs": 300}'),
    ('campaign_copy',      true,  '{}'),
    ('opposition_research', true, '{}'),
    ('canvassing',         true,  '{}'),
    ('fundraising',        true,  '{}'),
    ('accountability',     true,  '{}'),
    ('empathy_simulator',  true,  '{}'),
    ('narrative_contagion', true, '{}'),
    ('coalition_detector', true,  '{}'),
    ('candidate_briefings', true, '{}'),
    ('call_intelligence',  true,  '{}'),
    ('coaching_debate',    true,  '{}'),
    ('multilingual',       true,  '{}'),
    ('question_anticipation', true, '{}'),
    ('local_issues',       true,  '{}'),
    ('policy_diff',        true,  '{}'),
    ('faction_mapper',     true,  '{}'),
    ('regulatory_monitor', true,  '{}'),
    ('media_monitor',      true,  '{}'),
    ('disinfo_warning',    true,  '{}'),
    ('compliance',         true,  '{}'),
    ('meetings',           true,  '{}'),
    ('knowledge_base',     true,  '{}')
ON CONFLICT (module_name) DO NOTHING;
