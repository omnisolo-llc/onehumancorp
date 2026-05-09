package onboarding

import (
	"context"
	"database/sql"
	"testing"

	"onehumancorp/srcs/server/orchestration"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open database: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE tenants (
			id TEXT PRIMARY KEY,
			owner_email TEXT,
			tier TEXT,
			name TEXT,
			category TEXT,
			description TEXT,
			status TEXT,
			created_at DATETIME,
			updated_at DATETIME
		);
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT,
			title TEXT,
			description TEXT,
			status TEXT,
			agent_id TEXT,
			priority TEXT,
			payload BLOB,
			parent_plan_id TEXT,
			dependencies BLOB,
			created_at DATETIME,
			updated_at DATETIME
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create tables: %v", err)
	}

	return db
}

func TestOnboardingFlow(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	tenantStore := NewSqliteTenantStore(db)
	taskStore := orchestration.NewSqliteTaskStore(db)
	service := NewService(tenantStore, taskStore)
	ctx := context.Background()

	// 1. Start Onboarding
	req := OnboardingRequest{
		Name:        "Maya's Cakes",
		Category:    "Food",
		Description: "Custom cakes for all occasions",
	}

	res, err := service.StartOnboarding(ctx, req)
	assert.NoError(t, err)
	assert.NotNil(t, res)
	assert.NotEmpty(t, res.TenantID)
	assert.Equal(t, "PROVISIONING", res.Status)

	// 2. Check Tasks Dispatched
	tasks, err := taskStore.GetTasksByOrganization(ctx, res.TenantID)
	assert.NoError(t, err)
	assert.Len(t, tasks, 3)

	for _, task := range tasks {
		assert.Equal(t, "PENDING", task.Status)
		assert.Equal(t, "P0", task.Priority)
		assert.Equal(t, res.TenantID, task.OrganizationID)
	}

	// 3. Check Status (Should still be PROVISIONING)
	statusRes, err := service.GetOnboardingStatus(ctx, res.TenantID)
	assert.NoError(t, err)
	assert.Equal(t, "PROVISIONING", statusRes.Status)

	// 4. Mock Agents Completing Tasks
	for _, task := range tasks {
		err := taskStore.UpdateTaskStatus(ctx, task.ID, "COMPLETED")
		assert.NoError(t, err)
	}

	// 5. Check Status Again (Should be READY)
	statusRes2, err := service.GetOnboardingStatus(ctx, res.TenantID)
	assert.NoError(t, err)
	assert.Equal(t, "READY", statusRes2.Status)

	// Verify Tenant is updated in DB
	tenant, err := tenantStore.GetTenant(ctx, res.TenantID)
	assert.NoError(t, err)
	assert.Equal(t, "READY", tenant.Status)
}
