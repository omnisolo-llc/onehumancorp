package autodream

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
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
		ID:             taskID + "-summary", // Simplification
		OrganizationID: claims.OrganizationID,           // Secure isolation
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

type ConflictResolutionResponse struct {
	ResolvedFact   string   `json:"resolved_fact"`
	ConflictingIDs []string `json:"conflicting_ids"`
}

func (s *Service) ResolveConflicts(ctx context.Context, organizationID string, topic string) error {
	if organizationID == "" {
		return fmt.Errorf("organization ID cannot be empty")
	}

	// 1. Embed topic
	topicEmb, err := s.llm.GenerateEmbedding(ctx, topic)
	if err != nil {
		return fmt.Errorf("failed to embed topic: %w", err)
	}

	// 2. Search for relevant memories
	memories, err := s.vectorRepo.SemanticSearch(ctx, organizationID, topicEmb, 10)
	if err != nil {
		return fmt.Errorf("failed to search for conflicting memories: %w", err)
	}

	if len(memories) <= 1 {
		return nil // No conflict if 0 or 1 memory
	}

	var combinedContext string
	for _, mem := range memories {
		combinedContext += fmt.Sprintf("- ID: %s (Date: %s, Source: %s): %s\n", mem.ID, mem.CreatedAt.Format(time.RFC3339), mem.SourceTaskID, mem.Content)
	}

	// 3. Ask LLM to resolve and identify ONLY the genuinely conflicting IDs.
	prompt := fmt.Sprintf(`Given the following facts about the topic '%s', identify if there are any genuine conflicts.
If there are conflicts, resolve them by favoring more recent facts, explicit owner overrides, or higher reliability sources.
Return your answer strictly in the following JSON format without markdown blocks:
{
  "resolved_fact": "The single source of truth without prefix or explanation",
  "conflicting_ids": ["id_of_conflict_1", "id_of_conflict_2"]
}

Only include the IDs of the facts that directly conflict with the resolved fact or are now obsolete because of it. Do not include IDs of facts that are simply related to the search but not in conflict.

Facts:
%s`, topic, combinedContext)

	rawResponse, err := s.llm.Reason(ctx, prompt)
	if err != nil {
		return fmt.Errorf("failed to resolve conflict via LLM: %w", err)
	}

	// Strip markdown blocks if the LLM still returns them
	rawResponse = strings.TrimPrefix(rawResponse, "```json\n")
	rawResponse = strings.TrimSuffix(rawResponse, "\n```")
	rawResponse = strings.TrimSpace(rawResponse)

	var resolution ConflictResolutionResponse
	if err := json.Unmarshal([]byte(rawResponse), &resolution); err != nil {
		return fmt.Errorf("failed to parse LLM conflict resolution response: %w", err)
	}

	if len(resolution.ConflictingIDs) == 0 {
		return nil // No genuine conflicts found by the LLM
	}

	resolvedEmb, err := s.llm.GenerateEmbedding(ctx, resolution.ResolvedFact)
	if err != nil {
		return fmt.Errorf("failed to embed resolved fact: %w", err)
	}

	// 4. Delete ONLY the identified conflicting memories
	for _, id := range resolution.ConflictingIDs {
		if err := s.vectorRepo.Delete(ctx, id, organizationID); err != nil {
			// Log and continue, best effort deletion
			fmt.Printf("warning: failed to delete old memory %s: %v\n", id, err)
		}
	}

	// 5. Upsert new resolved fact
	record := &memory.EmbeddingRecord{
		ID:             fmt.Sprintf("resolved-%d", time.Now().UnixNano()),
		OrganizationID: organizationID,
		MemoryType:     "RESOLVED_FACT",
		Content:        resolution.ResolvedFact,
		Embedding:      resolvedEmb,
		CreatedAt:      time.Now(),
		SourceTaskID:   "system-resolver",
	}

	if err := s.vectorRepo.Upsert(ctx, record); err != nil {
		return fmt.Errorf("failed to upsert resolved memory: %w", err)
	}

	return nil
}

func (s *Service) PruneStaleContext(ctx context.Context, organizationID string, threshold time.Duration) (int64, error) {
	if organizationID == "" {
		return 0, fmt.Errorf("organization ID cannot be empty")
	}

	olderThan := time.Now().Add(-threshold)
	return s.vectorRepo.PruneStaleContext(ctx, organizationID, olderThan)
}

func (s *Service) GetSharedContext(ctx context.Context, query string) (string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", fmt.Errorf("unauthorized: missing claims or organization ID")
	}

	queryEmb, err := s.llm.GenerateEmbedding(ctx, query)
	if err != nil {
		return "", fmt.Errorf("failed to generate embedding for query: %w", err)
	}

	memories, err := s.vectorRepo.SemanticSearch(ctx, claims.OrganizationID, queryEmb, 5)
	if err != nil {
		return "", fmt.Errorf("failed to search for context: %w", err)
	}

	var sharedContext string
	for _, mem := range memories {
		sharedContext += mem.Content + "\n"
	}

	return sharedContext, nil
}
