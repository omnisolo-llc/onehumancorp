package repositories

import (
	"context"
	"testing"
	"time"

	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/db/models"
)

func TestCreateAndGetEpicTasks(t *testing.T) {
	pool := db.NewTestProvider(t)
	db.RunMigrations(pool, "../migrations")

	ctx := context.Background()
	repo := NewEpicTaskRepository(pool)

	epicID := uuid.New().String()
	epic := &models.Epic{
		ID: epicID,
	}

	err := repo.CreateEpic(ctx, epic)
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
