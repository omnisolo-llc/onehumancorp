DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='pricing_rules' AND column_name='target_id') THEN
        ALTER TABLE pricing_rules ADD COLUMN target_id TEXT NOT NULL DEFAULT '';
    END IF;
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='pricing_rules' AND column_name='is_active') THEN
        ALTER TABLE pricing_rules ADD COLUMN is_active BOOLEAN NOT NULL DEFAULT TRUE;
    END IF;
END
$$;

CREATE INDEX IF NOT EXISTS idx_pricing_rules_target ON pricing_rules(target_id);

ALTER TABLE pricing_rules DROP CONSTRAINT IF EXISTS pricing_rules_tenant_id_target_id_key;
ALTER TABLE pricing_rules ADD CONSTRAINT pricing_rules_tenant_id_target_id_key UNIQUE (tenant_id, target_id);
