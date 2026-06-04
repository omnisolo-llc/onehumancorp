-- Migration 072: Autonomous Client Intake Questionnaire Engine

CREATE TABLE IF NOT EXISTS questionnaire_templates (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE questionnaire_templates ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_questionnaire_templates ON questionnaire_templates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS questionnaire_questions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    template_id TEXT REFERENCES questionnaire_templates(id) ON DELETE CASCADE,
    type TEXT NOT NULL, -- 'text', 'multiple_choice', 'photo_upload'
    text TEXT NOT NULL,
    is_required BOOLEAN DEFAULT false,
    options JSONB, -- For multiple_choice
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE questionnaire_questions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_questionnaire_questions ON questionnaire_questions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS intake_submissions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    customer_id TEXT, -- Might be null if guest checkout
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'submitted', -- 'draft', 'submitted', 'processed'
    parsed_entities JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE intake_submissions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_intake_submissions ON intake_submissions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE TABLE IF NOT EXISTS intake_submission_answers (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    submission_id TEXT REFERENCES intake_submissions(id) ON DELETE CASCADE,
    question_id TEXT REFERENCES questionnaire_questions(id) ON DELETE CASCADE,
    answer_text TEXT,
    answer_photo_url TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE intake_submission_answers ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_intake_submission_answers ON intake_submission_answers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
