package autodream

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
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

type ConsolidateResponse struct {
	SupersededIDs []string `json:"superseded_ids"`
	Summary       string   `json:"summary"`
}

func (s *Service) Consolidate(ctx context.Context, taskID string, logs []string) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return fmt.Errorf("unauthorized: missing claims or organization ID")
	}

	var combinedLogs string
	for _, log := range logs {
		combinedLogs += log + "\n"
	}

	if len(combinedLogs) == 0 {
		return nil
	}

	logsEmbedding, err := s.llm.GenerateEmbedding(ctx, combinedLogs)
	if err != nil {
		return fmt.Errorf("failed to generate embedding for logs: %w", err)
	}

	existingMemories, err := s.vectorRepo.SearchConsolidatedMemories(ctx, claims.OrganizationID, "", logsEmbedding, 5)
	if err != nil {
		return fmt.Errorf("failed to search existing memories: %w", err)
	}

	var existingContext string
	for _, mem := range existingMemories {
		existingContext += fmt.Sprintf("- ID: %s, Date: %s, Content: %s\n", mem.ID, mem.CreatedAt.Format(time.RFC3339), mem.Content)
	}

	jsonPrompt := fmt.Sprintf(`You are an AI Memory Consolidator.
Here are existing memories related to the new information:
%s

Here are the new logs from a recent task:
%s

Identify which existing memories are DIRECTLY CONFLICTING with or SUPERSEDED by the new logs.
If an existing memory is unrelated or still valid, do NOT list it as superseded.
Respond ONLY with a valid JSON object matching this schema:
{
  "superseded_ids": ["id1", "id2"],
  "summary": "The synthesized, conflict-resolved memory content capturing key decisions and facts."
}`, existingContext, combinedLogs)

	jsonResponse, err := s.llm.Reason(ctx, jsonPrompt)
	if err != nil {
		return fmt.Errorf("failed to synthesize memory: %w", err)
	}

	// Clean Markdown backticks if any
	jsonResponse = strings.TrimPrefix(jsonResponse, "```json")
	jsonResponse = strings.TrimPrefix(jsonResponse, "```")
	jsonResponse = strings.TrimSuffix(jsonResponse, "```")
	jsonResponse = strings.TrimSpace(jsonResponse)

	var result ConsolidateResponse
	if err := json.Unmarshal([]byte(jsonResponse), &result); err != nil {
		// Fallback if parsing fails - just use the whole text as summary and supersede nothing
		result.Summary = jsonResponse
		result.SupersededIDs = []string{}
	}

	if result.Summary == "" {
		return nil
	}

	embedding, err := s.llm.GenerateEmbedding(ctx, result.Summary)
	if err != nil {
		return fmt.Errorf("failed to generate embedding: %w", err)
	}

	record := &memory.ConsolidatedMemoryRecord{
		ID:             taskID + "-consolidated",
		OrganizationID: claims.OrganizationID,
		AgentID:        "", // Cross-department by default
		Content:        result.Summary,
		Embedding:      embedding,
		SourceType:     "TASK_SUMMARY",
		CreatedAt:      time.Now(),
	}

	if err := s.vectorRepo.UpsertConsolidatedMemory(ctx, record); err != nil {
		return fmt.Errorf("failed to upsert consolidated memory record: %w", err)
	}

	// Only delete the specific IDs that the LLM identified as superseded or conflicting
	for _, oldMemID := range result.SupersededIDs {
		_ = s.vectorRepo.DeleteConsolidatedMemory(ctx, oldMemID, claims.OrganizationID)
	}

	return nil
}

func (s *Service) PruneStaleMemories(ctx context.Context, organizationID string, olderThan time.Duration) error {
	cutoff := time.Now().Add(-olderThan)

	oldMemories, err := s.vectorRepo.GetOldMemories(ctx, organizationID, cutoff, 100)
	if err != nil {
		return fmt.Errorf("failed to fetch old memories: %w", err)
	}

	for _, mem := range oldMemories {
		prompt := fmt.Sprintf(`You are an AI Memory Pruner.
Here is an old memory from %s:
"%s"

Is this memory likely stale, completely irrelevant, or no longer useful for a business context?
Answer exactly with "YES" if it should be pruned, or "NO" if it should be kept.`, mem.CreatedAt.Format(time.RFC3339), mem.Content)

		decision, err := s.llm.Reason(ctx, prompt)
		if err != nil {
			continue // Skip on error
		}

		decision = strings.TrimSpace(decision)

		if decision == "YES" {
			_ = s.vectorRepo.DeleteConsolidatedMemory(ctx, mem.ID, organizationID)
		}
	}

	return nil
}

// StartGlobalPruningWorker starts a background ticker that periodically runs PruneStaleMemories for all tenants.
// We pass a db provider to fetch all unique organization_ids to prune globally.
func (s *Service) StartGlobalPruningWorker(ctx context.Context, interval time.Duration, olderThan time.Duration, provider db.Provider) {
	ticker := time.NewTicker(interval)
	go func() {
		for {
			select {
			case <-ctx.Done():
				ticker.Stop()
				return
			case <-ticker.C:
				// Fetch all unique organization IDs that have memories
				rows, err := provider.Query(ctx, "SELECT DISTINCT organization_id FROM consolidated_memory")
				if err != nil {
					continue
				}

				var orgIDs []string
				for rows.Next() {
					var orgID string
					if err := rows.Scan(&orgID); err == nil {
						orgIDs = append(orgIDs, orgID)
					}
				}
				rows.Close()

				for _, orgID := range orgIDs {
					_ = s.PruneStaleMemories(ctx, orgID, olderThan)
				}
			}
		}
	}()
}
