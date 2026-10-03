-- Additive usage accounting schema. No account, credit or spending grant is created.
CREATE TABLE IF NOT EXISTS ohc_usage_accounts (tenant_id TEXT PRIMARY KEY, limit_micros BIGINT NOT NULL CHECK(limit_micros >= 0), spent_micros BIGINT NOT NULL DEFAULT 0 CHECK(spent_micros >= 0), reserved_micros BIGINT NOT NULL DEFAULT 0 CHECK(reserved_micros >= 0));
CREATE TABLE IF NOT EXISTS ohc_usage_records (tenant_id TEXT NOT NULL, event_id TEXT NOT NULL, request_digest TEXT NOT NULL, scope_json TEXT NOT NULL, state TEXT NOT NULL, reserved_micros BIGINT NOT NULL CHECK(reserved_micros >= 0), charged_micros BIGINT, provider_cost_micros BIGINT, receipt_json TEXT, receipt_digest TEXT, created_at TEXT NOT NULL DEFAULT (CAST(CURRENT_TIMESTAMP AS TEXT)), PRIMARY KEY(tenant_id,event_id), FOREIGN KEY(tenant_id) REFERENCES ohc_usage_accounts(tenant_id));
CREATE TABLE IF NOT EXISTS ohc_usage_receipts (tenant_id TEXT NOT NULL, provider TEXT NOT NULL, provider_request_id TEXT NOT NULL, event_id TEXT NOT NULL, PRIMARY KEY(tenant_id,provider,provider_request_id), FOREIGN KEY(tenant_id,event_id) REFERENCES ohc_usage_records(tenant_id,event_id));

-- Migration-owned DDL; restricted request handlers need only row privileges.
ALTER TABLE ohc_usage_accounts ENABLE ROW LEVEL SECURITY;
ALTER TABLE ohc_usage_accounts FORCE ROW LEVEL SECURITY;
DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE schemaname=current_schema() AND tablename='ohc_usage_accounts' AND policyname='ohc_usage_tenant') THEN CREATE POLICY ohc_usage_tenant ON ohc_usage_accounts USING (tenant_id=current_setting('app.current_tenant',true)) WITH CHECK (tenant_id=current_setting('app.current_tenant',true)); END IF; END $$;
ALTER TABLE ohc_usage_records ENABLE ROW LEVEL SECURITY;
ALTER TABLE ohc_usage_records FORCE ROW LEVEL SECURITY;
DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE schemaname=current_schema() AND tablename='ohc_usage_records' AND policyname='ohc_usage_tenant') THEN CREATE POLICY ohc_usage_tenant ON ohc_usage_records USING (tenant_id=current_setting('app.current_tenant',true)) WITH CHECK (tenant_id=current_setting('app.current_tenant',true)); END IF; END $$;
ALTER TABLE ohc_usage_receipts ENABLE ROW LEVEL SECURITY;
ALTER TABLE ohc_usage_receipts FORCE ROW LEVEL SECURITY;
DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE schemaname=current_schema() AND tablename='ohc_usage_receipts' AND policyname='ohc_usage_tenant') THEN CREATE POLICY ohc_usage_tenant ON ohc_usage_receipts USING (tenant_id=current_setting('app.current_tenant',true)) WITH CHECK (tenant_id=current_setting('app.current_tenant',true)); END IF; END $$;
