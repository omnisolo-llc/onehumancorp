-- Create brand_voice_profiles table
CREATE TABLE IF NOT EXISTS brand_voice_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id VARCHAR NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    tone_descriptors JSONB NOT NULL DEFAULT '[]', -- Array of strings e.g. ["bubbly", "professional"]
    vocabulary_preferences JSONB NOT NULL DEFAULT '{}', -- Key/value pair e.g. {"greeting": "Hey there!"}
    specific_knowledge_facts JSONB NOT NULL DEFAULT '[]', -- Array of strings e.g. ["We don't do weekend emergency rates."]
    emoji_usage_level VARCHAR NOT NULL DEFAULT 'moderate', -- 'none', 'low', 'moderate', 'high'
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT unique_tenant_brand_voice UNIQUE (tenant_id)
);

-- Row Level Security
ALTER TABLE brand_voice_profiles ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_policy_brand_voice ON brand_voice_profiles
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id', true));

-- Triggers for updated_at
CREATE TRIGGER update_brand_voice_profiles_modtime
    BEFORE UPDATE ON brand_voice_profiles
    FOR EACH ROW EXECUTE FUNCTION update_modified_column();
