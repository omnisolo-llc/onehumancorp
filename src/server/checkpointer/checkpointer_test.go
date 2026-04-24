package checkpointer

import (
	"context"
	"reflect"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
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

func TestCompressDecompress(t *testing.T) {
	original := []byte(`{"hello":"world","deep":{"nested":[1,2,3]}}`)

	compressed, err := compressData(original)
	if err != nil {
		t.Fatalf("Failed to compress data: %v", err)
	}

	// Verify compressed data starts and ends with quotes to be valid JSON
	if len(compressed) < 2 || compressed[0] != '"' || compressed[len(compressed)-1] != '"' {
		t.Errorf("Compressed data is not properly quoted: %s", string(compressed))
	}

	decompressed, err := decompressData(compressed)
	if err != nil {
		t.Fatalf("Failed to decompress data: %v", err)
	}

	if string(decompressed) != string(original) {
		t.Errorf("Expected decompressed data to match original. Got %s, want %s", string(decompressed), string(original))
	}
}

func TestDecompressBackwardCompatibility(t *testing.T) {
	// Old data (uncompressed JSON)
	oldData := []byte(`{"hello":"world"}`)

	decompressed, err := decompressData(oldData)
	if err != nil {
		t.Fatalf("Decompression of backward compatible data failed: %v", err)
	}

	if string(decompressed) != string(oldData) {
		t.Errorf("Expected data to be returned as-is. Got %s, want %s", string(decompressed), string(oldData))
	}

	// Old data wrapped in quotes (e.g. if somehow just stringified) - this shouldn't be valid base64/gzip, so fallback to raw data
	oldQuotedData := []byte(`"not-base64-or-gzip"`)
	decompressed2, err := decompressData(oldQuotedData)
	if err != nil {
		t.Fatalf("Decompression of invalid quoted data failed: %v", err)
	}

	if string(decompressed2) != string(oldQuotedData) {
		t.Errorf("Expected data to be returned as-is on fail. Got %s, want %s", string(decompressed2), string(oldQuotedData))
	}
}
