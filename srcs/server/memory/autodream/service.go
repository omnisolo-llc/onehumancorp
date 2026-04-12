package autodream

import (
	"context"
	"fmt"
	"log/slog"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/memory"
)

type MemoryConsolidator interface {
	Consolidate(ctx context.Context, taskID, tenantID, taskLogs string) error
}

type MinimaxClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamService struct {
	repo   memory.VectorRepository
	client MinimaxClient
}

func NewAutoDreamService(repo memory.VectorRepository, client MinimaxClient) *AutoDreamService {
	return &AutoDreamService{
		repo:   repo,
		client: client,
	}
}

func (s *AutoDreamService) Consolidate(ctx context.Context, taskID, tenantID, taskLogs string) error {
	slog.Info("AutoDream: Consolidating memory for task", "taskID", taskID)

	// Mock summarization for now. In real implementation, this would call LLM.
	summary := fmt.Sprintf("Summarized insights from task %s: %s", taskID, taskLogs)

	embedding := make([]float32, 1536)
	if s.client != nil {
		resp, err := s.client.GenerateEmbedding(ctx, summary)
		if err == nil && len(resp) == 1536 {
			embedding = resp
		} else {
			slog.Warn("AutoDream: failed to embed, using default empty embedding", "error", err)
		}
	}

	mem := memory.Memory{
		ID:           uuid.New().String(),
		TenantID:     tenantID,
		MemoryType:   "TASK_SUMMARY",
		Content:      summary,
		Embedding:    embedding,
		SourceTaskID: taskID,
	}

	err := s.repo.Upsert(ctx, mem)
	if err != nil {
		return fmt.Errorf("failed to upsert memory embedding: %w", err)
	}

	return nil
}
