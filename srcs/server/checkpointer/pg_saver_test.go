package checkpointer

import (
	"context"
	"database/sql"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestPGSaver(t *testing.T) {
	os.Setenv("DATABASE_URL", "sqlite://:memory:")
	ctx := context.Background()
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to create db provider: %v", err)
	}

	// Create table manually for tests as we are not running migrations
	_, err = provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_checkpoints (
			thread_id TEXT NOT NULL,
			checkpoint_id TEXT NOT NULL,
			parent_id TEXT,
			checkpoint JSONB NOT NULL,
			metadata JSONB DEFAULT '{}',
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			PRIMARY KEY (thread_id, checkpoint_id)
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	saver := NewPGSaver(provider)

	threadID := "thread-1"
	parentID := "parent-0"
	chkpt := &CheckpointState{
		ThreadID: threadID,
		CheckpointID: "chk-1",
		ParentID: &parentID,
		Checkpoint: map[string]interface{}{
			"foo": "bar",
			"baz": map[string]interface{}{
				"qux": 123.0, // JSON numbers parse as float64
			},
		},
		Metadata: map[string]interface{}{
			"meta_key": "meta_val",
		},
	}

	err = saver.PutCheckpoint(ctx, threadID, chkpt)
	if err != nil {
		t.Fatalf("PutCheckpoint failed: %v", err)
	}

	loaded, err := saver.GetCheckpoint(ctx, threadID)
	if err != nil {
		t.Fatalf("GetCheckpoint failed: %v", err)
	}

	if loaded.ThreadID != threadID {
		t.Errorf("expected thread_id %q, got %q", threadID, loaded.ThreadID)
	}
	if loaded.CheckpointID != chkpt.CheckpointID {
		t.Errorf("expected checkpoint_id %q, got %q", chkpt.CheckpointID, loaded.CheckpointID)
	}
	if loaded.ParentID == nil || *loaded.ParentID != parentID {
		t.Errorf("expected parent_id %q, got %v", parentID, loaded.ParentID)
	}

	if loaded.Checkpoint["foo"] != "bar" {
		t.Errorf("expected checkpoint foo=bar, got %v", loaded.Checkpoint["foo"])
	}
	baz := loaded.Checkpoint["baz"].(map[string]interface{})
	if baz["qux"] != 123.0 {
		t.Errorf("expected checkpoint baz.qux=123.0, got %v", baz["qux"])
	}
	if loaded.Metadata["meta_key"] != "meta_val" {
		t.Errorf("expected metadata meta_key=meta_val, got %v", loaded.Metadata["meta_key"])
	}

	list, err := saver.ListCheckpoints(ctx, threadID)
	if err != nil {
		t.Fatalf("ListCheckpoints failed: %v", err)
	}
	if len(list) != 1 {
		t.Errorf("expected 1 checkpoint in list, got %d", len(list))
	}

	// Test error on not found
	_, err = saver.GetCheckpoint(ctx, "non-existent")
	if err != sql.ErrNoRows {
		t.Errorf("expected sql.ErrNoRows, got %v", err)
	}
}
