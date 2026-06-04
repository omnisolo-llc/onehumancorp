-- Create brand_voice_profiles table
CREATE TABLE IF NOT EXISTS brand_voice_profiles (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    tone_descriptors TEXT NOT NULL DEFAULT '[]',
    vocabulary_preferences TEXT NOT NULL DEFAULT '{}',
    specific_knowledge_facts TEXT NOT NULL DEFAULT '[]',
    emoji_usage_level TEXT NOT NULL DEFAULT 'moderate',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(tenant_id)
);
