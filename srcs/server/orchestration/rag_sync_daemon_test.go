package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestRagSyncDaemon_ProcessSync(t *testing.T) {
	defer ClearSemaphore()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create swarm_memory_embeddings table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context, sync_status)
		VALUES
			('m1', '{"fact":"sky is blue"}', 'pending'),
			('m2', '{"fact":"water is wet"}', 'synced'),
			('m3', '{"fact":"grass is green"}', NULL)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	var receivedPayload map[string]interface{}
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/mcp/rag/sync" && r.Method == http.MethodPost {
			if err := json.NewDecoder(r.Body).Decode(&receivedPayload); err != nil {
				w.WriteHeader(http.StatusBadRequest)
				return
			}
			w.WriteHeader(http.StatusOK)
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer srv.Close()

	daemon := NewRagSyncDaemon(dbWrapper, 1*time.Minute, srv.URL)

	daemon.ProcessSync(context.Background())

	recordsInterface, ok := receivedPayload["records"].([]interface{})
	if !ok {
		t.Fatalf("expected records in payload")
	}
	if len(recordsInterface) != 2 {
		t.Fatalf("expected 2 memories to be synced, got %d", len(recordsInterface))
	}

	var syncStatus1, syncStatus2 string
	err = sqlDB.QueryRow("SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'm1'").Scan(&syncStatus1)
	if err != nil {
		t.Fatalf("failed to query m1 sync status: %v", err)
	}
	if syncStatus1 != "synced" {
		t.Errorf("expected m1 to be synced, got %s", syncStatus1)
	}

	err = sqlDB.QueryRow("SELECT sync_status FROM swarm_memory_embeddings WHERE memory_id = 'm3'").Scan(&syncStatus2)
	if err != nil {
		t.Fatalf("failed to query m3 sync status: %v", err)
	}
	if syncStatus2 != "synced" {
		t.Errorf("expected m3 to be synced, got %s", syncStatus2)
	}
}

func TestRagSyncDaemon_StartStop(t *testing.T) {
	defer ClearSemaphore()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_at TIMESTAMP NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create swarm_memory_embeddings table: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	daemon := NewRagSyncDaemon(dbWrapper, 10*time.Millisecond, "http://dummy")

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	daemon.Start(ctx)
	time.Sleep(50 * time.Millisecond)
	daemon.Stop()
	time.Sleep(10 * time.Millisecond)
}
