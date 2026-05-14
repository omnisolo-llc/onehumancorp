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

func TestPostgresRLSE2EIntegration(t *testing.T) {
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
		DROP TABLE IF EXISTS rls_e2e_agents;
		CREATE TABLE rls_e2e_agents (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			department TEXT NOT NULL
		);
		ALTER TABLE rls_e2e_agents ENABLE ROW LEVEL SECURITY;
		ALTER TABLE rls_e2e_agents FORCE ROW LEVEL SECURITY;
		DROP POLICY IF EXISTS rls_e2e_agents_policy ON rls_e2e_agents;
		CREATE POLICY rls_e2e_agents_policy ON rls_e2e_agents
			FOR ALL
			USING (tenant_id = nullif(current_setting('app.current_tenant', true), '')::text);
	`
	_, err = db.ExecContext(ctx, setupSQL)
	assert.NoError(t, err)

	// 2. Insert test data
	disableSQL := `ALTER TABLE rls_e2e_agents NO FORCE ROW LEVEL SECURITY;`
	_, err = db.ExecContext(ctx, disableSQL)
	assert.NoError(t, err)

	insertSQL := `
		INSERT INTO rls_e2e_agents (id, tenant_id, department) VALUES
		('a1', 'tenant_A', 'sales'),
		('a2', 'tenant_B', 'support')
	`
	_, err = db.ExecContext(ctx, insertSQL)
	assert.NoError(t, err)

	enableSQL := `ALTER TABLE rls_e2e_agents FORCE ROW LEVEL SECURITY;`
	_, err = db.ExecContext(ctx, enableSQL)
	assert.NoError(t, err)

	setupRoleSQL := `
		DO $$
		BEGIN
		  IF NOT EXISTS (
			SELECT FROM pg_catalog.pg_roles
			WHERE  rolname = 'rls_test_user') THEN
			CREATE ROLE rls_test_user LOGIN;
		  END IF;
		END
		$$;
		GRANT ALL PRIVILEGES ON TABLE rls_e2e_agents TO rls_test_user;
	`
	_, err = db.ExecContext(ctx, setupRoleSQL)
	assert.NoError(t, err)

	// 3. Test Tenant A
	connA, err := db.Conn(ctx)
	assert.NoError(t, err)
	defer connA.Close()

	_, err = connA.ExecContext(ctx, "SET ROLE rls_test_user")
	assert.NoError(t, err)
	_, err = connA.ExecContext(ctx, "SELECT set_config('app.current_tenant', 'tenant_A', true)")
	assert.NoError(t, err)

	rowsA, err := connA.QueryContext(ctx, "SELECT id, department FROM rls_e2e_agents")
	assert.NoError(t, err)
	defer rowsA.Close()

	var countA int
	for rowsA.Next() {
		countA++
	}
	assert.Equal(t, 1, countA, "Tenant A should see exactly 1 agent")

	// 4. Test API manipulation (Tenant A trying to read without setting context / bad context)
	connManipulate, err := db.Conn(ctx)
	assert.NoError(t, err)
	defer connManipulate.Close()

	_, err = connManipulate.ExecContext(ctx, "SET ROLE rls_test_user")
	assert.NoError(t, err)
	_, err = connManipulate.ExecContext(ctx, "SELECT set_config('app.current_tenant', '', true)")
	assert.NoError(t, err)

	rowsManipulate, err := connManipulate.QueryContext(ctx, "SELECT id FROM rls_e2e_agents")
	assert.NoError(t, err)
	defer rowsManipulate.Close()

	var countEmpty int
	for rowsManipulate.Next() {
		countEmpty++
	}
	assert.Equal(t, 0, countEmpty, "Empty tenant context should see 0 agents")

	// Cleanup
	_, err = db.ExecContext(ctx, "DROP TABLE IF EXISTS rls_e2e_agents")
	assert.NoError(t, err)
}
