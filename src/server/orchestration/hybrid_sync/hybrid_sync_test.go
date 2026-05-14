package hybrid_sync

import (
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open DB: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE crdt_deltas (
			id TEXT PRIMARY KEY,
			entity_id TEXT,
			data TEXT,
			updated_at TEXT,
			synced BOOLEAN DEFAULT false
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestBackgroundWorkerSync(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	_, err := db.Exec(`INSERT INTO crdt_deltas (id, entity_id, data, updated_at) VALUES ('1', 'e1', 'd1', '2023-01-01T00:00:00Z')`)
	if err != nil {
		t.Fatalf("Failed to insert data: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	worker := NewBackgroundWorker(db, server.URL, 1*time.Second)

	err = worker.syncDeltas(context.Background())
	if err != nil {
		t.Errorf("Unexpected error: %v", err)
	}

	var synced bool
	err = db.QueryRow("SELECT synced FROM crdt_deltas WHERE id = '1'").Scan(&synced)
	if err != nil {
		t.Fatalf("Failed to query DB: %v", err)
	}
	if !synced {
		t.Errorf("Expected delta to be synced")
	}
}
