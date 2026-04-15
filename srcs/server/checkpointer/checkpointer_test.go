package checkpointer

import (
	"context"
	"reflect"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestProvider(t *testing.T) db.Provider {
	provider := db.NewTestProvider(t)

	// Create table
	query := `
	CREATE TABLE IF NOT EXISTS swarm_checkpoints (
		thread_id TEXT NOT NULL,
		checkpoint_id TEXT NOT NULL,
		parent_id TEXT,
		checkpoint JSONB NOT NULL,
		metadata JSONB DEFAULT '{}',
		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
		PRIMARY KEY (thread_id, checkpoint_id)
	);`
	_, err := provider.Exec(context.Background(), query)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return provider
}

func TestPgCheckpointSaver_SaveAndLoad(t *testing.T) {
	provider := setupTestProvider(t)
	saver := NewPgCheckpointSaver(provider)
	ctx := context.Background()

	parentID := "parent-1"
	cp := &Checkpoint{
		ThreadID:     "thread-1",
		CheckpointID: "cp-1",
		ParentID:     &parentID,
		Data: map[string]interface{}{
			"step": float64(1),
			"data": "some value",
		},
		Metadata: map[string]interface{}{
			"agent": "SWE-1",
		},
		CreatedAt: time.Now().UTC().Truncate(time.Second),
	}

	// Test PutCheckpoint
	err := saver.PutCheckpoint(ctx, cp)
	if err != nil {
		t.Fatalf("PutCheckpoint failed: %v", err)
	}

	// Test GetCheckpoint
	retrieved, err := saver.GetCheckpoint(ctx, cp.ThreadID, cp.CheckpointID)
	if err != nil {
		t.Fatalf("GetCheckpoint failed: %v", err)
	}

	if retrieved.ThreadID != cp.ThreadID {
		t.Errorf("expected thread ID %q, got %q", cp.ThreadID, retrieved.ThreadID)
	}
	if retrieved.CheckpointID != cp.CheckpointID {
		t.Errorf("expected checkpoint ID %q, got %q", cp.CheckpointID, retrieved.CheckpointID)
	}
	if *retrieved.ParentID != *cp.ParentID {
		t.Errorf("expected parent ID %q, got %q", *cp.ParentID, *retrieved.ParentID)
	}
	if !reflect.DeepEqual(retrieved.Data, cp.Data) {
		t.Errorf("expected data %v, got %v", cp.Data, retrieved.Data)
	}
	if !reflect.DeepEqual(retrieved.Metadata, cp.Metadata) {
		t.Errorf("expected metadata %v, got %v", cp.Metadata, retrieved.Metadata)
	}
	if !retrieved.CreatedAt.Equal(cp.CreatedAt) {
		t.Errorf("expected created_at %v, got %v", cp.CreatedAt, retrieved.CreatedAt)
	}
}

func TestPgCheckpointSaver_ListCheckpoints(t *testing.T) {
	provider := setupTestProvider(t)
	saver := NewPgCheckpointSaver(provider)
	ctx := context.Background()

	threadID := "thread-list"
	for i := 1; i <= 3; i++ {
		cp := &Checkpoint{
			ThreadID:     threadID,
			CheckpointID: string(rune('0' + i)),
			Data:         map[string]interface{}{"i": float64(i)},
			Metadata:     map[string]interface{}{},
			CreatedAt:    time.Now().Add(time.Duration(i) * time.Hour).UTC().Truncate(time.Second),
		}
		if err := saver.PutCheckpoint(ctx, cp); err != nil {
			t.Fatalf("PutCheckpoint %d failed: %v", i, err)
		}
	}

	checkpoints, err := saver.ListCheckpoints(ctx, threadID)
	if err != nil {
		t.Fatalf("ListCheckpoints failed: %v", err)
	}

	if len(checkpoints) != 3 {
		t.Errorf("expected 3 checkpoints, got %d", len(checkpoints))
	}

	// Should be ordered by created_at DESC
	if checkpoints[0].CheckpointID != "3" {
		t.Errorf("expected first checkpoint to be '3', got %q", checkpoints[0].CheckpointID)
	}
}
