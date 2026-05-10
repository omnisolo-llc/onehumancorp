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

func TestPostgresRLSIntegration(t *testing.T) {
	dbURL := os.Getenv("DATABASE_URL")
	if dbURL == "" {
		dbURL = "postgres://postgres:postgres@localhost:5432/test?sslmode=disable"
	}

	db, err := sql.Open("postgres", dbURL)
	if err != nil {
		t.Skipf("Skipping integration test: %v", err)
	}
	defer db.Close()

	if err := db.Ping(); err != nil {
		t.Skipf("Skipping integration test due to ping failure: %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// 1. Setup tables
	setupSQL := `
		DROP TABLE IF EXISTS rls_test_products;
		CREATE TABLE rls_test_products (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			name TEXT NOT NULL
		);
		ALTER TABLE rls_test_products ENABLE ROW LEVEL SECURITY;
		DROP POLICY IF EXISTS rls_test_products_policy ON rls_test_products;
		CREATE POLICY rls_test_products_policy ON rls_test_products
			USING (tenant_id = current_setting('app.current_tenant', true));
	`
	_, err = db.ExecContext(ctx, setupSQL)
	assert.NoError(t, err)

	// 2. Insert test data
	insertSQL := `
		INSERT INTO rls_test_products (id, tenant_id, name) VALUES
		('p1', 'tenant_A', 'Product A1'),
		('p2', 'tenant_A', 'Product A2'),
		('p3', 'tenant_B', 'Product B1')
	`
	_, err = db.ExecContext(ctx, insertSQL)
	assert.NoError(t, err)

	// 3. Test Tenant A
	connA, err := db.Conn(ctx)
	assert.NoError(t, err)
	defer connA.Close()

	_, err = connA.ExecContext(ctx, "SET app.current_tenant = 'tenant_A'")
	assert.NoError(t, err)

	rowsA, err := connA.QueryContext(ctx, "SELECT id, name FROM rls_test_products")
	assert.NoError(t, err)
	defer rowsA.Close()

	var countA int
	for rowsA.Next() {
		countA++
	}
	assert.Equal(t, 2, countA, "Tenant A should see exactly 2 products")

	// 4. Test Tenant B
	connB, err := db.Conn(ctx)
	assert.NoError(t, err)
	defer connB.Close()

	_, err = connB.ExecContext(ctx, "SET app.current_tenant = 'tenant_B'")
	assert.NoError(t, err)

	rowsB, err := connB.QueryContext(ctx, "SELECT id, name FROM rls_test_products")
	assert.NoError(t, err)
	defer rowsB.Close()

	var countB int
	for rowsB.Next() {
		countB++
	}
	assert.Equal(t, 1, countB, "Tenant B should see exactly 1 product")

	// 5. Test empty tenant context (should see 0 records if query manipulates API without tenant context)
	connEmpty, err := db.Conn(ctx)
	assert.NoError(t, err)
	defer connEmpty.Close()

	_, err = connEmpty.ExecContext(ctx, "SET app.current_tenant = ''")
	assert.NoError(t, err)

	rowsEmpty, err := connEmpty.QueryContext(ctx, "SELECT id, name FROM rls_test_products")
	assert.NoError(t, err)
	defer rowsEmpty.Close()

	var countEmpty int
	for rowsEmpty.Next() {
		countEmpty++
	}
	assert.Equal(t, 0, countEmpty, "Empty tenant context should see 0 products")

	// Cleanup
	_, err = db.ExecContext(ctx, "DROP TABLE IF EXISTS rls_test_products")
	assert.NoError(t, err)
}
