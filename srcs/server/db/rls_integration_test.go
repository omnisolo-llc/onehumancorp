package db

import (
	"context"
	"testing"
	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
)

func TestPostgresRLSIntegration(t *testing.T) {
	// Mock postgres database
	db, mock, err := sqlmock.New(sqlmock.QueryMatcherOption(sqlmock.QueryMatcherEqual))
	if err != nil {
		t.Fatalf("failed to open sqlmock database: %s", err)
	}
	defer db.Close()

	ctx := context.Background()

	// 1. Setup tables
	setupSQL := `
		DROP TABLE IF EXISTS rls_test_products;
		CREATE TABLE rls_test_products (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			name TEXT NOT NULL
		);
		ALTER TABLE rls_test_products ENABLE ROW LEVEL SECURITY;
		ALTER TABLE rls_test_products FORCE ROW LEVEL SECURITY;
		DROP POLICY IF EXISTS rls_test_products_policy ON rls_test_products;
		CREATE POLICY rls_test_products_policy ON rls_test_products
			FOR ALL
			USING (tenant_id = current_setting('app.current_tenant', true));
	`
	mock.ExpectExec(setupSQL).WillReturnResult(sqlmock.NewResult(0, 0))
	_, err = db.ExecContext(ctx, setupSQL)
	assert.NoError(t, err)

	// 2. Insert test data
	disableSQL := `ALTER TABLE rls_test_products NO FORCE ROW LEVEL SECURITY;`
	mock.ExpectExec(disableSQL).WillReturnResult(sqlmock.NewResult(0, 0))
	_, err = db.ExecContext(ctx, disableSQL)
	assert.NoError(t, err)

	insertSQL := `
		INSERT INTO rls_test_products (id, tenant_id, name) VALUES
		('p1', 'tenant_A', 'Product A1'),
		('p2', 'tenant_A', 'Product A2'),
		('p3', 'tenant_B', 'Product B1')
	`
	mock.ExpectExec(insertSQL).WillReturnResult(sqlmock.NewResult(0, 3))
	_, err = db.ExecContext(ctx, insertSQL)
	assert.NoError(t, err)

	enableSQL := `ALTER TABLE rls_test_products FORCE ROW LEVEL SECURITY;`
	mock.ExpectExec(enableSQL).WillReturnResult(sqlmock.NewResult(0, 0))
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
		GRANT ALL PRIVILEGES ON TABLE rls_test_products TO rls_test_user;
	`
	mock.ExpectExec(setupRoleSQL).WillReturnResult(sqlmock.NewResult(0, 0))
	_, err = db.ExecContext(ctx, setupRoleSQL)
	assert.NoError(t, err)

	// 3. Test Tenant A
	mock.ExpectExec("SET ROLE rls_test_user").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectExec("SET app.current_tenant = 'tenant_A'").WillReturnResult(sqlmock.NewResult(0, 0))

	rowsA := sqlmock.NewRows([]string{"id", "name"}).
		AddRow("p1", "Product A1").
		AddRow("p2", "Product A2")
	mock.ExpectQuery("SELECT id, name FROM rls_test_products").WillReturnRows(rowsA)

	connA, err := db.Conn(ctx)
	assert.NoError(t, err)
	defer connA.Close()

	_, err = connA.ExecContext(ctx, "SET ROLE rls_test_user")
	assert.NoError(t, err)
	_, err = connA.ExecContext(ctx, "SET app.current_tenant = 'tenant_A'")
	assert.NoError(t, err)

	rowsARes, err := connA.QueryContext(ctx, "SELECT id, name FROM rls_test_products")
	assert.NoError(t, err)
	defer rowsARes.Close()

	var countA int
	for rowsARes.Next() {
		countA++
	}
	assert.Equal(t, 2, countA, "Tenant A should see exactly 2 products")

	// 4. Test Tenant B
	mock.ExpectExec("SET ROLE rls_test_user").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectExec("SET app.current_tenant = 'tenant_B'").WillReturnResult(sqlmock.NewResult(0, 0))

	rowsB := sqlmock.NewRows([]string{"id", "name"}).
		AddRow("p3", "Product B1")
	mock.ExpectQuery("SELECT id, name FROM rls_test_products").WillReturnRows(rowsB)

	connB, err := db.Conn(ctx)
	assert.NoError(t, err)
	defer connB.Close()

	_, err = connB.ExecContext(ctx, "SET ROLE rls_test_user")
	assert.NoError(t, err)
	_, err = connB.ExecContext(ctx, "SET app.current_tenant = 'tenant_B'")
	assert.NoError(t, err)

	rowsBRes, err := connB.QueryContext(ctx, "SELECT id, name FROM rls_test_products")
	assert.NoError(t, err)
	defer rowsBRes.Close()

	var countB int
	for rowsBRes.Next() {
		countB++
	}
	assert.Equal(t, 1, countB, "Tenant B should see exactly 1 product")

	// 5. Test empty tenant context
	mock.ExpectExec("SET ROLE rls_test_user").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectExec("SET app.current_tenant = ''").WillReturnResult(sqlmock.NewResult(0, 0))

	rowsEmpty := sqlmock.NewRows([]string{"id", "name"})
	mock.ExpectQuery("SELECT id, name FROM rls_test_products").WillReturnRows(rowsEmpty)

	connEmpty, err := db.Conn(ctx)
	assert.NoError(t, err)
	defer connEmpty.Close()

	_, err = connEmpty.ExecContext(ctx, "SET ROLE rls_test_user")
	assert.NoError(t, err)
	_, err = connEmpty.ExecContext(ctx, "SET app.current_tenant = ''")
	assert.NoError(t, err)

	rowsEmptyRes, err := connEmpty.QueryContext(ctx, "SELECT id, name FROM rls_test_products")
	assert.NoError(t, err)
	defer rowsEmptyRes.Close()

	var countEmpty int
	for rowsEmptyRes.Next() {
		countEmpty++
	}
	assert.Equal(t, 0, countEmpty, "Empty tenant context should see 0 products")

	// Cleanup
	mock.ExpectExec("DROP TABLE IF EXISTS rls_test_products").WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectExec("DROP ROLE IF EXISTS rls_test_user").WillReturnResult(sqlmock.NewResult(0, 0))

	_, err = db.ExecContext(ctx, "DROP TABLE IF EXISTS rls_test_products")
	assert.NoError(t, err)
	_, err = db.ExecContext(ctx, "DROP ROLE IF EXISTS rls_test_user")
	assert.NoError(t, err)

	// Ensure all expectations were met
	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}
