package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestCentrifugeHubBroadcast_LocalFallback(t *testing.T) {
	localMesh := NewLocalTeammateMesh()
	hub := NewCentrifugeHub(localMesh)
	hub.isCloud = false // Force fallback

	msg := MeshMessage{
		AgentID:   "agent1",
		EventType: "test",
		Channel:   "mesh:test",
	}

	data, _ := json.Marshal(msg)
	req := httptest.NewRequest("POST", "/api/v1/mesh/broadcast", bytes.NewBuffer(data))
	w := httptest.NewRecorder()

	var receivedData []byte
	err := localMesh.Subscribe(context.Background(), "mesh:test", func(d []byte) {
		receivedData = d
	})
	require.NoError(t, err)

	hub.HandleBroadcast(w, req)

	assert.Equal(t, http.StatusOK, w.Result().StatusCode)

	// Wait a bit for the async channel delivery
	time.Sleep(50 * time.Millisecond)

	assert.NotNil(t, receivedData)
}

func TestCentrifugeHub_DatabaseLock(t *testing.T) {
	db, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer db.Close()

	store := NewPostgresTaskStoreV4(db)

	mock.ExpectBegin()
	rows := sqlmock.NewRows([]string{"id", "organization_id", "title", "status", "dependencies"}).
		AddRow("task-123", "org-1", "Test Task", "PENDING", "[]")
	mock.ExpectQuery("^SELECT id, organization_id, title, status, dependencies FROM shared_tasks_v4 WHERE status = 'PENDING' AND organization_id = \\$1 FOR UPDATE SKIP LOCKED LIMIT 1$").
		WithArgs("org-1").
		WillReturnRows(rows)

	mock.ExpectExec("^UPDATE shared_tasks_v4 SET status = 'IN_PROGRESS' WHERE id = \\$1$").
		WithArgs("task-123").
		WillReturnResult(sqlmock.NewResult(1, 1))

	mock.ExpectCommit()

	task, err := store.ClaimTaskV4(context.Background(), "org-1", "agent-1")
	require.NoError(t, err)
	assert.NotNil(t, task)
	assert.Equal(t, "task-123", task.ID)
	assert.Equal(t, "IN_PROGRESS", task.Status)

	require.NoError(t, mock.ExpectationsWereMet())
}
