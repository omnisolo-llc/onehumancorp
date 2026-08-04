CREATE TABLE IF NOT EXISTS tooltips (
    id TEXT NOT NULL,
    tenant_id TEXT NOT NULL,
    text TEXT NOT NULL,
    PRIMARY KEY (tenant_id, id)
);
ALTER TABLE tooltips ENABLE ROW LEVEL SECURITY;
CREATE POLICY "tenant_isolation" ON tooltips FOR ALL USING (tenant_id = current_setting('app.current_tenant', true));
