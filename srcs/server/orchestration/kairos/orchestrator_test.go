package kairos

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestSharedTaskOrchestrator_CreateTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	o := NewSharedTaskOrchestrator(db)
	ctx := context.Background()

	task := &SharedTaskV4{
		OrganizationID: "org-1",
		Title:          "Test Task",
	}

	mock.ExpectExec("INSERT INTO shared_tasks_v4").
		WithArgs(sqlmock.AnyArg(), task.OrganizationID, task.Title, nil, "PENDING", nil, "P2", nil, nil, "[]").
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = o.CreateTask(ctx, task)
	require.NoError(t, err)
	assert.NotEmpty(t, task.ID)
	assert.Equal(t, "PENDING", task.Status)
	assert.Equal(t, "P2", task.Priority)
	assert.Equal(t, "[]", task.Dependencies)
	assert.NotZero(t, task.CreatedAt)
	assert.NotZero(t, task.UpdatedAt)

	// test with provided ID, Status, Priority, Dependencies
	task2 := &SharedTaskV4{
		ID:             "my-id",
		OrganizationID: "org-2",
		Title:          "Task 2",
		Status:         "ASSIGNED",
		Priority:       "P1",
		Dependencies:   `["dep-1"]`,
	}

	mock.ExpectExec("INSERT INTO shared_tasks_v4").
		WithArgs("my-id", task2.OrganizationID, task2.Title, nil, "ASSIGNED", nil, "P1", nil, nil, `["dep-1"]`).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = o.CreateTask(ctx, task2)
	require.NoError(t, err)
	assert.Equal(t, "my-id", task2.ID)
}

func TestSharedTaskOrchestrator_GetTask(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	o := NewSharedTaskOrchestrator(db)
	ctx := context.Background()

	now := time.Now()
	mock.ExpectQuery("SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at FROM shared_tasks_v4 WHERE id = \\$1 AND organization_id = \\$2").
		WithArgs("my-id", "org-1").
		WillReturnRows(sqlmock.NewRows([]string{"id", "organization_id", "title", "description", "status", "agent_id", "priority", "payload", "parent_plan_id", "dependencies", "created_at", "updated_at"}).
			AddRow("my-id", "org-1", "Task Title", nil, "PENDING", nil, "P2", nil, nil, "[]", now, now))

	task, err := o.GetTask(ctx, "my-id", "org-1")
	require.NoError(t, err)
	assert.NotNil(t, task)
	assert.Equal(t, "my-id", task.ID)
	assert.Equal(t, "org-1", task.OrganizationID)
	assert.Equal(t, "Task Title", task.Title)

	// test not found
	mock.ExpectQuery("SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at FROM shared_tasks_v4 WHERE id = \\$1 AND organization_id = \\$2").
		WithArgs("non-existent", "org-1").
		WillReturnError(sql.ErrNoRows)

	task, err = o.GetTask(ctx, "non-existent", "org-1")
	require.Error(t, err)
	assert.Nil(t, task)
	assert.Equal(t, "task not found", err.Error())

	// test other error
	mock.ExpectQuery("SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at FROM shared_tasks_v4 WHERE id = \\$1 AND organization_id = \\$2").
		WithArgs("error-id", "org-1").
		WillReturnError(sql.ErrConnDone)

	task, err = o.GetTask(ctx, "error-id", "org-1")
	require.Error(t, err)
	assert.Nil(t, task)
}

func TestSharedTaskOrchestrator_UpdateTaskStatus(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	o := NewSharedTaskOrchestrator(db)
	ctx := context.Background()

	mock.ExpectExec("UPDATE shared_tasks_v4 SET status = \\$1, updated_at = CURRENT_TIMESTAMP WHERE id = \\$2 AND organization_id = \\$3").
		WithArgs("COMPLETED", "my-id", "org-1").
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = o.UpdateTaskStatus(ctx, "my-id", "org-1", "COMPLETED")
	require.NoError(t, err)
}
