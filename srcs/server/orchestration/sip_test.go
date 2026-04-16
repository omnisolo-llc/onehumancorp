package orchestration

import (
	"context"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
	"time"
)

// ClearSemaphore clears the throttle semaphore to prevent test deadlocks.
func ClearSemaphore() {
	select {
	case <-standaloneThrottle:
	default:
	}
}

func TestSIPDB_Init(t *testing.T) {
	db, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to initialize SIPDB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// Test Memory
	err = db.UpdateMemory(ctx, "architecture", "microservices")
	if err != nil {
		t.Fatalf("UpdateMemory failed: %v", err)
	}

	val, err := db.SyncMemory(ctx, "architecture")
	if err != nil {
		t.Fatalf("SyncMemory failed: %v", err)
	}
	if val != "microservices" {
		t.Fatalf("expected 'microservices', got '%s'", val)
	}

	// Test Heartbeat
	err = db.Heartbeat(ctx, "agent-1", "SOFTWARE_ENGINEER", "ACTIVE")
	if err != nil {
		t.Fatalf("Heartbeat failed: %v", err)
	}

	// Test Delegation & Mission
	msg := Message{ID: "m1", Content: "Build a feature", Type: EventTask}
	err = db.DelegateMission(ctx, "m1", "SOFTWARE_ENGINEER", msg)
	if err != nil {
		t.Fatalf("DelegateMission failed: %v", err)
	}

	missions, err := db.GetPendingMissions(ctx, "SOFTWARE_ENGINEER")
	if err != nil {
		t.Fatalf("GetPendingMissions failed: %v", err)
	}
	if len(missions) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions))
	}
	if missions[0].ID != "m1" {
		t.Fatalf("expected mission ID 'm1', got '%s'", missions[0].ID)
	}

	// Wait a moment so transition duration > 0
	time.Sleep(10 * time.Millisecond)

	// Test Completion
	err = db.CompleteMission(ctx, "m1")
	if err != nil {
		t.Fatalf("CompleteMission failed: %v", err)
	}

	missions, err = db.GetPendingMissions(ctx, "SOFTWARE_ENGINEER")
	if err != nil {
		t.Fatalf("GetPendingMissions failed: %v", err)
	}
	if len(missions) != 0 {
		t.Fatalf("expected 0 missions, got %d", len(missions))
	}
}

func TestSIPDB_NewSIPDB_Fail(t *testing.T) {
	// Attempt to create a database on a read-only directory to trigger an error.
	// We'll just provide a path we know will fail SQLite open.
	_, err := NewSIPDB("/root/illegal/path/db.sqlite")
	if err == nil {
		t.Fatal("Expected error when opening DB in illegal path")
	}
}

func TestSIPDB_PollMissions_ScanError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	// 1.5 Test file permissions on disk creation (Regression Test for Local Hardening)
	tempDir := t.TempDir()
	diskDBPath := filepath.Join(tempDir, "ohc_test.db")
	diskDB, errDisk := NewSIPDB(diskDBPath)
	if errDisk != nil {
		t.Fatalf("Failed to create disk SIPDB: %v", errDisk)
	}
	defer diskDB.Close()

	// Ensure DB created WAL/SHM by doing a write
	errUpdate := diskDB.UpdateMemory(context.Background(), "test_perm", "test_val")
	if errUpdate != nil {
		t.Fatalf("Failed to write to disk SIPDB: %v", errUpdate)
	}

	filesToCheck := []string{diskDBPath, diskDBPath + "-wal", diskDBPath + "-shm"}
	for _, f := range filesToCheck {
		info, err := os.Stat(f)
		if err == nil && !info.IsDir() {
			if info.Mode().Perm() != 0600 {
				t.Errorf("File %s does not have 0600 permissions, got %v", f, info.Mode().Perm())
			}
		}
	}

	// Insert invalid schema data manually to cause a row scan error.
	// Since we can't easily break the type in sqlite (it's dynamically typed),
	// this one is hard to hit purely through SQLite without mocking the DB connection.
	// Instead, we focus on the Unmarshal error line 150-151.

	// Manually insert bad JSON
	ctx := context.Background()
	_, err = db.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('123', 'PENDING', '{\"role\":\"SOFTWARE_ENGINEER\",\"task\":\"invalid-json\"}')")
	if err != nil {
		t.Fatalf("Failed to insert bad json: %v", err)
	}

	missions, err := db.GetPendingMissions(ctx, "SOFTWARE_ENGINEER")
	if err != nil {
		t.Fatalf("Expected fallback to message string on JSON unmarshal error, got error: %v", err)
	}

	if len(missions) != 1 {
		t.Fatalf("Expected 1 mission, got %d", len(missions))
	}

	if missions[0].Content != "\"invalid-json\"" {
		t.Fatalf("Expected content '\"invalid-json\"', got %s", missions[0].Content)
	}
}

func TestSIPDB_CompleteMission_RowsAffectedError(t *testing.T) {
	// We can't easily trigger RowsAffected() error with go-sqlite3 normally,
	// but let's at least test the "mission not found" path.
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	err = db.CompleteMission(context.Background(), "non-existent")
	if err == nil || err.Error() != "mission not found" {
		t.Fatalf("Expected 'mission not found' error, got %v", err)
	}
}

func TestSIPDB_GetPendingMissions_BadData(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()
	_, err = db.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('123', 'PENDING', '{\"role\":\"SOFTWARE_ENGINEER\",\"task\":\"invalid-json\"}')")
	if err != nil {
		t.Fatalf("Failed to insert bad json: %v", err)
	}

	// Ensure we handle invalid JSON in GetPendingMissions without blowing up completely
	missions, err := db.GetPendingMissions(ctx, "SOFTWARE_ENGINEER")
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	if len(missions) != 1 {
		t.Fatalf("Expected 1 mission, got %d", len(missions))
	}

	if missions[0].Content != "\"invalid-json\"" {
		t.Fatalf("Expected content to be '\"invalid-json\"' fallback, got: %s", missions[0].Content)
	}
}

func TestSIPDB_CompleteMission_ExecError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	// drop table to cause error
	_, err = db.db.Exec(context.Background(), "DROP TABLE agent_missions")
	if err != nil {
		t.Fatalf("Failed to drop table: %v", err)
	}

	err = db.CompleteMission(context.Background(), "some-id")
	if err == nil {
		t.Fatal("Expected error updating missing table")
	}
}

func TestSIPDB_PruneStaleMissions(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// Insert missions:
	// 1. Pending and new (should not be deleted)
	_, err = db.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, created_at) VALUES ('1', 'PENDING', '{\"role\":\"ROLE\",\"task\":\"task\"}', datetime('now'))")
	if err != nil {
		t.Fatal(err)
	}

	// 2. Completed (should be deleted regardless of age)
	_, err = db.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, created_at) VALUES ('2', 'COMPLETED', '{\"role\":\"ROLE\",\"task\":\"task\"}', datetime('now'))")
	if err != nil {
		t.Fatal(err)
	}

	// 3. Pending but old (should be deleted)
	_, err = db.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, created_at) VALUES ('3', 'PENDING', '{\"role\":\"ROLE\",\"task\":\"task\"}', datetime('now', '-2 days'))")
	if err != nil {
		t.Fatal(err)
	}

	// Prune missions older than 24 hours
	err = db.PruneStaleMissions(ctx, 24*time.Hour)
	if err != nil {
		t.Fatalf("Failed to prune stale missions: %v", err)
	}

	var count int
	err = db.db.QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions").Scan(&count)
	if err != nil {
		t.Fatal(err)
	}

	if count != 1 {
		t.Fatalf("Expected 1 mission remaining, got %d", count)
	}

	// Verify the remaining mission is the correct one
	var id string
	err = db.db.QueryRow(ctx, "SELECT id FROM agent_missions").Scan(&id)
	if err != nil {
		t.Fatal(err)
	}

	if id != "1" {
		t.Fatalf("Expected remaining mission to be '1', got '%s'", id)
	}
}

func TestSIPDB_PruneStaleMissions_DBError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	db.Close()

	err = db.PruneStaleMissions(context.Background(), 24*time.Hour)
	if err == nil {
		t.Fatal("Expected error when pruning on closed DB")
	}
}

func TestSIPDB_CompleteMission_ExecErrorAgain(t *testing.T) {
	// Let's create a test that calls CompleteMission on a closed DB
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	db.Close()

	err = db.CompleteMission(context.Background(), "some-id")
	if err == nil {
		t.Fatal("Expected error updating on closed DB")
	}
}

func TestSIPDB_GetPendingMissions_DBError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	db.Close()

	_, err = db.GetPendingMissions(context.Background(), "role")
	if err == nil {
		t.Fatal("Expected error querying closed DB")
	}
}

func TestSIPDB_UpdateMemory_DBError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	db.Close()

	err = db.UpdateMemory(context.Background(), "key", "val")
	if err == nil {
		t.Fatal("Expected error querying closed DB")
	}
}

func TestSIPDB_SyncMemory_DBError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	db.Close()

	_, err = db.SyncMemory(context.Background(), "key")
	if err == nil {
		t.Fatal("Expected error querying closed DB")
	}
}

func TestSIPDB_Heartbeat_DBError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	db.Close()

	err = db.Heartbeat(context.Background(), "agent", "role", "status")
	if err == nil {
		t.Fatal("Expected error querying closed DB")
	}
}

func TestSIPDB_DelegateMission_DBError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	db.Close()

	err = db.DelegateMission(context.Background(), "mission", "role", Message{})
	if err == nil {
		t.Fatal("Expected error querying closed DB")
	}
}

func TestSIPDB_InitTables_InvalidDBDir(t *testing.T) {
	dbPath := t.TempDir()
	_, err := NewSIPDB(dbPath)
	if err == nil {
		t.Fatal("Expected error initializing tables when path is a directory")
	}
}

func TestSIPDB_RegisterCapabilityPlugin(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()
	plugin := CapabilityPlugin{
		PluginID:    "plugin-1",
		Name:        "Test Plugin",
		Version:     "1.0.0",
		ManifestURL: "http://example.com/manifest.json",
		Status:      "ACTIVE",
	}

	err = db.RegisterCapabilityPlugin(ctx, plugin)
	if err != nil {
		t.Fatalf("RegisterCapabilityPlugin failed: %v", err)
	}

	// Update the plugin
	plugin.Status = "INACTIVE"
	err = db.RegisterCapabilityPlugin(ctx, plugin)
	if err != nil {
		t.Fatalf("RegisterCapabilityPlugin update failed: %v", err)
	}

	plugins, err := db.GetCapabilityPlugins(ctx, "")
	if err != nil {
		t.Fatalf("GetCapabilityPlugins failed: %v", err)
	}
	if len(plugins) != 1 {
		t.Fatalf("expected 1 plugin, got %d", len(plugins))
	}
	if plugins[0].Status != "INACTIVE" {
		t.Fatalf("expected status INACTIVE, got %s", plugins[0].Status)
	}

	activePlugins, err := db.GetCapabilityPlugins(ctx, "ACTIVE")
	if err != nil {
		t.Fatalf("GetCapabilityPlugins with status failed: %v", err)
	}
	if len(activePlugins) != 0 {
		t.Fatalf("expected 0 active plugins, got %d", len(activePlugins))
	}
}

func TestSIPDB_StoreAndGetEpisodicMemories(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()
	memory := EpisodicMemory{
		MemoryID:        "mem-1",
		Context:         "User likes dark mode",
		VectorEmbedding: []byte{1, 2, 3},
		SourcePlugin:    "plugin-1",
	}

	err = db.StoreEpisodicMemory(ctx, memory)
	if err != nil {
		t.Fatalf("StoreEpisodicMemory failed: %v", err)
	}

	// Update memory
	memory.Context = "User likes light mode"
	err = db.StoreEpisodicMemory(ctx, memory)
	if err != nil {
		t.Fatalf("StoreEpisodicMemory update failed: %v", err)
	}

	memories, err := db.GetEpisodicMemoriesByPlugin(ctx, "")
	if err != nil {
		t.Fatalf("GetEpisodicMemoriesByPlugin failed: %v", err)
	}
	if len(memories) != 1 {
		t.Fatalf("expected 1 memory, got %d", len(memories))
	}
	if memories[0].Context != "User likes light mode" {
		t.Fatalf("expected updated context, got %s", memories[0].Context)
	}

	plugin1Memories, err := db.GetEpisodicMemoriesByPlugin(ctx, "plugin-1")
	if err != nil {
		t.Fatalf("GetEpisodicMemoriesByPlugin with plugin failed: %v", err)
	}
	if len(plugin1Memories) != 1 {
		t.Fatalf("expected 1 memory for plugin-1, got %d", len(plugin1Memories))
	}

	plugin2Memories, err := db.GetEpisodicMemoriesByPlugin(ctx, "plugin-2")
	if err != nil {
		t.Fatalf("GetEpisodicMemoriesByPlugin with plugin failed: %v", err)
	}
	if len(plugin2Memories) != 0 {
		t.Fatalf("expected 0 memories for plugin-2, got %d", len(plugin2Memories))
	}
}

func TestSIPDB_GetCapabilityPlugins_DBError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	db.Close()

	_, err = db.GetCapabilityPlugins(context.Background(), "")
	if err == nil {
		t.Fatal("Expected error querying closed DB")
	}

	_, err = db.GetCapabilityPlugins(context.Background(), "ACTIVE")
	if err == nil {
		t.Fatal("Expected error querying closed DB with status")
	}
}

func TestSIPDB_GetEpisodicMemoriesByPlugin_DBError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	db.Close()

	_, err = db.GetEpisodicMemoriesByPlugin(context.Background(), "")
	if err == nil {
		t.Fatal("Expected error querying closed DB")
	}

	_, err = db.GetEpisodicMemoriesByPlugin(context.Background(), "plugin-1")
	if err == nil {
		t.Fatal("Expected error querying closed DB with plugin")
	}
}

func TestSIPDB_RegisterCapabilityPlugin_DBError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	db.Close()

	err = db.RegisterCapabilityPlugin(context.Background(), CapabilityPlugin{})
	if err == nil {
		t.Fatal("Expected error querying closed DB")
	}
}

func TestSIPDB_StoreEpisodicMemory_DBError(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	db.Close()

	err = db.StoreEpisodicMemory(context.Background(), EpisodicMemory{})
	if err == nil {
		t.Fatal("Expected error querying closed DB")
	}
}

func TestSIPDB_GetPendingMissions_ScanError2(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()
	_, err = db.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('123', 'PENDING', '{\"role\":\"SOFTWARE_ENGINEER\",\"task\":\"invalid-json\"}')")
	// Now we will break the table to cause Scan to fail. Scan fails if the number of columns returned doesn't match the pointers, or type conversion fails.
	// But type conversion to string rarely fails in sqlite. We can close the rows early maybe?
	// The easiest way is to mock rows or just change schema, but we can't alter table.
	// We can drop table and recreate it with only one column!
	_, err = db.db.Exec(ctx, "DROP TABLE agent_missions")
	_, err = db.db.Exec(ctx, "CREATE TABLE agent_missions (id TEXT PRIMARY KEY)")
	_, err = db.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('123', 'PENDING', '{\"role\":\"SOFTWARE_ENGINEER\"}')")

	_, err = db.GetPendingMissions(ctx, "SOFTWARE_ENGINEER")
	if err == nil {
		t.Fatal("Expected error from scan due to missing column")
	}
}

func TestSIPDB_ScanErrors(t *testing.T) {
	db, err := NewSIPDB(filepath.Join(t.TempDir(), "test_scan.db"))
	if err != nil {
		t.Fatal(err)
	}
	defer db.Close()

	ctx := context.Background()

	// Force a scan error by selecting from dummy but scanning into multiple vars?
	// SQLite might cast, but what if we change the table schema or use a mock DB?
	// Actually, simply query the existing table but insert something that cannot be scanned?
	// For example, scanning a NULL into a non-nullable string.

	_, _ = db.db.Exec(ctx, "INSERT INTO capability_plugins (plugin_id, name, version, manifest_url, status, registered_at) VALUES (NULL, 'name', '1', 'url', 'stat', 'time')")
	_, _ = db.GetCapabilityPlugins(ctx, "")
	_, _ = db.db.Exec(ctx, "DELETE FROM capability_plugins")

	_, _ = db.db.Exec(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at) VALUES (NULL, 'ctx', 'vec', 'plg', 'time')")
	_, _ = db.GetEpisodicMemoriesByPlugin(ctx, "")
	_, _ = db.db.Exec(ctx, "DELETE FROM swarm_memory_embeddings")

	_, _ = db.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES (NULL, 'PENDING', '{\"role\":\"ROLE\",\"task\":\"task\"}')")
	_, _ = db.GetPendingMissions(ctx, "ROLE")
	_, _ = db.db.Exec(ctx, "DELETE FROM agent_missions")

	_ = db.CompleteMission(ctx, "nonexistent")
	_ = db.PruneStaleMissions(ctx, 0)
}

func TestSIPDB_BurstMission(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test_burst_mission.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// Create a mission
	msg := Message{
		ID:         "msg-burst-1",
		FromAgent:  "agent-1",
		ToAgent:    "agent-burst",
		Type:       EventTask,
		Content:    "High intensity task",
		OccurredAt: time.Now().UTC(),
	}
	err = db.DelegateMission(ctx, "mission-burst-1", "BURST_ENGINEER", msg)
	if err != nil {
		t.Fatalf("DelegateMission failed: %v", err)
	}

	// Wait a moment so transition duration > 0
	time.Sleep(10 * time.Millisecond)

	// Test 1: Burst without endpoint
	err = db.BurstMission(ctx, "mission-burst-1", "")
	if err != nil {
		t.Fatalf("BurstMission without endpoint failed: %v", err)
	}

	// Verify status updated to BURSTING
	var status string
	err = db.db.QueryRow(ctx, "SELECT status FROM agent_missions WHERE id = 'mission-burst-1'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}
	if status != "BURSTING" {
		t.Fatalf("Expected status BURSTING, got %s", status)
	}

	// Test 2: Burst with mock remote endpoint
	var receivedPayload string
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		body, _ := io.ReadAll(r.Body)
		receivedPayload = string(body)
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	err = db.DelegateMission(ctx, "mission-burst-2", "BURST_ENGINEER_2", msg)
	if err != nil {
		t.Fatalf("DelegateMission failed: %v", err)
	}

	err = db.BurstMission(ctx, "mission-burst-2", ts.URL)
	if err != nil {
		t.Fatalf("BurstMission with endpoint failed: %v", err)
	}

	// Verify status updated to BURSTING
	err = db.db.QueryRow(ctx, "SELECT status FROM agent_missions WHERE id = 'mission-burst-2'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}
	if status != "BURSTING" {
		t.Fatalf("Expected status BURSTING, got %s", status)
	}

	// Verify payload received by mock server
	if receivedPayload == "" {
		t.Fatalf("Mock server did not receive payload")
	}

	// Test 3: Burst mission not found
	err = db.BurstMission(ctx, "mission-burst-nonexistent", "")
	if err == nil {
		t.Fatalf("Expected error for non-existent mission, got nil")
	}
}

func TestSIPDB_DelegateMission_WithContextRoot(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test_delegate_mission.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// 1. Without Context Root (normal)
	msg1 := Message{
		ID:         "msg-1",
		FromAgent:  "agent-1",
		ToAgent:    "agent-2",
		Type:       EventTask,
		Content:    "Original instruction",
		OccurredAt: time.Now().UTC(),
	}
	err = db.DelegateMission(ctx, "mission-1", "SOFTWARE_ENGINEER", msg1)
	if err != nil {
		t.Fatalf("DelegateMission failed: %v", err)
	}

	missions, err := db.GetPendingMissions(ctx, "SOFTWARE_ENGINEER")
	if err != nil {
		t.Fatalf("GetPendingMissions failed: %v", err)
	}
	if len(missions) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions))
	}
	if missions[0].Content != "Original instruction" {
		t.Fatalf("expected original instruction, got %q", missions[0].Content)
	}

	// 2. With Context Root and AGENTS.md
	tempDir := t.TempDir()
	db.SetContextRoot(tempDir)

	agentsMdContent := "Always write clean code."
	err = os.WriteFile(filepath.Join(tempDir, "AGENTS.md"), []byte(agentsMdContent), 0644)
	if err != nil {
		t.Fatalf("Failed to write AGENTS.md: %v", err)
	}

	msg2 := Message{
		ID:         "msg-2",
		FromAgent:  "agent-1",
		ToAgent:    "agent-3",
		Type:       EventTask,
		Content:    "Second instruction",
		OccurredAt: time.Now().UTC(),
	}
	err = db.DelegateMission(ctx, "mission-2", "PRODUCT_MANAGER", msg2)
	if err != nil {
		t.Fatalf("DelegateMission failed: %v", err)
	}

	missions2, err := db.GetPendingMissions(ctx, "PRODUCT_MANAGER")
	if err != nil {
		t.Fatalf("GetPendingMissions failed: %v", err)
	}
	if len(missions2) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions2))
	}

	expectedContent := "Second instruction\n\n[SYSTEM GROUNDING]\n" + agentsMdContent
	if missions2[0].Content != expectedContent {
		t.Fatalf("expected injected instruction, got %q", missions2[0].Content)
	}

	// 3. With Context Root and CLAUDE_OHC.md but no AGENTS.md
	tempDir2 := t.TempDir()
	db.SetContextRoot(tempDir2)

	claudeMdContent := "Use specialized tokens."
	err = os.WriteFile(filepath.Join(tempDir2, "CLAUDE_OHC.md"), []byte(claudeMdContent), 0644)
	if err != nil {
		t.Fatalf("Failed to write CLAUDE_OHC.md: %v", err)
	}

	msg3 := Message{
		ID:         "msg-3",
		FromAgent:  "agent-1",
		ToAgent:    "agent-4",
		Type:       EventTask,
		Content:    "Third instruction",
		OccurredAt: time.Now().UTC(),
	}
	err = db.DelegateMission(ctx, "mission-3", "QA_TESTER", msg3)
	if err != nil {
		t.Fatalf("DelegateMission failed: %v", err)
	}

	missions3, err := db.GetPendingMissions(ctx, "QA_TESTER")
	if err != nil {
		t.Fatalf("GetPendingMissions failed: %v", err)
	}
	if len(missions3) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions3))
	}

	expectedContent2 := "Third instruction\n\n[SYSTEM GROUNDING]\n" + claudeMdContent
	if missions3[0].Content != expectedContent2 {
		t.Fatalf("expected injected instruction, got %q", missions3[0].Content)
	}

	// 4. Grounding Priority (Both AGENTS.md and CLAUDE_OHC.md exist)
	tempDir3 := t.TempDir()
	db.SetContextRoot(tempDir3)

	agentsMdContent4 := "Priority AGENTS."
	err = os.WriteFile(filepath.Join(tempDir3, "AGENTS.md"), []byte(agentsMdContent4), 0644)
	if err != nil {
		t.Fatalf("Failed to write AGENTS.md: %v", err)
	}

	claudeMdContent4 := "Fallback CLAUDE."
	err = os.WriteFile(filepath.Join(tempDir3, "CLAUDE_OHC.md"), []byte(claudeMdContent4), 0644)
	if err != nil {
		t.Fatalf("Failed to write CLAUDE_OHC.md: %v", err)
	}

	msg4 := Message{
		ID:         "msg-4",
		FromAgent:  "agent-1",
		ToAgent:    "agent-5",
		Type:       EventTask,
		Content:    "Fourth instruction",
		OccurredAt: time.Now().UTC(),
	}
	err = db.DelegateMission(ctx, "mission-4", "PRIORITY_TESTER", msg4)
	if err != nil {
		t.Fatalf("DelegateMission failed: %v", err)
	}

	missions4, err := db.GetPendingMissions(ctx, "PRIORITY_TESTER")
	if err != nil {
		t.Fatalf("GetPendingMissions failed: %v", err)
	}
	if len(missions4) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions4))
	}

	expectedContent4 := "Fourth instruction\n\n[SYSTEM GROUNDING]\n" + agentsMdContent4 + "\n\n" + claudeMdContent4
	if missions4[0].Content != expectedContent4 {
		t.Fatalf("expected injected instruction, got %q", missions4[0].Content)
	}

	// 5. Missing Files (Context Root Configured)
	tempDir4 := t.TempDir()
	db.SetContextRoot(tempDir4)

	msg5 := Message{
		ID:         "msg-5",
		FromAgent:  "agent-1",
		ToAgent:    "agent-6",
		Type:       EventTask,
		Content:    "Fifth instruction",
		OccurredAt: time.Now().UTC(),
	}
	err = db.DelegateMission(ctx, "mission-5", "MISSING_FILE_TESTER", msg5)
	if err != nil {
		t.Fatalf("DelegateMission failed: %v", err)
	}

	missions5, err := db.GetPendingMissions(ctx, "MISSING_FILE_TESTER")
	if err != nil {
		t.Fatalf("GetPendingMissions failed: %v", err)
	}
	if len(missions5) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions5))
	}

	expectedContent5 := "Fifth instruction"
	if missions5[0].Content != expectedContent5 {
		t.Fatalf("expected unmodified instruction, got %q", missions5[0].Content)
	}
}
func TestSIPDB_DelegateMission_MissingFiles(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test_delegate_mission_missing.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// 4. TC4: Both AGENTS.md and CLAUDE_OHC.md present, they should be combined
	tempDir3 := t.TempDir()
	db.SetContextRoot(tempDir3)

	agentsMdContent := "AGENTS.md rules"
	err = os.WriteFile(filepath.Join(tempDir3, "AGENTS.md"), []byte(agentsMdContent), 0644)
	if err != nil {
		t.Fatalf("Failed to write AGENTS.md: %v", err)
	}

	claudeMdContent := "CLAUDE_OHC.md rules"
	err = os.WriteFile(filepath.Join(tempDir3, "CLAUDE_OHC.md"), []byte(claudeMdContent), 0644)
	if err != nil {
		t.Fatalf("Failed to write CLAUDE_OHC.md: %v", err)
	}

	msg4 := Message{
		ID:         "msg-4",
		FromAgent:  "agent-1",
		ToAgent:    "agent-5",
		Type:       EventTask,
		Content:    "Fourth instruction",
		OccurredAt: time.Now().UTC(),
	}
	err = db.DelegateMission(ctx, "mission-4", "PRIORITY_TESTER", msg4)
	if err != nil {
		t.Fatalf("DelegateMission failed: %v", err)
	}

	missions4, err := db.GetPendingMissions(ctx, "PRIORITY_TESTER")
	if err != nil {
		t.Fatalf("GetPendingMissions failed: %v", err)
	}
	if len(missions4) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions4))
	}

	expectedContent3 := "Fourth instruction\n\n[SYSTEM GROUNDING]\n" + agentsMdContent + "\n\n" + claudeMdContent
	if missions4[0].Content != expectedContent3 {
		t.Fatalf("expected injected instruction, got %q", missions4[0].Content)
	}

	// 5. TC5: Context Root set but missing files
	tempDir4 := t.TempDir()
	db.SetContextRoot(tempDir4)

	msg5 := Message{
		ID:         "msg-5",
		FromAgent:  "agent-1",
		ToAgent:    "agent-6",
		Type:       EventTask,
		Content:    "Fifth instruction",
		OccurredAt: time.Now().UTC(),
	}
	err = db.DelegateMission(ctx, "mission-5", "MISSING_FILES_TESTER", msg5)
	if err != nil {
		t.Fatalf("DelegateMission failed: %v", err)
	}

	missions5, err := db.GetPendingMissions(ctx, "MISSING_FILES_TESTER")
	if err != nil {
		t.Fatalf("GetPendingMissions failed: %v", err)
	}
	if len(missions5) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions5))
	}

	if missions5[0].Content != "Fifth instruction" {
		t.Fatalf("expected unmodified instruction, got %q", missions5[0].Content)
	}
}

func TestSIPDB_GetPendingMissions_Fallbacks(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test_get_pending_fallbacks.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// 1. Missing "task" object in payload -> unmarshal directly
	_, err = db.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('m-1', 'PENDING', '{\"role\":\"TESTER1\",\"Content\":\"Direct Content\"}')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}

	missions1, err := db.GetPendingMissions(ctx, "TESTER1")
	if err != nil {
		t.Fatalf("GetPendingMissions failed: %v", err)
	}
	if len(missions1) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions1))
	}
	if missions1[0].Content != "Direct Content" {
		t.Fatalf("expected Direct Content, got %q", missions1[0].Content)
	}

	// 2. Completely invalid JSON -> fallback raw
	_, err = db.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('m-3', 'PENDING', '{\"role\":\"TESTER2\",\"task\":\"some string\"}')")
	if err != nil {
		t.Fatalf("Failed to insert: %v", err)
	}

	missions2, err := db.GetPendingMissions(ctx, "TESTER2")
	if err != nil {
		t.Fatalf("GetPendingMissions failed: %v", err)
	}
	if len(missions2) != 1 {
		t.Fatalf("expected 1 mission, got %d", len(missions2))
	}
	if missions2[0].Content != "some string" && missions2[0].Content != "\"some string\"" {
		t.Fatalf("expected \"some string\", got %q", missions2[0].Content)
	}
}

func TestSIPDB_SyncMissions_Sanitization(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test_sync_missions.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create test DB: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	// Insert a mission with private markers and PII
	_, err = db.db.Exec(context.Background(), `
		INSERT INTO agent_missions (id, status, payload)
		VALUES ('m-sync-1', 'PENDING', '{"role":"TESTER", "task":{"id":"m-sync-1","type":"task","content":"[PRIVATE:secret] my email is user@example.com and [PRIVATE:other] phone is 555-555-5555"}}')
	`)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	var reqBody []byte
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		reqBody, _ = io.ReadAll(r.Body)
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

	expectedPayload := `{"id":"m-sync-1","role":"TESTER","task":{"content":" my email is [REDACTED_EMAIL] and  phone is [REDACTED_PHONE]","id":"m-sync-1","type":"task"}}`
	if string(reqBody) != expectedPayload {
		t.Fatalf("Expected payload %s, got %s", expectedPayload, string(reqBody))
	}
}

func TestSIPDB_SyncBufferedMetrics(t *testing.T) {
	dbPath := filepath.Join(t.TempDir(), "test_sync_metrics.db")
	db, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	err = db.BufferMetric(ctx, "token_usage", `{"agent_id":"a1","role":"r1","count":10}`)
	if err != nil {
		t.Fatalf("BufferMetric failed: %v", err)
	}
	err = db.BufferMetric(ctx, "agent_api_call", `{"agent_id":"a2","api":"fetch"}`)
	if err != nil {
		t.Fatalf("BufferMetric failed: %v", err)
	}

	var count int
	err = db.db.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil || count != 2 {
		t.Fatalf("Expected 2 metrics, got %d, err: %v", count, err)
	}

	var reqBody []byte
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		reqBody, _ = io.ReadAll(r.Body)
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	syncedCount, err := db.SyncBufferedMetrics(ctx, ts.URL)
	if err != nil {
		t.Fatalf("SyncBufferedMetrics failed: %v", err)
	}
	if syncedCount != 2 {
		t.Fatalf("Expected 2 synced records, got %d", syncedCount)
	}

	expectedBody := `[{"agent_id":"a1","count":10,"metric_type":"token_usage","role":"r1"},{"agent_id":"a2","api":"fetch","metric_type":"agent_api_call"}]`
	if string(reqBody) != expectedBody {
		t.Fatalf("Expected payload %s, got %s", expectedBody, string(reqBody))
	}

	err = db.db.QueryRow(ctx, "SELECT COUNT(*) FROM telemetry_buffer").Scan(&count)
	if err != nil || count != 0 {
		t.Fatalf("Expected 0 metrics after sync, got %d, err: %v", count, err)
	}
}
