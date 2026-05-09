package db

import (
	"context"
	"database/sql"
	"os"
	"testing"
	"time"

	_ "github.com/lib/pq"
	"github.com/stretchr/testify/assert"
)

func TestUnifiedDataModelRLSIntegration(t *testing.T) {
	dbURL := os.Getenv("DATABASE_URL")
	if dbURL == "" {
		dbURL = "postgres://postgres:postgres@localhost:5432/test?sslmode=disable"
	}

	db, err := sql.Open("postgres", dbURL)
	if err != nil {
		t.Fatalf("Skipping integration test: %v", err)
	}
	defer db.Close()

	if err := db.Ping(); err != nil {
		t.Fatalf("Skipping integration test due to ping failure: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// 1. Setup unified tables
	setupSQL := `
		CREATE EXTENSION IF NOT EXISTS vector;

		DROP TABLE IF EXISTS test_agent_memory CASCADE;
		DROP TABLE IF EXISTS test_order_line_item CASCADE;
		DROP TABLE IF EXISTS test_order CASCADE;
		DROP TABLE IF EXISTS test_item_variant CASCADE;
		DROP TABLE IF EXISTS test_catalog_item CASCADE;
		DROP TABLE IF EXISTS test_customer CASCADE;
		DROP TABLE IF EXISTS test_tenant CASCADE;

		CREATE TABLE test_tenant (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			name VARCHAR NOT NULL,
			domain VARCHAR NOT NULL,
			tier VARCHAR NOT NULL,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE test_customer (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			tenant_id UUID NOT NULL REFERENCES test_tenant(id) ON DELETE CASCADE,
			email VARCHAR NOT NULL,
			phone VARCHAR,
			preferences JSONB DEFAULT '{}',
			last_active TIMESTAMP WITH TIME ZONE,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE test_catalog_item (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			tenant_id UUID NOT NULL REFERENCES test_tenant(id) ON DELETE CASCADE,
			title VARCHAR NOT NULL,
			description VARCHAR,
			item_type VARCHAR NOT NULL,
			is_active BOOLEAN NOT NULL DEFAULT TRUE,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE test_item_variant (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			tenant_id UUID NOT NULL REFERENCES test_tenant(id) ON DELETE CASCADE,
			catalog_item_id UUID NOT NULL REFERENCES test_catalog_item(id) ON DELETE CASCADE,
			sku VARCHAR NOT NULL,
			price DECIMAL(10, 2) NOT NULL,
			inventory_count INT NOT NULL DEFAULT 0,
			attributes JSONB DEFAULT '{}',
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE test_order (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			tenant_id UUID NOT NULL REFERENCES test_tenant(id) ON DELETE CASCADE,
			customer_id UUID NOT NULL REFERENCES test_customer(id) ON DELETE CASCADE,
			status VARCHAR NOT NULL,
			total_amount DECIMAL(10, 2) NOT NULL,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE test_order_line_item (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			tenant_id UUID NOT NULL REFERENCES test_tenant(id) ON DELETE CASCADE,
			order_id UUID NOT NULL REFERENCES test_order(id) ON DELETE CASCADE,
			variant_id UUID NOT NULL REFERENCES test_item_variant(id) ON DELETE CASCADE,
			quantity INT NOT NULL,
			unit_price DECIMAL(10, 2) NOT NULL,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE test_agent_memory (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			tenant_id UUID NOT NULL REFERENCES test_tenant(id) ON DELETE CASCADE,
			customer_id UUID REFERENCES test_customer(id) ON DELETE SET NULL,
			department VARCHAR NOT NULL,
			embedding vector(1536),
			raw_context JSONB DEFAULT '{}',
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);

		ALTER TABLE test_tenant ENABLE ROW LEVEL SECURITY;
		ALTER TABLE test_tenant FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_customer ENABLE ROW LEVEL SECURITY;
		ALTER TABLE test_customer FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_catalog_item ENABLE ROW LEVEL SECURITY;
		ALTER TABLE test_catalog_item FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_item_variant ENABLE ROW LEVEL SECURITY;
		ALTER TABLE test_item_variant FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_order ENABLE ROW LEVEL SECURITY;
		ALTER TABLE test_order FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_order_line_item ENABLE ROW LEVEL SECURITY;
		ALTER TABLE test_order_line_item FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_agent_memory ENABLE ROW LEVEL SECURITY;
		ALTER TABLE test_agent_memory FORCE ROW LEVEL SECURITY;

		CREATE POLICY tenant_isolation_test_tenant ON test_tenant USING (id = nullif(current_setting('app.current_tenant', true), '')::uuid);
		CREATE POLICY tenant_isolation_test_customer ON test_customer USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid);
		CREATE POLICY tenant_isolation_test_catalog_item ON test_catalog_item USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid);
		CREATE POLICY tenant_isolation_test_item_variant ON test_item_variant USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid);
		CREATE POLICY tenant_isolation_test_order ON test_order USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid);
		CREATE POLICY tenant_isolation_test_order_line_item ON test_order_line_item USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid);
		CREATE POLICY tenant_isolation_test_agent_memory ON test_agent_memory USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::uuid);

		DO $$
		BEGIN
		  IF NOT EXISTS (
			SELECT FROM pg_catalog.pg_roles
			WHERE  rolname = 'rls_test_user') THEN
			CREATE ROLE rls_test_user LOGIN;
		  END IF;
		END
		$$;
		GRANT ALL PRIVILEGES ON TABLE test_tenant TO rls_test_user;
		GRANT ALL PRIVILEGES ON TABLE test_customer TO rls_test_user;
		GRANT ALL PRIVILEGES ON TABLE test_catalog_item TO rls_test_user;
		GRANT ALL PRIVILEGES ON TABLE test_item_variant TO rls_test_user;
		GRANT ALL PRIVILEGES ON TABLE test_order TO rls_test_user;
		GRANT ALL PRIVILEGES ON TABLE test_order_line_item TO rls_test_user;
		GRANT ALL PRIVILEGES ON TABLE test_agent_memory TO rls_test_user;
	`
	_, err = db.ExecContext(ctx, setupSQL)
	assert.NoError(t, err)

	// 2. Insert test data
	// Need to bypass RLS for setup (running as superuser, disable FORCE RLS)
	disableSQL := `
		ALTER TABLE test_tenant NO FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_customer NO FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_catalog_item NO FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_item_variant NO FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_order NO FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_order_line_item NO FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_agent_memory NO FORCE ROW LEVEL SECURITY;
	`
	_, err = db.ExecContext(ctx, disableSQL)
	assert.NoError(t, err)

	insertSQL := `
		-- Insert two tenants
		INSERT INTO test_tenant (id, name, domain, tier) VALUES
		('11111111-1111-1111-1111-111111111111', 'Tenant A', 'a.com', 'free'),
		('22222222-2222-2222-2222-222222222222', 'Tenant B', 'b.com', 'pro');

		-- Insert customers
		INSERT INTO test_customer (id, tenant_id, email) VALUES
		('aaaaaaaa-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'custA@a.com'),
		('bbbbbbbb-1111-1111-1111-111111111111', '22222222-2222-2222-2222-222222222222', 'custB@b.com');

		-- Insert catalog items
		INSERT INTO test_catalog_item (id, tenant_id, title, item_type) VALUES
		('cccccccc-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'Item A', 'product'),
		('cccccccc-2222-2222-2222-222222222222', '22222222-2222-2222-2222-222222222222', 'Item B', 'service');

		-- Insert item variants
		INSERT INTO test_item_variant (id, tenant_id, catalog_item_id, sku, price) VALUES
		('dddddddd-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'cccccccc-1111-1111-1111-111111111111', 'SKU-A', 10.00),
		('dddddddd-2222-2222-2222-222222222222', '22222222-2222-2222-2222-222222222222', 'cccccccc-2222-2222-2222-222222222222', 'SKU-B', 20.00);

		-- Insert orders
		INSERT INTO test_order (id, tenant_id, customer_id, status, total_amount) VALUES
		('eeeeeeee-1111-1111-1111-111111111111', '11111111-1111-1111-1111-111111111111', 'aaaaaaaa-1111-1111-1111-111111111111', 'confirmed', 10.00),
		('eeeeeeee-2222-2222-2222-222222222222', '22222222-2222-2222-2222-222222222222', 'bbbbbbbb-1111-1111-1111-111111111111', 'confirmed', 20.00);
	`
	_, err = db.ExecContext(ctx, insertSQL)
	assert.NoError(t, err)

	enableSQL := `
		ALTER TABLE test_tenant FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_customer FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_catalog_item FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_item_variant FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_order FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_order_line_item FORCE ROW LEVEL SECURITY;
		ALTER TABLE test_agent_memory FORCE ROW LEVEL SECURITY;
	`
	_, err = db.ExecContext(ctx, enableSQL)
	assert.NoError(t, err)


	// 3. Test Tenant A
	connA, err := db.Conn(ctx)
	assert.NoError(t, err)
	defer connA.Close()

	_, err = connA.ExecContext(ctx, "SET ROLE rls_test_user")
	assert.NoError(t, err)
	_, err = connA.ExecContext(ctx, "SET app.current_tenant = '11111111-1111-1111-1111-111111111111'")
	assert.NoError(t, err)

	rowsA, err := connA.QueryContext(ctx, "SELECT id FROM test_order")
	assert.NoError(t, err)
	defer rowsA.Close()

	var countA int
	for rowsA.Next() {
		countA++
	}
	assert.Equal(t, 1, countA, "Tenant A should see exactly 1 order")

	// 4. Test Tenant B
	connB, err := db.Conn(ctx)
	assert.NoError(t, err)
	defer connB.Close()

	_, err = connB.ExecContext(ctx, "SET ROLE rls_test_user")
	assert.NoError(t, err)
	_, err = connB.ExecContext(ctx, "SET app.current_tenant = '22222222-2222-2222-2222-222222222222'")
	assert.NoError(t, err)

	rowsB, err := connB.QueryContext(ctx, "SELECT id FROM test_order")
	assert.NoError(t, err)
	defer rowsB.Close()

	var countB int
	for rowsB.Next() {
		countB++
	}
	assert.Equal(t, 1, countB, "Tenant B should see exactly 1 order")

	// 5. Test empty tenant context (should see 0 records if query manipulates API without tenant context)
	// PostgreSQL requires valid UUID for casting in our policy, so we'll test with a fake UUID.
	connEmpty, err := db.Conn(ctx)
	assert.NoError(t, err)
	defer connEmpty.Close()

	_, err = connEmpty.ExecContext(ctx, "SET ROLE rls_test_user")
	assert.NoError(t, err)
	_, err = connEmpty.ExecContext(ctx, "SET app.current_tenant = ''")
	assert.NoError(t, err)

	rowsEmpty, err := connEmpty.QueryContext(ctx, "SELECT id FROM test_order")
	assert.NoError(t, err)
	defer rowsEmpty.Close()

	var countEmpty int
	for rowsEmpty.Next() {
		countEmpty++
	}
	assert.Equal(t, 0, countEmpty, "Unknown tenant context should see 0 orders")

	// Cleanup
	_, err = db.ExecContext(ctx, `
		DROP TABLE IF EXISTS test_agent_memory CASCADE;
		DROP TABLE IF EXISTS test_order_line_item CASCADE;
		DROP TABLE IF EXISTS test_order CASCADE;
		DROP TABLE IF EXISTS test_item_variant CASCADE;
		DROP TABLE IF EXISTS test_catalog_item CASCADE;
		DROP TABLE IF EXISTS test_customer CASCADE;
		DROP TABLE IF EXISTS test_tenant CASCADE;
		DROP ROLE IF EXISTS rls_test_user;
	`)
	assert.NoError(t, err)
}
