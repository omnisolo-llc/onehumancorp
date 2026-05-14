package db

import (
	"context"
	"os"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestProvider_TenantIsolation_E2E(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	provider := &Provider{DB: db}

	ctxTenantA := context.WithValue(context.Background(), TenantKey, "tenant_a")

	task := &Task{
		ID:       "task-isolated",
		TenantID: "tenant_a",
		Status:   "PENDING",
	}

	mock.ExpectExec(`INSERT INTO tasks \(id, tenant_id, status, created_at, updated_at\)`).
		WithArgs("task-isolated", "tenant_a", "PENDING").
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = provider.CreateTask(ctxTenantA, task)
	assert.NoError(t, err)

	// Now try to claim it with Tenant A
	mock.ExpectQuery(`SELECT status FROM tasks WHERE id = \? AND tenant_id = \?`).
		WithArgs("task-isolated", "tenant_a").
		WillReturnRows(sqlmock.NewRows([]string{"status"}).AddRow("PENDING"))

	mock.ExpectExec(`UPDATE tasks SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = \? AND status = 'PENDING' AND tenant_id = \?`).
		WithArgs("task-isolated", "tenant_a").
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = provider.ClaimTask(ctxTenantA, "task-isolated")
	assert.NoError(t, err)

	// Context without tenant ID should fail or not see the data based on how Provider acts
	ctxNoTenant := context.Background()

	err = provider.ClaimTask(ctxNoTenant, "task-isolated")
	assert.Error(t, err)
	assert.Contains(t, err.Error(), "missing tenant_id")

	assert.NoError(t, mock.ExpectationsWereMet())
}
