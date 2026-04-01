package orchestration

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"testing"
)

func TestSIPDB_RetryContextCancel(t *testing.T) {
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer db.Close()

	ctx, cancel := context.WithCancel(context.Background())
	cancel() // cancel immediately

	// Close the DB to force a transient/permanent error on exec
	db.Close()

	err = db.UpdateMemory(ctx, "k", "v")
	if err == nil || err != context.Canceled {
		t.Fatalf("expected context.Canceled, got %v", err)
	}
}

func TestSIPDB_NewSIPDB_InvalidPath(t *testing.T) {
	// sqlite generally allows weird paths, but /root/locked.db should fail to open or init
	db, err := NewSIPDB("/root/locked.db")
	if err == nil {
		db.Close()
		t.Fatalf("expected error for invalid db path")
	}
}

func TestSIPDB_SyncMemory_NoRows(t *testing.T) {
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to init: %v", err)
	}
	defer db.Close()

	ctx := context.Background()
	val, err := db.SyncMemory(ctx, "non_existent_key")
	if err != nil {
		t.Fatalf("expected no error for sql.ErrNoRows, got %v", err)
	}
	if val != "" {
		t.Fatalf("expected empty string, got %s", val)
	}
}

func TestSIPDB_GetPendingMissions_Fallback(t *testing.T) {
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to init: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// Manually insert malformed JSON task
	_, err = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('m2', 'PENDING', '{\"role\":\"ROLE\",\"task\":\"invalid_json\"}')")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	missions, err := db.GetPendingMissions(ctx, "ROLE")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(missions) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions))
	}
	if missions[0].Content != "\"invalid_json\"" || missions[0].ID != "m2" || missions[0].Type != EventTask {
		t.Fatalf("fallback msg parsing failed: %+v", missions[0])
	}
}

func TestSIPDB_GetPendingMissions_MissingID(t *testing.T) {
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to init: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// Manually insert JSON without ID
	_, err = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('m3', 'PENDING', '{\"role\":\"ROLE\",\"task\":{\"type\":\"TASK\"}}')")
	if err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	missions, err := db.GetPendingMissions(ctx, "ROLE")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(missions) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions))
	}
	if missions[0].ID != "m3" {
		t.Fatalf("fallback msg ID parsing failed: %+v", missions[0])
	}
}

func TestSIPDB_CompleteMission_NotFound(t *testing.T) {
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to init: %v", err)
	}
	defer db.Close()

	ctx := context.Background()
	err = db.CompleteMission(ctx, "non_existent_mission")
	if err == nil || err.Error() != "mission not found" {
		t.Fatalf("expected 'mission not found', got %v", err)
	}
}

func TestSIPDB_DBClosedErrors(t *testing.T) {
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to init: %v", err)
	}
	db.Close() // close db to force errors

	ctx := context.Background()

	_, err = db.SyncMemory(ctx, "k")
	if err == nil {
		t.Fatalf("expected error on SyncMemory after close")
	}

	_, err = db.GetPendingMissions(ctx, "ROLE")
	if err == nil {
		t.Fatalf("expected error on GetPendingMissions after close")
	}

	err = db.CompleteMission(ctx, "m1")
	if err == nil {
		t.Fatalf("expected error on CompleteMission after close")
	}

	err = db.UpdateMemory(ctx, "k", "v")
	if err == nil {
		t.Fatalf("expected error on UpdateMemory after close")
	}

	err = db.Heartbeat(ctx, "a1", "r1", "s1")
	if err == nil {
		t.Fatalf("expected error on Heartbeat after close")
	}

	err = db.DelegateMission(ctx, "m1", "r1", Message{ID: "m1"})
	if err == nil {
		t.Fatalf("expected error on DelegateMission after close")
	}
}

// Add necessary coverage tests without deleting anything

// TestSIPDB_GetCapabilityPlugins_ScanError tests handling of scan errors
func TestSIPDB_GetCapabilityPlugins_ScanError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()
	_, err = db.db.Exec("DROP TABLE capability_plugins")
	if err != nil {
		t.Fatal(err)
	}
	_, err = db.db.Exec("CREATE TABLE capability_plugins (plugin_id TEXT, name TEXT, version TEXT, manifest_url TEXT, status TEXT, registered_at TEXT)")
	if err != nil {
		t.Fatal(err)
	}
	_, err = db.db.Exec("INSERT INTO capability_plugins (plugin_id, name, version, manifest_url, status, registered_at) VALUES (NULL, 'name', 'version', 'url', 'status', '2023-01-01 00:00:00')")
	if err != nil {
		t.Fatal(err)
	}

	_, err = db.GetCapabilityPlugins(ctx, "")
	if err == nil {
		t.Fatal("Expected scan error due to NULL plugin_id")
	}
}

// TestSIPDB_GetEpisodicMemoriesByPlugin_ScanError tests handling of scan errors
func TestSIPDB_GetEpisodicMemoriesByPlugin_ScanError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()
	_, err = db.db.Exec("DROP TABLE swarm_memory_embeddings")
	if err != nil {
		t.Fatal(err)
	}
	_, err = db.db.Exec("CREATE TABLE swarm_memory_embeddings (memory_id TEXT, context TEXT, vector_embedding BLOB, source_plugin TEXT, created_at TEXT)")
	if err != nil {
		t.Fatal(err)
	}
	_, err = db.db.Exec("INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at) VALUES (NULL, 'ctx', NULL, 'plugin', '2023-01-01 00:00:00')")
	if err != nil {
		t.Fatal(err)
	}

	_, err = db.GetEpisodicMemoriesByPlugin(ctx, "")
	if err == nil {
		t.Fatal("Expected scan error due to NULL memory_id")
	}
}

// Test GetPendingMissions Scan error for 165-166
func TestSIPDB_GetPendingMissions_ScanError_Coverage(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()
	ctx := context.Background()
	_, _ = db.db.ExecContext(ctx, "DROP TABLE agent_missions")
	_, _ = db.db.ExecContext(ctx, "CREATE TABLE agent_missions (id TEXT, status TEXT, payload TEXT, created_at DATETIME)")
	_, _ = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES (NULL, 'PENDING', '{\"role\":\"ROLE\"}')")
	_, err = db.GetPendingMissions(ctx, "ROLE")
	if err == nil {
		t.Fatal("Expected scan error due to NULL task")
	}
}




func TestSIPDB_SyncMissions_RagContextAndHeader(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test_sync_rag.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// Insert a pending mission with rag_context
	payload := map[string]interface{}{
		"role": "AGENT",
		"task": map[string]interface{}{
			"Content": "Some task",
		},
		"rag_context": "highly_sensitive_data",
		"other_key": "safe_data",
	}
	payloadBytes, _ := json.Marshal(payload)

	_, err = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('m-rag-1', 'PENDING', ?)", string(payloadBytes))
	if err != nil {
		t.Fatalf("Failed to insert mission: %v", err)
	}

	var reqBody []byte
	var reqHeader string
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		reqBody, _ = io.ReadAll(r.Body)
		reqHeader = r.Header.Get("X-Conflict-Resolution")
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	syncedCount, err := db.SyncMissions(ctx, ts.URL)
	if err != nil {
		t.Fatalf("SyncMissions failed: %v", err)
	}
	if syncedCount != 1 {
		t.Fatalf("Expected 1 synced record, got %d", syncedCount)
	}

	if reqHeader != "client-wins" {
		t.Fatalf("Expected X-Conflict-Resolution header to be 'client-wins', got '%s'", reqHeader)
	}

	var parsedBody map[string]interface{}
	if err := json.Unmarshal(reqBody, &parsedBody); err != nil {
		t.Fatalf("Failed to parse request body: %v", err)
	}

	if _, ok := parsedBody["rag_context"]; ok {
		t.Fatalf("Expected rag_context to be stripped, but it was present in payload: %s", string(reqBody))
	}

	if parsedBody["other_key"] != "safe_data" {
		t.Fatalf("Expected other_key to be preserved, got %v", parsedBody["other_key"])
	}

	// Verify status updated to SYNCED
	var status string
	err = db.db.QueryRowContext(ctx, "SELECT status FROM agent_missions WHERE id = 'm-rag-1'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}
	if status != "SYNCED" {
		t.Fatalf("Expected status SYNCED, got %s", status)
	}
}

func TestSIPDB_SyncMissions_ConflictResolution(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test_sync_conflict.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	_, err = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('m-conflict-1', 'PENDING', '{}')")
	if err != nil {
		t.Fatalf("Failed to insert mission: %v", err)
	}

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Simulate 409 Conflict
		w.WriteHeader(http.StatusConflict)
	}))
	defer ts.Close()

	syncedCount, err := db.SyncMissions(ctx, ts.URL)
	if err != nil {
		t.Fatalf("SyncMissions failed: %v", err)
	}
	if syncedCount != 1 {
		t.Fatalf("Expected HTTP 409 Conflict to be treated as successful sync, got %d synced records", syncedCount)
	}

	// Verify status updated to SYNCED
	var status string
	err = db.db.QueryRowContext(ctx, "SELECT status FROM agent_missions WHERE id = 'm-conflict-1'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}
	if status != "SYNCED" {
		t.Fatalf("Expected status SYNCED on conflict, got %s", status)
	}
}
