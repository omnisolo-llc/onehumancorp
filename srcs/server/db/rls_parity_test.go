package db

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

// In SQLite, since we don't have true RLS or session variables like current_setting('app.tenant_id'),
// our provider strategy involves applying tenant_id where clauses dynamically at the query layer
// or simulating RLS via views. Here we verify that a simplified simulation prevents cross-tenant access.

func setupSQLiteDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	setupSQL := `
		CREATE TABLE rls_test_products (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			name TEXT NOT NULL
		);

		INSERT INTO rls_test_products (id, tenant_id, name) VALUES
		('p1', 'tenant_A', 'Product A1'),
		('p2', 'tenant_A', 'Product A2'),
		('p3', 'tenant_B', 'Product B1');
	`
	_, err = db.Exec(setupSQL)
	require.NoError(t, err)
	return db
}

func TestSQLiteRLSParity(t *testing.T) {
	db := setupSQLiteDB(t)
	defer db.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	baseQuery := "SELECT id, name, tenant_id FROM rls_test_products"

	// 1. Test Tenant A
	rowsA, err := simulateRLSQuery(ctx, db, "tenant_A", baseQuery)
	require.NoError(t, err)
	defer rowsA.Close()

	var countA int
	for rowsA.Next() {
		countA++
	}
	assert.Equal(t, 2, countA, "Tenant A should see exactly 2 products via simulated RLS")

	// 2. Test Tenant B
	rowsB, err := simulateRLSQuery(ctx, db, "tenant_B", baseQuery)
	require.NoError(t, err)
	defer rowsB.Close()

	var countB int
	for rowsB.Next() {
		countB++
	}
	assert.Equal(t, 1, countB, "Tenant B should see exactly 1 product via simulated RLS")

	// 3. Test empty tenant context
	rowsEmpty, err := simulateRLSQuery(ctx, db, "", baseQuery)
	require.NoError(t, err)
	defer rowsEmpty.Close()

	var countEmpty int
	for rowsEmpty.Next() {
		countEmpty++
	}
	assert.Equal(t, 0, countEmpty, "Empty tenant context should see 0 products via simulated RLS")
}
