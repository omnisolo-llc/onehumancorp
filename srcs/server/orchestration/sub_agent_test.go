package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestSubAgentSpawner(t *testing.T) {
	pool, err := db.NewPool("sqlite://file::memory:?cache=shared", 1)
	require.NoError(t, err)
	defer pool.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	err = db.RunMigrations(pool, "../db/migrations")
	require.NoError(t, err)

	spawner := NewDefaultSubAgentSpawner(pool, nil, nil, 2)
	defer spawner.Stop()

	// Test Monitor routine coverage
	go func() {
		_ = spawner.Monitor(ctx)
	}()

	_, err = pool.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, priority, status)
		VALUES ('task-123', 'org-1', 'SubAgent Task', 'DELEGATED', 'PENDING')
	`)
	require.NoError(t, err)

	task := &SharedTask{
		ID:             "task-123",
		OrganizationID: "org-1",
		Priority:       "DELEGATED",
	}

	err = spawner.Spawn(ctx, task)
	require.NoError(t, err)

	// In standalone mode, it sleeps for 100ms.
	time.Sleep(200 * time.Millisecond)

	var status string
	err = pool.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'task-123'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "COMPLETED", status)
}

// mockProvider overrides IsSQLite to false for testing the else branch.
type mockProvider struct {
	db.Provider
}

func (m *mockProvider) IsSQLite() bool {
	return false
}

func TestSubAgentSpawner_CloudMode(t *testing.T) {
	pool, err := db.NewPool("sqlite://file::memory:?cache=shared", 1)
	require.NoError(t, err)
	defer pool.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	err = db.RunMigrations(pool, "../db/migrations")
	require.NoError(t, err)

	mockDB := &mockProvider{Provider: pool}

	spawner := NewDefaultSubAgentSpawner(mockDB, nil, nil, 2)
	defer spawner.Stop()

	_, err = pool.Exec(ctx, `
		INSERT INTO shared_tasks (id, organization_id, title, priority, status)
		VALUES ('task-456', 'org-1', 'SubAgent Task Cloud', 'DELEGATED', 'PENDING')
	`)
	require.NoError(t, err)

	task := &SharedTask{
		ID:             "task-456",
		OrganizationID: "org-1",
		Priority:       "DELEGATED",
	}

	err = spawner.Spawn(ctx, task)
	require.NoError(t, err)

	// Wait for goroutine to execute CompleteTask
	time.Sleep(200 * time.Millisecond)

	var status string
	err = pool.QueryRow(ctx, "SELECT status FROM shared_tasks WHERE id = 'task-456'").Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "COMPLETED", status)
}
