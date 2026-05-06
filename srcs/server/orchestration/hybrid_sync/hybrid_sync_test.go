package hybrid_sync

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"
	"time"

	"onehumancorp/srcs/server/orchestration"

	"github.com/DATA-DOG/go-sqlmock"
	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT,
			parent_plan_id TEXT,
			dependencies TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	require.NoError(t, err)

	return db
}

func TestSyncLocalToCloud(t *testing.T) {
	// 1. Setup Local SQLite Store
	localDB := setupTestDB(t)
	defer localDB.Close()
	localStore := orchestration.NewSqliteTaskStore(localDB)

	// 2. Setup Cloud Postgres Store with Mock
	cloudDB, mock, err := sqlmock.New()
	require.NoError(t, err)
	defer cloudDB.Close()
	cloudStore := orchestration.NewPostgresTaskStore(cloudDB)

	ctx := context.Background()

	// 3. Insert mock mission in local SQLite store
	payloadStr := `{"rag_context": "secret [PRIVATE:PII] mission data"}`
	rawPayload := json.RawMessage(payloadStr)
	desc := "A private RAG mission"

	mission := &orchestration.SharedTask{
		OrganizationID: "org-1",
		Title:          "Private Mission",
		Description:    &desc,
		Status:         "PENDING",
		Priority:       "P1",
		Payload:        &rawPayload,
	}

	err = localStore.CreateTask(ctx, mission)
	require.NoError(t, err)

	savedLocalMission, err := localStore.GetTask(ctx, mission.ID)
	require.NoError(t, err)
	assert.NotNil(t, savedLocalMission)

	// 4. Create Synchronizer
	synchronizer := NewMissionSynchronizer(localStore, cloudStore)

	// 5. Expect Cloud Store interaction (Postgres CreateTask)
	sanitizedPayloadStr := `{"rag_context": "secret [REDACTED] mission data"}`
	sanitizedPayloadBytes := []byte(sanitizedPayloadStr)
	depsBytes := []byte("[]")

	mock.ExpectBegin()
	mock.ExpectExec("SELECT set_config\\('app.current_tenant', \\$1, true\\)").WithArgs(savedLocalMission.OrganizationID).WillReturnResult(sqlmock.NewResult(0, 0))
	mock.ExpectQuery(`INSERT INTO shared_tasks`).
		WithArgs(
			savedLocalMission.OrganizationID,
			savedLocalMission.Title,
			savedLocalMission.Description,
			savedLocalMission.Status,
			savedLocalMission.AgentID,
			savedLocalMission.Priority,
			sanitizedPayloadBytes,
			savedLocalMission.ParentPlanID,
			depsBytes,
		).
		WillReturnRows(sqlmock.NewRows([]string{"id", "created_at", "updated_at"}).
			AddRow(savedLocalMission.ID, time.Now(), time.Now()))
	mock.ExpectCommit()

	// 6. Execute Sync
	err = synchronizer.SyncLocalToCloud(ctx, savedLocalMission)
	require.NoError(t, err)

	// Ensure payload was NOT sanitized locally on the mission object since we used a copy
	assert.Equal(t, string(*savedLocalMission.Payload), payloadStr)

	// 7. Verify mock expectations
	err = mock.ExpectationsWereMet()
	require.NoError(t, err)
}
