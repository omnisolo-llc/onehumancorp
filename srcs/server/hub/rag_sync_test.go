package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	_ "modernc.org/sqlite"
	"go.opentelemetry.io/otel/metric/noop"
)

func setupTestDB(t *testing.T) *db.DB {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDB)
	dbWrapper := &db.DB{Provider: provider}

	_, err = provider.Exec(context.Background(), `
		DROP TABLE IF EXISTS autodream_memories;
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			sync_status VARCHAR(50) DEFAULT 'pending',
			last_sync_timestamp TIMESTAMP NULL
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return dbWrapper
}

func TestFetchPendingSyncs(t *testing.T) {
	telemetry.InitWithMeter(noop.NewMeterProvider().Meter("test"))

	dbWrapper := setupTestDB(t)
	svc := NewSqliteRAGSyncService(dbWrapper)
	ctx := context.Background()

	vec := []float32{0.1, 0.2, 0.3}
	vecBytes, _ := json.Marshal(vec)

	_, err := dbWrapper.Provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, embedding, sync_status)
		VALUES ('test-1', 'content 1', $1, 'pending'),
		       ('test-2', 'content 2', NULL, 'synced')
	`, string(vecBytes))
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	records, err := svc.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("FetchPendingSyncs failed: %v", err)
	}

	if len(records) != 1 {
		t.Fatalf("Expected 1 record, got %d", len(records))
	}

	r := records[0]
	if r.ID != "test-1" {
		t.Errorf("Expected ID 'test-1', got %s", r.ID)
	}
	if len(r.Vector) != 3 || r.Vector[0] != 0.1 {
		t.Errorf("Expected vector [0.1, 0.2, 0.3], got %v", r.Vector)
	}
	if r.SyncStatus != SyncStatusPending {
		t.Errorf("Expected status 'pending', got %s", r.SyncStatus)
	}
}

func TestMarkSynced(t *testing.T) {
	telemetry.InitWithMeter(noop.NewMeterProvider().Meter("test"))

	dbWrapper := setupTestDB(t)
	svc := NewSqliteRAGSyncService(dbWrapper)
	ctx := context.Background()

	_, err := dbWrapper.Provider.Exec(ctx, `
		INSERT INTO autodream_memories (id, content, sync_status)
		VALUES ('test-1', 'content 1', 'pending'),
		       ('test-2', 'content 2', 'pending')
	`)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	err = svc.MarkSynced(ctx, []string{"test-1", "test-2"})
	if err != nil {
		t.Fatalf("MarkSynced failed: %v", err)
	}

	// Verify
	rows, err := dbWrapper.Provider.Query(ctx, "SELECT sync_status FROM autodream_memories WHERE id IN ('test-1', 'test-2')")
	if err != nil {
		t.Fatalf("Failed to query verification data: %v", err)
	}
	defer rows.Close()

	count := 0
	for rows.Next() {
		var status string
		if err := rows.Scan(&status); err != nil {
			t.Fatalf("Failed to scan: %v", err)
		}
		if status != "synced" {
			t.Errorf("Expected status 'synced', got %s", status)
		}
		count++
	}
	if count != 2 {
		t.Errorf("Expected 2 synced rows, found %d", count)
	}
}

func TestProcessIncomingSync(t *testing.T) {
	telemetry.InitWithMeter(noop.NewMeterProvider().Meter("test"))

	dbWrapper := setupTestDB(t)
	svc := NewSqliteRAGSyncService(dbWrapper)
	ctx := context.Background()

	record := RAGSyncRecord{
		ID:         "cloud-1",
		Content:    "cloud content",
		Vector:     []float32{0.5, 0.5},
		SyncStatus: SyncStatusSynced,
		LastSyncAt: time.Now(),
	}

	err := svc.ProcessIncomingSync(ctx, []RAGSyncRecord{record})
	if err != nil {
		t.Fatalf("ProcessIncomingSync failed: %v", err)
	}

	// Verify
	rows, err := dbWrapper.Provider.Query(ctx, "SELECT id, content, sync_status FROM autodream_memories WHERE id = 'cloud-1'")
	if err != nil {
		t.Fatalf("Failed to query verification data: %v", err)
	}
	defer rows.Close()

	if !rows.Next() {
		t.Fatal("Expected record not found")
	}

	var id, content, status string
	if err := rows.Scan(&id, &content, &status); err != nil {
		t.Fatalf("Failed to scan: %v", err)
	}

	if id != "cloud-1" || content != "cloud content" || status != "synced" {
		t.Errorf("Mismatch in expected data: got id=%s, content=%s, status=%s", id, content, status)
	}
}
