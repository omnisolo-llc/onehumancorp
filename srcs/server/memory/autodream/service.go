package autodream

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/memory"
)

type LLMClient interface {
	Reason(ctx context.Context, prompt string) (string, error)
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type Service struct {
	vectorRepo *memory.VectorRepository
	llm        LLMClient
}

func NewService(vectorRepo *memory.VectorRepository, llm LLMClient) *Service {
	return &Service{
		vectorRepo: vectorRepo,
		llm:        llm,
	}
}

func (s *Service) Consolidate(ctx context.Context, taskID string, logs []string) error {
	// 1. Gather text
	var combinedLogs string
	for _, log := range logs {
		combinedLogs += log + "\n"
	}

	if len(combinedLogs) == 0 {
		return nil // Nothing to consolidate
	}

	// 2. Synthesize
	prompt := fmt.Sprintf("Summarize the key technical decisions, user preferences, and permanent facts from these logs:\n%s", combinedLogs)
	summary, err := s.llm.Reason(ctx, prompt)
	if err != nil {
		return fmt.Errorf("failed to synthesize memory: %w", err)
	}

	// 3. Embed
	embedding, err := s.llm.GenerateEmbedding(ctx, summary)
	if err != nil {
		return fmt.Errorf("failed to generate embedding: %w", err)
	}

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return fmt.Errorf("unauthorized: missing claims or organization ID")
	}

	// 4. Persist
	record := &memory.EmbeddingRecord{
		ID:           taskID + "-summary", // Simplification
		OrganizationID: claims.OrganizationID,           // Secure isolation
		MemoryType:   "TASK_SUMMARY",
		Content:      summary,
		Embedding:    embedding,
		CreatedAt:    time.Now(),
		SourceTaskID: taskID,
	}

	if err := s.vectorRepo.Upsert(ctx, record); err != nil {
		return fmt.Errorf("failed to upsert memory record: %w", err)
	}

	return nil
}
