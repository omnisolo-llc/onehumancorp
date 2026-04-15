package checkpointer

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestPgCheckpointSaver(t *testing.T) {
	// Setup in-memory SQLite for testing
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	// Create table
	_, err = sqlDB.Exec(`
		CREATE TABLE swarm_checkpoints (
			thread_id TEXT NOT NULL,
			checkpoint_id TEXT NOT NULL,
			parent_id TEXT,
			checkpoint TEXT NOT NULL,
			metadata TEXT DEFAULT '{}',
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			PRIMARY KEY (thread_id, checkpoint_id)
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	saver := NewPgCheckpointSaver(provider)
	ctx := context.Background()

	cp := &Checkpoint{
		ThreadID:     "thread-1",
		CheckpointID: "cp-1",
		ParentID:     "",
		Checkpoint:   map[string]interface{}{"step": 1, "data": "hello"},
		Metadata:     map[string]interface{}{"agent": "jules"},
	}

	// Test PutCheckpoint
	err = saver.PutCheckpoint(ctx, cp.ThreadID, cp)
	if err != nil {
		t.Errorf("PutCheckpoint failed: %v", err)
	}

	// Test GetCheckpoint
	retrieved, err := saver.GetCheckpoint(ctx, cp.ThreadID)
	if err != nil {
		t.Errorf("GetCheckpoint failed: %v", err)
	}
	if retrieved.CheckpointID != cp.CheckpointID {
		t.Errorf("expected checkpoint_id %s, got %s", cp.CheckpointID, retrieved.CheckpointID)
	}
	if retrieved.Checkpoint["step"].(float64) != 1 {
		t.Errorf("expected step 1, got %v", retrieved.Checkpoint["step"])
	}

	// Test ListCheckpoints
	time.Sleep(1100 * time.Millisecond) // Ensure different timestamp in SQLite
	cp2 := &Checkpoint{
		ThreadID:     "thread-1",
		CheckpointID: "cp-2",
		ParentID:     "cp-1",
		Checkpoint:   map[string]interface{}{"step": 2, "data": "world"},
		Metadata:     map[string]interface{}{"agent": "jules"},
	}
	err = saver.PutCheckpoint(ctx, cp2.ThreadID, cp2)
	if err != nil {
		t.Errorf("PutCheckpoint 2 failed: %v", err)
	}

	list, err := saver.ListCheckpoints(ctx, "thread-1")
	if err != nil {
		t.Errorf("ListCheckpoints failed: %v", err)
	}
	if len(list) != 2 {
		t.Errorf("expected 2 checkpoints, got %d", len(list))
	}
	if list[0].CheckpointID != "cp-2" {
		t.Errorf("expected latest checkpoint_id cp-2, got %s", list[0].CheckpointID)
	}
}
