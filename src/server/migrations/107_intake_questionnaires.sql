-- +goose Up
-- Migration 107: Autonomous Client Intake Questionnaire Engine

CREATE TABLE IF NOT EXISTS questionnaire_templates (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_questionnaire_templates_tenant ON questionnaire_templates(tenant_id);

CREATE TABLE IF NOT EXISTS questions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    template_id TEXT NOT NULL REFERENCES questionnaire_templates(id) ON DELETE CASCADE,
    type TEXT NOT NULL, -- 'text', 'multiple_choice', 'photo_upload'
    text TEXT NOT NULL,
    is_required BOOLEAN NOT NULL DEFAULT true,
    options JSONB, -- For multiple choice
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_questions_template ON questions(template_id);
CREATE INDEX IF NOT EXISTS idx_questions_tenant ON questions(tenant_id);

CREATE TABLE IF NOT EXISTS intake_submissions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    template_id TEXT NOT NULL REFERENCES questionnaire_templates(id) ON DELETE CASCADE,
    customer_id TEXT,
    customer_name TEXT,
    customer_email TEXT,
    status TEXT NOT NULL DEFAULT 'submitted', -- 'draft', 'submitted', 'processed'
    parsed_entities JSONB, -- AI extracted data
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_intake_submissions_tenant ON intake_submissions(tenant_id);

CREATE TABLE IF NOT EXISTS submission_answers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    submission_id TEXT NOT NULL REFERENCES intake_submissions(id) ON DELETE CASCADE,
    question_id TEXT NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    raw_response TEXT,
    media_url TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_submission_answers_submission ON submission_answers(submission_id);
CREATE INDEX IF NOT EXISTS idx_submission_answers_tenant ON submission_answers(tenant_id);

-- Enforce RLS
ALTER TABLE questionnaire_templates ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_questionnaire_templates ON questionnaire_templates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE questions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_questions ON questions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE intake_submissions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_intake_submissions ON intake_submissions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE submission_answers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_submission_answers ON submission_answers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
DROP TABLE IF EXISTS submission_answers CASCADE;
DROP TABLE IF EXISTS intake_submissions CASCADE;
DROP TABLE IF EXISTS questions CASCADE;
DROP TABLE IF EXISTS questionnaire_templates CASCADE;
