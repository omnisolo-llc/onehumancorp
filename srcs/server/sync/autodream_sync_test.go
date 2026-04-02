package sync

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupDB(t *testing.T) db.Provider {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()
	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to create db provider: %v", err)
	}

	// For SQLite fallback testing, we replace VECTOR(1536) with BLOB as per instructions
	_, err = dbWrapper.Provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS embedding_cache (
			content_hash TEXT PRIMARY KEY,
			embedding BLOB NOT NULL,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			synced_to_cloud BOOLEAN DEFAULT false
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return dbWrapper.Provider
}

func TestProcessForecastTick(t *testing.T) {
	provider := setupDB(t)
	ctx := context.Background()

	_, err := provider.Exec(ctx, "INSERT INTO embedding_cache (content_hash, embedding, synced_to_cloud) VALUES ($1, $2, $3)", "hash1", []byte("vector1"), false)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}
	_, err = provider.Exec(ctx, "INSERT INTO embedding_cache (content_hash, embedding, synced_to_cloud) VALUES ($1, $2, $3)", "hash2", []byte("vector2"), true)
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	var syncedCount int
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/v1/sync/autodream" {
			t.Errorf("expected path /api/v1/sync/autodream, got %s", r.URL.Path)
		}
		if r.Method != "POST" {
			t.Errorf("expected method POST, got %s", r.Method)
		}
		var payload []AutoDream
		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			t.Errorf("failed to decode payload: %v", err)
		}
		syncedCount += len(payload)

		if len(payload) != 1 || payload[0].ContentHash != "hash1" {
			t.Errorf("unexpected payload: %+v", payload)
		}

		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	engine := NewAutoDreamSyncEngine(provider, ts.URL)
	engine.ProcessForecastTick()

	if syncedCount != 1 {
		t.Errorf("expected 1 item to be synced, got %d", syncedCount)
	}

	var isSynced bool
	err = provider.QueryRow(ctx, "SELECT synced_to_cloud FROM embedding_cache WHERE content_hash = $1", "hash1").Scan(&isSynced)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if !isSynced {
		t.Errorf("expected hash1 to be marked as synced")
	}
}
