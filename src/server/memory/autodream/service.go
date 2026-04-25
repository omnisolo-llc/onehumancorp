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

	if s.llm == nil {
		// Degrade gracefully if no LLM is configured
		return nil
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

	// Conflict Resolution: Search for similar facts in the past to merge or override
	similarRecords, err := s.vectorRepo.SemanticSearch(ctx, claims.OrganizationID, embedding, 3)
	if err == nil && len(similarRecords) > 0 {
		var similarContext string
		for _, rec := range similarRecords {
			similarContext += fmt.Sprintf("- (ID: %s) %s\n", rec.ID, rec.Content)
		}

		resolvePrompt := fmt.Sprintf(
			"You are a memory consolidation agent. Here is a newly extracted fact:\n%s\n\n"+
			"Here are similar past facts:\n%s\n\n"+
			"If the new fact conflicts with the past facts, resolve the conflict by keeping the new fact (it is more recent) but incorporate any missing details from the old facts. If they don't conflict, just return the new fact merged with any relevant context. Do not include introductory text, just the resolved fact.",
			summary, similarContext)

		resolvedSummary, err := s.llm.Reason(ctx, resolvePrompt)
		if err == nil && resolvedSummary != "" {
			summary = resolvedSummary

			// Delete the old superseded records to avoid redundant, contradictory information
			for _, rec := range similarRecords {
				// Fire-and-forget delete on older similar records
				_ = s.vectorRepo.Delete(ctx, rec.ID)
			}

			// Re-embed the resolved summary
			resolvedEmbedding, err := s.llm.GenerateEmbedding(ctx, summary)
			if err == nil {
				embedding = resolvedEmbedding
			}
		}
	}

	// 4. Persist
	record := &memory.EmbeddingRecord{
		ID:             taskID + "-summary", // Simplification
		OrganizationID: claims.OrganizationID,           // Secure isolation
		SourceType:     "TASK_SUMMARY",
		Content:        summary,
		Embedding:      embedding,
		CreatedAt:      time.Now(),
	}

	if err := s.vectorRepo.Upsert(ctx, record); err != nil {
		return fmt.Errorf("failed to upsert memory record: %w", err)
	}

	return nil
}

// PruneStaleContext removes context older than the specified threshold for a specific tenant.
func (s *Service) PruneStaleContext(ctx context.Context, orgID string, maxAge time.Duration) error {
	olderThan := time.Now().Add(-maxAge)
	return s.vectorRepo.PruneStale(ctx, orgID, olderThan)
}



// ResolveConflicts searches for conflicting context and merges them.
func (s *Service) ResolveConflicts(ctx context.Context, orgID string) error {
	recentMemories, err := s.vectorRepo.GetRecentMemories(ctx, orgID, time.Now().Add(-24*time.Hour))
	if err != nil {
		return fmt.Errorf("failed to fetch recent memories: %w", err)
	}

	for _, recentRec := range recentMemories {
		if recentRec.Embedding == nil || len(recentRec.Embedding) == 0 {
			continue
		}

		similarRecords, err := s.vectorRepo.SemanticSearch(ctx, orgID, recentRec.Embedding, 5)
		if err != nil || len(similarRecords) <= 1 {
			continue // No similar records found (only itself or none)
		}

		var similarContext string
		var idsToDelete []string
		hasOther := false

		for _, rec := range similarRecords {
			if rec.ID == recentRec.ID {
				continue
			}
			hasOther = true
			similarContext += fmt.Sprintf("- (ID: %s) %s\n", rec.ID, rec.Content)
			idsToDelete = append(idsToDelete, rec.ID)
		}

		if !hasOther {
			continue
		}
        idsToDelete = append(idsToDelete, recentRec.ID)

		resolvePrompt := fmt.Sprintf(
			"You are a memory consolidation agent. Here is a recently added fact:\n%s\n\n"+
				"Here are similar past facts:\n%s\n\n"+
				"If the new fact conflicts with the past facts, resolve the conflict by keeping the new fact (it is more recent) but incorporate any missing details from the old facts. If they don't conflict, just return the new fact merged with any relevant context. Do not include introductory text, just the resolved fact.",
			recentRec.Content, similarContext)

		resolvedSummary, err := s.llm.Reason(ctx, resolvePrompt)
		if err != nil || resolvedSummary == "" {
			continue
		}

		resolvedEmbedding, err := s.llm.GenerateEmbedding(ctx, resolvedSummary)
		if err != nil {
			continue
		}

		for _, id := range idsToDelete {
			_ = s.vectorRepo.Delete(ctx, id)
		}

		resolvedRecord := &memory.EmbeddingRecord{
			ID:             recentRec.ID + "-merged",
			OrganizationID: orgID,
			SourceType:     "TASK_SUMMARY",
			Content:        resolvedSummary,
			Embedding:      resolvedEmbedding,
			CreatedAt:      time.Now(),
		}

		_ = s.vectorRepo.Upsert(ctx, resolvedRecord)
	}

	return nil
}
