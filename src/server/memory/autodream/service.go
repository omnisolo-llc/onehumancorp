package autodream

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/memory"
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
	var combinedLogs string
	for _, log := range logs {
		combinedLogs += log + "\n"
	}

	if len(combinedLogs) == 0 {
		return nil
	}

	prompt := fmt.Sprintf("Summarize the key technical decisions, user preferences, and permanent facts from these logs:\n%s", combinedLogs)
	summary, err := s.llm.Reason(ctx, prompt)
	if err != nil {
		return fmt.Errorf("failed to synthesize memory: %w", err)
	}

	embedding, err := s.llm.GenerateEmbedding(ctx, summary)
	if err != nil {
		return fmt.Errorf("failed to generate embedding: %w", err)
	}

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return fmt.Errorf("unauthorized: missing claims or organization ID")
	}

	// Semantic Search to find existing similar memories to resolve conflicts
	results, err := s.vectorRepo.SemanticSearch(ctx, claims.OrganizationID, embedding, 1)
	if err == nil && len(results) > 0 {
		top := results[0]
		// If similarity > 0.90, merge to resolve conflict
		if top.Score > 0.90 && top.Record.MemoryType == "TASK_SUMMARY" {
			mergePrompt := fmt.Sprintf("Merge these two summaries into one cohesive memory, resolving any conflicting facts by keeping the newer information:\nOld: %s\nNew: %s", top.Record.Content, summary)
			mergedSummary, err := s.llm.Reason(ctx, mergePrompt)
			if err == nil {
				mergedEmbedding, err := s.llm.GenerateEmbedding(ctx, mergedSummary)
				if err == nil {
					top.Record.Content = mergedSummary
					top.Record.Embedding = mergedEmbedding
					top.Record.CreatedAt = time.Now()
					top.Record.SourceTaskID = taskID
					if err := s.vectorRepo.UpdateRecord(ctx, top.Record); err != nil {
						return fmt.Errorf("failed to update merged memory record: %w", err)
					}
					return nil // Successfully updated existing memory
				}
			}
		}
	}

	record := &memory.EmbeddingRecord{
		ID:             taskID + "-summary",
		OrganizationID: claims.OrganizationID,
		MemoryType:     "TASK_SUMMARY",
		Content:        summary,
		Embedding:      embedding,
		CreatedAt:      time.Now(),
		SourceTaskID:   taskID,
	}

	if err := s.vectorRepo.Upsert(ctx, record); err != nil {
		return fmt.Errorf("failed to upsert memory record: %w", err)
	}

	return nil
}

func (s *Service) PruneStaleContext(ctx context.Context, retention time.Duration) error {
	// Prune TASK_SUMMARY memories that are older than the specified retention.
	// We only aggressively prune ephemeral or old task summaries that haven't been touched,
	// keeping in mind the rule to be conservative.
	if err := s.vectorRepo.DeleteOldMemories(ctx, "TASK_SUMMARY", retention); err != nil {
		return fmt.Errorf("failed to prune stale context: %w", err)
	}
	return nil
}
