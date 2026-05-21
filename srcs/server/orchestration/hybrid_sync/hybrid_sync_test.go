package hybrid_sync

import (
	"context"
	"database/sql"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
	_ "github.com/mattn/go-sqlite3"
)

func TestSyncLocalToCloud(t *testing.T) {
	// Setup real SQLite memory db
	sqliteDB, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqliteDB.Close()

	// Create table and insert a dummy mission in SQLite
	_, err = sqliteDB.Exec(`
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
			tenant_id TEXT
		);
		INSERT INTO agent_missions (id, status, payload, tenant_id)
		VALUES ('mission-1', 'PENDING', '{"rag_context": "User email is test@example.com"}', 'org-1');
	`)
	if err != nil {
		t.Fatalf("failed to setup sqlite table: %v", err)
	}

	// Setup mock Postgres db
	pgDB, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("failed to create sqlmock: %v", err)
	}
	defer pgDB.Close()

	// Initialize Daemon
	daemon := NewSyncDaemon(sqliteDB, pgDB)

	// We expect the scrubber to remove "test@example.com"
	expectedPayload := `{"rag_context":"User email is [REDACTED]"}`

	// Expect the INSERT query in Postgres
	mock.ExpectExec(`INSERT INTO agent_missions \(id, status, payload, tenant_id\)`).
		WithArgs("mission-1", "PENDING", expectedPayload, "org-1").
		WillReturnResult(sqlmock.NewResult(1, 1))

	// The mission object we are syncing
	mission := &AgentMission{
		MissionID:      "mission-1",
		OrganizationID: "org-1",
		Status:         "PENDING",
	}
	mission.Payload.RagContext = "User email is test@example.com"

	// Run Sync
	err = daemon.SyncLocalToCloud(context.Background(), mission)
	if err != nil {
		t.Fatalf("SyncLocalToCloud failed: %v", err)
	}

	// Verify mock expectations
	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}
