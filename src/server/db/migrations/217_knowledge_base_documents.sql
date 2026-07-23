-- Knowledge Base Documents

CREATE TABLE IF NOT EXISTS knowledge_base_documents (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    source_type TEXT NOT NULL, -- e.g. "manual_upload", "email_forward"
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS knowledge_base_documents_tenant_id_idx ON knowledge_base_documents(tenant_id);

ALTER TABLE knowledge_base_documents ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_knowledge_base_documents
    ON knowledge_base_documents
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));

-- Knowledge Base Document Chunks
CREATE TABLE IF NOT EXISTS knowledge_base_document_chunks (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    document_id TEXT NOT NULL REFERENCES knowledge_base_documents(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    embedding vector(1536),
    chunk_index INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS knowledge_base_document_chunks_tenant_id_idx ON knowledge_base_document_chunks(tenant_id);
CREATE INDEX IF NOT EXISTS knowledge_base_document_chunks_embedding_idx ON knowledge_base_document_chunks USING hnsw (embedding vector_cosine_ops);
CREATE INDEX IF NOT EXISTS knowledge_base_document_chunks_document_id_idx ON knowledge_base_document_chunks(document_id);

ALTER TABLE knowledge_base_document_chunks ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_knowledge_base_document_chunks
    ON knowledge_base_document_chunks
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));
