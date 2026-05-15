package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
)

func TestCreateSharedTask_Postgres(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	tdb := &TasksDB{
		db:   db,
		isPg: true,
	}

	task := &SharedTask{
		ID:             "task-1",
		OrganizationID: "org-1",
		Title:          "Test Task",
		Status:         "PENDING",
		Dependencies:   []string{"dep-1", "dep-2"},
		CreatedAt:      time.Now(),
	}

	mock.ExpectExec("INSERT INTO shared_tasks").
		WithArgs(task.ID, task.OrganizationID, task.Title, task.Status, `["dep-1","dep-2"]`, task.CreatedAt).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = tdb.CreateSharedTask(context.Background(), task)
	assert.NoError(t, err)
	assert.NoError(t, mock.ExpectationsWereMet())
}

func TestClaimTask_Postgres(t *testing.T) {
	db, mock, err := sqlmock.New()
	assert.NoError(t, err)
	defer db.Close()

	tdb := &TasksDB{
		db:   db,
		isPg: true,
	}

	orgID := "org-1"
	createdAt := time.Now()

	mock.ExpectBegin()
	mock.ExpectQuery("SELECT id, title, status, dependencies, created_at FROM shared_tasks").
		WithArgs(orgID).
		WillReturnRows(sqlmock.NewRows([]string{"id", "title", "status", "dependencies", "created_at"}).
			AddRow("task-1", "Test Task", "PENDING", `["dep-1"]`, createdAt))
	mock.ExpectExec("UPDATE shared_tasks SET status = 'IN_PROGRESS' WHERE id = \\$1").
		WithArgs("task-1").
		WillReturnResult(sqlmock.NewResult(1, 1))
	mock.ExpectCommit()

	task, err := tdb.ClaimTask(context.Background(), orgID)
	assert.NoError(t, err)
	assert.NotNil(t, task)
	assert.Equal(t, "task-1", task.ID)
	assert.Equal(t, "IN_PROGRESS", task.Status)
	assert.Equal(t, []string{"dep-1"}, task.Dependencies)
	assert.NoError(t, mock.ExpectationsWereMet())
}
