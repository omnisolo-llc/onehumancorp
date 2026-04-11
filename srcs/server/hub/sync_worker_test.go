package hub

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestSyncWorkerHTTP(t *testing.T) {
	prov := setupTestDB(t)
	defer prov.Close()

	srv := NewSQLRAGSyncService(prov)
	ctx := context.Background()

	_, err := prov.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status) VALUES ('1', 'test1', 'pending'), ('2', 'test2', 'pending')")
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	// Create a test HTTP server to simulate the cloud endpoint
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"ok"}`))
	}))
	defer server.Close()

	worker := NewSyncWorker(prov, srv, true, server.URL)

	// manually run a single iteration instead of Start() to avoid infinite loop block in test
	worker.runSync(ctx)

	// verify that the records were marked synced
	pending, err := srv.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 0 {
		t.Fatalf("expected 0 pending records after sync, got %d", len(pending))
	}
}
