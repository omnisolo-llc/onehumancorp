-- Migration 072: Autonomous Client Intake Questionnaire Engine

CREATE TABLE IF NOT EXISTS questionnaire_templates (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    service_id TEXT REFERENCES services(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    status TEXT DEFAULT 'draft',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS questions (
    id TEXT PRIMARY KEY,
    template_id TEXT REFERENCES questionnaire_templates(id) ON DELETE CASCADE,
    type TEXT NOT NULL, -- "text, multiple_choice, photo_upload"
    text TEXT NOT NULL,
    is_required BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS intake_submissions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    template_id TEXT REFERENCES questionnaire_templates(id) ON DELETE CASCADE,
    customer_id TEXT REFERENCES customers(id) ON DELETE CASCADE,
    status TEXT DEFAULT 'draft', -- "draft, submitted, processed"
    parsed_entities JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS submission_answers (
    id TEXT PRIMARY KEY,
    submission_id TEXT REFERENCES intake_submissions(id) ON DELETE CASCADE,
    question_id TEXT REFERENCES questions(id) ON DELETE CASCADE,
    answer_text TEXT,
    photo_url TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE questionnaire_templates ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_questionnaire_templates ON questionnaire_templates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE intake_submissions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_intake_submissions ON intake_submissions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
