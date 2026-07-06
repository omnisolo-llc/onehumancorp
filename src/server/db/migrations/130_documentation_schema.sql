-- Create Help Articles Table
CREATE TABLE IF NOT EXISTS help_articles (
    id SERIAL PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    category VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    desc_text TEXT NOT NULL,
    link VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_help_articles_tenant_id ON help_articles(tenant_id);

-- Create Video Tutorials Table
CREATE TABLE IF NOT EXISTS video_tutorials (
    id SERIAL PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    duration VARCHAR(50) NOT NULL,
    video_url VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_video_tutorials_tenant_id ON video_tutorials(tenant_id);

-- Create Tooltips Table
CREATE TABLE IF NOT EXISTS tooltips (
    id VARCHAR(255) NOT NULL,
    tenant_id VARCHAR(255) NOT NULL,
    text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (tenant_id, id)
);

CREATE INDEX IF NOT EXISTS idx_tooltips_tenant_id ON tooltips(tenant_id);

-- Create Walkthrough Steps Table
CREATE TABLE IF NOT EXISTS walkthrough_steps (
    id SERIAL PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    page VARCHAR(255) NOT NULL,
    step_order INTEGER NOT NULL,
    selector VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_walkthrough_steps_tenant_id ON walkthrough_steps(tenant_id);
CREATE INDEX IF NOT EXISTS idx_walkthrough_steps_page ON walkthrough_steps(page);


ALTER TABLE help_articles ENABLE ROW LEVEL SECURITY;

ALTER TABLE video_tutorials ENABLE ROW LEVEL SECURITY;

ALTER TABLE tooltips ENABLE ROW LEVEL SECURITY;

ALTER TABLE walkthrough_steps ENABLE ROW LEVEL SECURITY;
