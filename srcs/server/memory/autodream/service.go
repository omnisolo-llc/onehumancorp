package autodream

import (
	"context"
	"fmt"
	"strings"
	"time"

	"github.com/google/uuid"
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
		ID:             taskID + "-summary", // Simplification
		OrganizationID: claims.OrganizationID,
		TaskID:         taskID,
		Content:        summary,
		Embedding:      embedding,
		SourceType:     "TASK_SUMMARY",
		CreatedAt:      time.Now(),
	}

	if err := s.vectorRepo.Upsert(ctx, record); err != nil {
		return fmt.Errorf("failed to upsert memory record: %w", err)
	}

	return nil
}

// PruneStaleMemories removes memories older than the given threshold.
func (s *Service) PruneStaleMemories(ctx context.Context, threshold time.Duration) error {
	cutoff := time.Now().Add(-threshold)
	return s.vectorRepo.Prune(ctx, cutoff)
}

// ResolveConflicts finds memories similar to the given memory ID, synthesizes a consolidated version, and replaces the conflicting ones.
func (s *Service) ResolveConflicts(ctx context.Context, memoryID string, organizationID string) error {
	// 1. Get the target memory
	target, err := s.vectorRepo.GetByID(ctx, memoryID, true)
	if err != nil {
		return fmt.Errorf("failed to get target memory: %w", err)
	}

	var embedding []float32
	if target.Embedding != nil && len(target.Embedding) > 0 {
		embedding = target.Embedding
	} else {
		// Re-embed content if missing
		emb, err := s.llm.GenerateEmbedding(ctx, target.Content)
		if err != nil {
			return fmt.Errorf("failed to embed target memory for search: %w", err)
		}
		embedding = emb
	}

	// 2. Search for similar memories with a distance threshold to avoid grouping unrelated memories
	// 0.25 is a reasonable threshold for cosine distance
	similar, err := s.vectorRepo.SemanticSearchWithThreshold(ctx, organizationID, embedding, 5, 0.25)
	if err != nil {
		return fmt.Errorf("failed to search similar memories: %w", err)
	}

	// If no similar memories (other than itself), nothing to do
	if len(similar) <= 1 {
		return nil
	}

	// 3. Synthesize conflicts
	var contents []string
	var conflictIDs []string
	for _, sim := range similar {
		contents = append(contents, sim.Content)
		conflictIDs = append(conflictIDs, sim.ID)
	}

	combinedContent := strings.Join(contents, "\n---\n")
	prompt := fmt.Sprintf("Analyze these potentially conflicting memories and synthesize a single accurate, resolved fact. Resolve any contradictions based on recency or clarity:\n%s", combinedContent)

	resolvedContent, err := s.llm.Reason(ctx, prompt)
	if err != nil {
		return fmt.Errorf("failed to synthesize resolved memory: %w", err)
	}

	resolvedEmbedding, err := s.llm.GenerateEmbedding(ctx, resolvedContent)
	if err != nil {
		return fmt.Errorf("failed to embed resolved memory: %w", err)
	}

	// 4. Persist resolved memory
	newRecord := &memory.EmbeddingRecord{
		ID:             uuid.New().String(),
		OrganizationID: organizationID,
		TaskID:         target.TaskID,
		Content:        resolvedContent,
		Embedding:      resolvedEmbedding,
		SourceType:     "RESOLVED_CONFLICT",
		CreatedAt:      time.Now(),
	}

	if err := s.vectorRepo.Upsert(ctx, newRecord); err != nil {
		return fmt.Errorf("failed to upsert resolved memory: %w", err)
	}

	// 5. Delete conflicting memories
	for _, id := range conflictIDs {
		if err := s.vectorRepo.Delete(ctx, id); err != nil {
			return fmt.Errorf("failed to delete conflicting memory %s: %w", id, err)
		}
	}

	return nil
}
