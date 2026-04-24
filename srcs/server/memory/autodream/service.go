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

	// 4. Conflict Resolution (detect and resolve old facts)
	// We only resolve conflicts for PERMANENT_FACT memory types, since TASK_SUMMARY should just be historical logs.
	// But actually, this task is just TASK_SUMMARY. The prompt mentions "detects when the same fact is stored multiple times with different values".
	// Let's implement a specific LLM-based fact extraction for facts if we wanted to, but the current code just creates TASK_SUMMARY.
	// Let's refine this: only detect conflicts if it's a specific memory type, like "FACT".
	// Since we are writing a generic consolidator, we'll avoid deleting history. We will not use SemanticSearch for deletion here to be conservative.

	// 5. Persist
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

// ExtractAndStoreFacts uses the LLM to extract permanent facts and stores them, overwriting old ones.
func (s *Service) ExtractAndStoreFacts(ctx context.Context, taskID string, content string) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return fmt.Errorf("unauthorized: missing claims or organization ID")
	}

	prompt := fmt.Sprintf("Extract any permanent business facts from this content (e.g. prices, rules). Return them as a list of independent sentences:\n%s", content)
	factsStr, err := s.llm.Reason(ctx, prompt)
	if err != nil {
		return fmt.Errorf("failed to extract facts: %w", err)
	}

	// Assume we just treat the whole output as one fact block for simplicity, but we could split.
	embedding, err := s.llm.GenerateEmbedding(ctx, factsStr)
	if err != nil {
		return fmt.Errorf("failed to generate embedding: %w", err)
	}

	// Find conflicts in facts
	similar, err := s.vectorRepo.SemanticSearch(ctx, claims.OrganizationID, embedding, 5, 0.90)
	if err == nil {
		for _, mem := range similar {
			if mem.MemoryType == "PERMANENT_FACT" {
				_ = s.vectorRepo.Delete(ctx, mem.ID, claims.OrganizationID)
			}
		}
	}

	record := &memory.EmbeddingRecord{
		ID:             taskID + "-fact",
		OrganizationID: claims.OrganizationID,
		MemoryType:     "PERMANENT_FACT",
		Content:        factsStr,
		Embedding:      embedding,
		CreatedAt:      time.Now(),
		SourceTaskID:   taskID,
	}

	return s.vectorRepo.Upsert(ctx, record)
}

// PruneStaleMemories runs background cleanup of old context.
func (s *Service) PruneStaleMemories(ctx context.Context, organizationID string, retention time.Duration) (int64, error) {
	// Conservative pruning: only prune transient task summaries, and only for this tenant
	olderThan := time.Now().Add(-retention)
	return s.vectorRepo.PruneStale(ctx, organizationID, "TASK_SUMMARY", olderThan)
}
