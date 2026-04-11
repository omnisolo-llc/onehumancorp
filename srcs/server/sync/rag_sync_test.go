package sync

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"database/sql"
	_ "modernc.org/sqlite"
)

func TestRAGSyncService_FetchPendingSyncs(t *testing.T) {
	// Initialize an in-memory SQLite DB
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	dbPool := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}


	defer dbPool.Close()

	ctx := context.Background()

	// Apply necessary schema manually for the test
	schema := `
	CREATE TABLE IF NOT EXISTS agent_memories (
		id TEXT PRIMARY KEY,
		organization_id VARCHAR NOT NULL,
		content TEXT NOT NULL,
		embedding TEXT,
		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);`
	if _, err := dbPool.Exec(ctx, schema); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	// Insert test data
	vector1 := []float32{1.1, 2.2, 3.3}
	vBytes1, _ := json.Marshal(vector1)

	insertQuery := `INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status) VALUES ($1, $2, $3, $4, $5)`
	if _, err := dbPool.Exec(ctx, insertQuery, "1", "org1", "context 1", string(vBytes1), string(SyncStatusPending)); err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}
	if _, err := dbPool.Exec(ctx, insertQuery, "2", "org1", "context 2", nil, string(SyncStatusPending)); err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	service := NewRAGSyncService(dbPool)

	// Test FetchPendingSyncs
	pending, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(pending) != 2 {
		t.Fatalf("expected 2 pending syncs, got %d", len(pending))
	}
	if pending[0].ID != "1" {
		t.Errorf("expected ID 1, got %s", pending[0].ID)
	}
	if len(pending[0].Vector) != 3 {
		t.Errorf("expected vector length 3, got %d", len(pending[0].Vector))
	}
}

func TestRAGSyncService_MarkSynced(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	dbPool := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}


	defer dbPool.Close()

	ctx := context.Background()
	schema := `
	CREATE TABLE IF NOT EXISTS agent_memories (
		id TEXT PRIMARY KEY,
		organization_id VARCHAR NOT NULL,
		content TEXT NOT NULL,
		embedding TEXT,
		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);`
	if _, err := dbPool.Exec(ctx, schema); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	insertQuery := `INSERT INTO agent_memories (id, organization_id, content, sync_status) VALUES ($1, $2, $3, $4)`
	if _, err := dbPool.Exec(ctx, insertQuery, "1", "org1", "context 1", string(SyncStatusPending)); err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	service := NewRAGSyncService(dbPool)

	// Test MarkSynced
	err = service.MarkSynced(ctx, []string{"1"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify update
	var status string
	err = dbPool.QueryRow(ctx, `SELECT sync_status FROM agent_memories WHERE id = $1`, "1").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}
	if status != string(SyncStatusSynced) {
		t.Errorf("expected status %s, got %s", SyncStatusSynced, status)
	}
}

func TestRAGSyncService_ProcessIncomingSync(t *testing.T) {
	sqliteDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	dbPool := &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}


	defer dbPool.Close()

	ctx := context.Background()
	schema := `
	CREATE TABLE IF NOT EXISTS agent_memories (
		id TEXT PRIMARY KEY,
		organization_id VARCHAR NOT NULL,
		content TEXT NOT NULL,
		embedding TEXT,
		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
		sync_status VARCHAR(50) DEFAULT 'pending',
		last_sync_at TIMESTAMP NULL
	);`
	if _, err := dbPool.Exec(ctx, schema); err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

	service := NewRAGSyncService(dbPool)

	// Test ProcessIncomingSync
	records := []RAGSyncRecord{
		{
			ID:         "1",
			Context:    "Incoming context",
			Vector:     []float32{1.0, 2.0},
			SyncStatus: SyncStatusSynced,
			LastSyncAt: time.Now(),
		},
	}

	err = service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify insert
	var content string
	var status string
	err = dbPool.QueryRow(ctx, `SELECT content, sync_status FROM agent_memories WHERE id = $1`, "1").Scan(&content, &status)
	if err != nil {
		t.Fatalf("failed to query records: %v", err)
	}
	if content != "Incoming context" {
		t.Errorf("expected content 'Incoming context', got '%s'", content)
	}
	if status != string(SyncStatusSynced) {
		t.Errorf("expected status %s, got %s", SyncStatusSynced, status)
	}
}

func TestTelemetryInitialization(t *testing.T) {
	if RagRecordsSyncedTotal == nil {
		t.Error("expected RagRecordsSyncedTotal to be initialized")
	}
	if RagSyncErrorsTotal == nil {
		t.Error("expected RagSyncErrorsTotal to be initialized")
	}
}
