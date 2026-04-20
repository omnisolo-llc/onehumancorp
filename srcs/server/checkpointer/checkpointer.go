package checkpointer

import (
	"context"
	"time"
)

// Checkpoint represents a single state snapshot for a given LangGraph thread.
type Checkpoint struct {
	ThreadID     string                 `json:"thread_id"`
	CheckpointID string                 `json:"checkpoint_id"`
	ParentID     *string                `json:"parent_id"`
	Data         map[string]interface{} `json:"checkpoint"`
	Metadata     map[string]interface{} `json:"metadata"`
	CreatedAt    time.Time              `json:"created_at"`
}

// CheckpointSaver interface defines the required methods for saving and loading agent states.
type CheckpointSaver interface {
	GetCheckpoint(ctx context.Context, threadID string, checkpointID string) (*Checkpoint, error)
	PutCheckpoint(ctx context.Context, checkpoint *Checkpoint) error
	ListCheckpoints(ctx context.Context, threadID string) ([]Checkpoint, error)
}
