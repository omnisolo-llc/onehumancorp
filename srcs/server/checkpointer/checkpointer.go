package checkpointer

import (
	"context"
	"crypto/rand"
	"encoding/hex"
)

// Checkpoint represents a single state snapshot for a given LangGraph thread.
type Checkpoint struct {
	ThreadID     string                 `json:"thread_id"`
	CheckpointID string                 `json:"checkpoint_id"`
	ParentID     string                 `json:"parent_id"`
	Checkpoint   map[string]interface{} `json:"checkpoint"`
	Metadata     map[string]interface{} `json:"metadata"`
}

// CheckpointSaver interface defines the required methods for saving and loading agent states.
type CheckpointSaver interface {
	GetCheckpoint(ctx context.Context, threadID string) (*Checkpoint, error)
	PutCheckpoint(ctx context.Context, threadID string, checkpoint *Checkpoint) error
	ListCheckpoints(ctx context.Context, threadID string) ([]Checkpoint, error)
}

func GenerateCheckpointID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b[0:4]) + "-" + hex.EncodeToString(b[4:6]) + "-" + hex.EncodeToString(b[6:8]) + "-" + hex.EncodeToString(b[8:10]) + "-" + hex.EncodeToString(b[10:])
}
