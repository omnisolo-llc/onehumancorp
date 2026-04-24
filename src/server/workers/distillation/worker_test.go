package distillation

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/checkpointer"
	"github.com/stretchr/testify/assert"
)

type mockCheckpointer struct {
	checkpoints []checkpointer.Checkpoint
}

func (m *mockCheckpointer) GetCheckpoint(ctx context.Context, threadID string, checkpointID string) (*checkpointer.Checkpoint, error) {
	for _, cp := range m.checkpoints {
		if cp.ThreadID == threadID && cp.CheckpointID == checkpointID {
			return &cp, nil
		}
	}
	return nil, nil
}

func (m *mockCheckpointer) PutCheckpoint(ctx context.Context, checkpoint *checkpointer.Checkpoint) error {
	m.checkpoints = append(m.checkpoints, *checkpoint)
	return nil
}

func (m *mockCheckpointer) ListCheckpoints(ctx context.Context, threadID string) ([]checkpointer.Checkpoint, error) {
	var result []checkpointer.Checkpoint
	for _, cp := range m.checkpoints {
		if cp.ThreadID == threadID {
			result = append(result, cp)
		}
	}
	return result, nil
}

func TestSemanticDistillationWorker_DistillThread(t *testing.T) {
	cp := &mockCheckpointer{}

	// Add test checkpoints
	cp.PutCheckpoint(context.Background(), &checkpointer.Checkpoint{
		ThreadID:     "thread-1",
		CheckpointID: "cp-1",
		Data:         map[string]interface{}{"key": "value"},
		CreatedAt:    time.Now(),
	})

	worker := NewSemanticDistillationWorker(nil, cp)

	err := worker.DistillThread(context.Background(), "thread-1")
	assert.NoError(t, err)
}
