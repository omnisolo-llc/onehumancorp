-- +goose Up
-- Migration 023: Autonomous Finance Ledger

CREATE TABLE IF NOT EXISTS bank_accounts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    account_name TEXT NOT NULL,
    institution_name TEXT,
    last_four TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS tax_profiles (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE UNIQUE,
    tax_rate_percent DECIMAL DEFAULT 0,
    estimated_tax_safe DECIMAL DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS transactions (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    bank_account_id TEXT REFERENCES bank_accounts(id) ON DELETE CASCADE,
    amount DECIMAL NOT NULL,
    currency TEXT DEFAULT 'USD',
    type TEXT NOT NULL, -- 'INCOME' or 'EXPENSE'
    status TEXT DEFAULT 'PENDING',
    description TEXT,
    vendor TEXT,
    categorized_as TEXT,
    date TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS receipts (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    transaction_id TEXT REFERENCES transactions(id) ON DELETE SET NULL,
    file_url TEXT NOT NULL,
    extracted_text TEXT,
    uploaded_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS ledger_entries (
    id TEXT PRIMARY KEY,
    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
    transaction_id TEXT REFERENCES transactions(id) ON DELETE CASCADE,
    account_type TEXT NOT NULL, -- 'ASSET', 'LIABILITY', 'EQUITY', 'REVENUE', 'EXPENSE'
    amount DECIMAL NOT NULL,
    entry_type TEXT NOT NULL, -- 'DEBIT' or 'CREDIT'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'bank_accounts',
            'tax_profiles',
            'transactions',
            'receipts',
            'ledger_entries'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', t_name);
            pol_name := format('tenant_isolation_%s', t_name);
            IF NOT EXISTS (
                SELECT 1
                FROM pg_policies
                WHERE schemaname = current_schema()
                    AND tablename = t_name
                    AND policyname = pol_name
            ) THEN
                EXECUTE format(
                    'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
                    pol_name,
                    t_name
                );
            END IF;
        END IF;
    END LOOP;
END
$$;

-- +goose Down
DO $$
DECLARE
    t_name text;
    pol_name text;
BEGIN
    FOR t_name IN
        SELECT unnest(ARRAY[
            'bank_accounts',
            'tax_profiles',
            'transactions',
            'receipts',
            'ledger_entries'
        ])
    LOOP
        IF to_regclass(t_name) IS NOT NULL THEN
            pol_name := format('tenant_isolation_%s', t_name);
            EXECUTE format('DROP POLICY IF EXISTS %I ON %I', pol_name, t_name);
            EXECUTE format('ALTER TABLE %I DISABLE ROW LEVEL SECURITY', t_name);
        END IF;
    END LOOP;
END
$$;

DROP TABLE IF EXISTS ledger_entries CASCADE;
DROP TABLE IF EXISTS receipts CASCADE;
DROP TABLE IF EXISTS transactions CASCADE;
DROP TABLE IF EXISTS tax_profiles CASCADE;
DROP TABLE IF EXISTS bank_accounts CASCADE;
