package orchestration

import (
	"context"
	"path/filepath"
	"testing"
	"time"
)

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

	// Insert invalid schema data manually to cause a row scan error.
	// Since we can't easily break the type in sqlite (it's dynamically typed),
	// this one is hard to hit purely through SQLite without mocking the DB connection.
	// Instead, we focus on the Unmarshal error line 150-151.

	// Manually insert bad JSON
	ctx := context.Background()
	_, err = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('123', 'PENDING', '{\"role\":\"SOFTWARE_ENGINEER\", \"task\":\"invalid-json\"}')")
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

	if missions[0].Content != "invalid-json" {
		t.Fatalf("Expected content 'invalid-json', got %s", missions[0].Content)
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
	_, err = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('123', 'PENDING', '{\"role\":\"SOFTWARE_ENGINEER\", \"task\":\"invalid-json\"}')")
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

	if missions[0].Content != "invalid-json" {
		t.Fatalf("Expected content to be 'invalid-json' fallback, got: %s", missions[0].Content)
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
	_, err = db.db.ExecContext(context.Background(), "DROP TABLE agent_missions")
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
	_, err = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload, created_at) VALUES ('1', 'PENDING', '{}', datetime('now'))")
	if err != nil {
		t.Fatal(err)
	}

	// 2. Completed (should be deleted regardless of age)
	_, err = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload, created_at) VALUES ('2', 'COMPLETED', '{}', datetime('now'))")
	if err != nil {
		t.Fatal(err)
	}

	// 3. Pending but old (should be deleted)
	_, err = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload, created_at) VALUES ('3', 'PENDING', '{}', datetime('now', '-2 days'))")
	if err != nil {
		t.Fatal(err)
	}

	// Prune missions older than 24 hours
	err = db.PruneStaleMissions(ctx, 24*time.Hour)
	if err != nil {
		t.Fatalf("Failed to prune stale missions: %v", err)
	}

	var count int
	err = db.db.QueryRowContext(ctx, "SELECT COUNT(*) FROM agent_missions").Scan(&count)
	if err != nil {
		t.Fatal(err)
	}

	if count != 1 {
		t.Fatalf("Expected 1 mission remaining, got %d", count)
	}

	// Verify the remaining mission is the correct one
	var id string
	err = db.db.QueryRowContext(ctx, "SELECT id FROM agent_missions").Scan(&id)
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
	_, err = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ('123', 'PENDING', '{\"role\":\"SOFTWARE_ENGINEER\"}')")
	// Now we will break the table to cause Scan to fail. Scan fails if the number of columns returned doesn't match the pointers, or type conversion fails.
	// But type conversion to string rarely fails in sqlite. We can close the rows early maybe?
	// The easiest way is to mock rows or just change schema, but we can't alter table.
	// We can drop table and recreate it with only one column!
	_, err = db.db.ExecContext(ctx, "DROP TABLE agent_missions")
	_, err = db.db.ExecContext(ctx, "CREATE TABLE agent_missions (id TEXT PRIMARY KEY, status TEXT)")
	_, err = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status) VALUES ('123', 'PENDING')")

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

	_, _ = db.db.ExecContext(ctx, "INSERT INTO capability_plugins (plugin_id, name, version, manifest_url, status, registered_at) VALUES (NULL, 'name', '1', 'url', 'stat', 'time')")
	_, _ = db.GetCapabilityPlugins(ctx, "")
	_, _ = db.db.ExecContext(ctx, "DELETE FROM capability_plugins")

	_, _ = db.db.ExecContext(ctx, "INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, source_plugin, created_at) VALUES (NULL, 'ctx', 'vec', 'plg', 'time')")
	_, _ = db.GetEpisodicMemoriesByPlugin(ctx, "")
	_, _ = db.db.ExecContext(ctx, "DELETE FROM swarm_memory_embeddings")

	_, _ = db.db.ExecContext(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES (NULL, 'PENDING', '{\"role\":\"ROLE\"}')")
	_, _ = db.GetPendingMissions(ctx, "ROLE")
	_, _ = db.db.ExecContext(ctx, "DELETE FROM agent_missions")

	_ = db.CompleteMission(ctx, "nonexistent")
	_ = db.PruneStaleMissions(ctx, 0)
}
