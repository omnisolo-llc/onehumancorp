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
		ID:             taskID + "-summary",   // Simplification
		OrganizationID: claims.OrganizationID, // Secure isolation
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

func (s *Service) ResolveConflicts(ctx context.Context, organizationID string) error {
	conflicts, err := s.vectorRepo.FindConflicts(ctx, organizationID, 0.05)
	if err != nil {
		return err
	}

	for _, c := range conflicts {
		// Automatic Resolution Logic based on Source Reliability or Recency

		// 1. Source Reliability (Owner Override wins)
		var loserID string
		if c.SourceType1 == "owner_override" && c.SourceType2 != "owner_override" {
			loserID = c.ID2 // Delete the loser
			_ = s.vectorRepo.DeleteMemories(ctx, []string{loserID})
			continue
		} else if c.SourceType2 == "owner_override" && c.SourceType1 != "owner_override" {
			loserID = c.ID1
			_ = s.vectorRepo.DeleteMemories(ctx, []string{loserID})
			continue
		}

		// 2. Recency (If > 24 hours apart, newest wins)
		timeDiff := c.CreatedAt1.Sub(c.CreatedAt2)
		if timeDiff < 0 {
			timeDiff = -timeDiff
		}
		if timeDiff > 24*time.Hour {
			if c.CreatedAt1.After(c.CreatedAt2) {
				_ = s.vectorRepo.DeleteMemories(ctx, []string{c.ID2})
			} else {
				_ = s.vectorRepo.DeleteMemories(ctx, []string{c.ID1})
			}
			continue
		}

		// 3. Fallback: LLM Synthesizes the conflict
		prompt := fmt.Sprintf("Merge these two related or conflicting facts into one clear fact:\n1. %s\n2. %s", c.Content1, c.Content2)
		merged, err := s.llm.Reason(ctx, prompt)
		if err != nil {
			continue
		}

		// Delete both
		if err := s.vectorRepo.DeleteMemories(ctx, []string{c.ID1, c.ID2}); err != nil {
			continue
		}

		emb, err := s.llm.GenerateEmbedding(ctx, merged)
		if err != nil {
			continue
		}

		record := &memory.EmbeddingRecord{
			ID:             c.ID1 + "-merged",
			OrganizationID: organizationID,
			MemoryType:     "MERGED_SUMMARY",
			Content:        merged,
			Embedding:      emb,
			CreatedAt:      time.Now(),
		}
		_ = s.vectorRepo.Upsert(ctx, record)
	}

	return nil
}

func (s *Service) PruneStaleContext(ctx context.Context, organizationID string, olderThan time.Duration) error {
	cutoff := time.Now().Add(-olderThan)
	// Conservative pruning implemented inside VectorRepository.PruneOlderThan
	return s.vectorRepo.PruneOlderThan(ctx, organizationID, cutoff)
}

// CrossDepartmentShare allows explicit sharing of memories between different AI agents/departments
func (s *Service) CrossDepartmentShare(ctx context.Context, organizationID string, fromDept string, toDept string, queryEmbedding []float32, limit int) ([]*memory.EmbeddingRecord, error) {
	// Search context written by the originating department
	records, err := s.vectorRepo.SemanticSearch(ctx, organizationID, queryEmbedding, limit)
	if err != nil {
		return nil, err
	}

	var shared []*memory.EmbeddingRecord
	for _, r := range records {
		if r.MemoryType == fromDept {
			// Tag memory for the target department implicitly by returning it
			// Alternatively, we could persist a copy, but returning it suffices for sharing context.
			shared = append(shared, r)
		}
	}
	return shared, nil
}
