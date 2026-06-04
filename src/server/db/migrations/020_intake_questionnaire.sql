-- Migration 020: Autonomous Client Intake Questionnaire Engine (Agent DB)

CREATE TABLE IF NOT EXISTS questionnaire_templates (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    service_id TEXT,
    title TEXT NOT NULL,
    status TEXT DEFAULT 'draft',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS questions (
    id TEXT PRIMARY KEY,
    template_id TEXT REFERENCES questionnaire_templates(id) ON DELETE CASCADE,
    type TEXT NOT NULL,
    text TEXT NOT NULL,
    is_required BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS intake_submissions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    template_id TEXT REFERENCES questionnaire_templates(id) ON DELETE CASCADE,
    customer_id TEXT,
    status TEXT DEFAULT 'draft',
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
