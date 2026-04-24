package repositories

import (
	"context"
	"testing"
	"time"

	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/db/models"
)

func TestCreateAndGetEpicTasks(t *testing.T) {
	pool := db.NewTestProvider(t)
	// Apply schema for the test. We can't use db.DB RunMigrations cleanly without the full wrapper.
	// We'll create the exact required table.
	_, err := pool.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS epics (
			id TEXT PRIMARY KEY,
			title TEXT,
			description TEXT,
			status TEXT,
			organization_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = pool.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS tasks (
			id TEXT PRIMARY KEY,
			epic_id TEXT NOT NULL,
			title TEXT,
			status TEXT NOT NULL,
			assigned_agent TEXT,
			organization_id TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			FOREIGN KEY(epic_id) REFERENCES epics(id)
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	ctx := context.Background()
	repo := NewEpicTaskRepository(pool)

	epicID := uuid.New().String()
	epic := &models.Epic{
		ID: epicID,
	}

	err = repo.CreateEpic(ctx, epic)
	require.NoError(t, err, "failed to create epic")

	agentID := "test_agent"
	task := &models.EpicTask{
		ID:            uuid.New().String(),
		EpicID:        epicID,
		Title:         "Test Task",
		Status:        "PENDING",
		AssignedAgent: &agentID,
		CreatedAt:     time.Now().UTC(),
		UpdatedAt:     time.Now().UTC(),
	}

	err = repo.CreateTask(ctx, task)
	require.NoError(t, err, "failed to create task")

	// Create another task for same epic
	task2 := &models.EpicTask{
		ID:            uuid.New().String(),
		EpicID:        epicID,
		Title:         "Test Task 2",
		Status:        "PENDING",
		CreatedAt:     time.Now().UTC().Add(time.Second),
		UpdatedAt:     time.Now().UTC().Add(time.Second),
	}
	err = repo.CreateTask(ctx, task2)
	require.NoError(t, err, "failed to create second task")

	// Create a task for another epic
	otherEpicID := uuid.New().String()
	otherEpic := &models.Epic{ID: otherEpicID}
	err = repo.CreateEpic(ctx, otherEpic)
	require.NoError(t, err)

	task3 := &models.EpicTask{
		ID:            uuid.New().String(),
		EpicID:        otherEpicID,
		Title:         "Other Epic Task",
		Status:        "PENDING",
		CreatedAt:     time.Now().UTC(),
		UpdatedAt:     time.Now().UTC(),
	}
	err = repo.CreateTask(ctx, task3)
	require.NoError(t, err)

	tasks, err := repo.GetTasksByEpicID(ctx, epicID)
	require.NoError(t, err, "failed to get tasks")

	assert.Len(t, tasks, 2)
	assert.Equal(t, task.ID, tasks[0].ID)
	assert.Equal(t, task2.ID, tasks[1].ID)

	assert.Equal(t, task.Title, tasks[0].Title)
	assert.Equal(t, task.Status, tasks[0].Status)
	assert.Equal(t, task.AssignedAgent, tasks[0].AssignedAgent)
}
