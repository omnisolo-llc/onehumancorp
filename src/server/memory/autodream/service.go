package autodream

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"time"
	"encoding/json"

	"github.com/fsnotify/fsnotify"
	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/memory"
	"github.com/onehumancorp/mono/src/server/telemetry"
	"gopkg.in/yaml.v3"
)

// ensure Service implements MemoryConsolidator
var _ MemoryConsolidator = (*Service)(nil)

// MemoryFile represents the structure of the YAML memory files
type MemoryFile struct {
	TaskID    string `yaml:"task_id"`
	AgentRole string `yaml:"agent_role"`
	Content   string `yaml:"content"`
}

// PushDBProvider defines the query operations required to run the PushToCloud sync
type PushDBProvider interface {
	Query(ctx context.Context, sql string, optionsAndArgs ...any) (interface{
		Next() bool
		Scan(dest ...any) error
		Close()
	}, error)
	Exec(ctx context.Context, sql string, arguments ...any) (int64, error)
}

type Service struct {
	vectorRepo *memory.VectorRepository
	llm        LLMClient
	watchDir   string
}

func NewService(vectorRepo *memory.VectorRepository, llm LLMClient, watchDir string) *Service {
	return &Service{
		vectorRepo: vectorRepo,
		llm:        llm,
		watchDir:   watchDir,
	}
}

func chunkString(s string, chunkSize int) []string {
	var chunks []string
	runes := []rune(s)
	for i := 0; i < len(runes); i += chunkSize {
		end := i + chunkSize
		if end > len(runes) {
			end = len(runes)
		}
		chunks = append(chunks, string(runes[i:end]))
	}
	return chunks
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

	chunks := chunkString(summary, 2000)

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return fmt.Errorf("unauthorized: missing claims or organization ID")
	}

	for i, chunk := range chunks {
		embedding, err := s.llm.GenerateEmbedding(ctx, chunk)
		if err != nil {
			return fmt.Errorf("failed to generate embedding: %w", err)
		}

		record := &memory.EmbeddingRecord{
			ID:             fmt.Sprintf("%s-summary-%d", taskID, i),
			OrganizationID: claims.OrganizationID,
			MemoryType:     "TASK_SUMMARY",
			Content:        chunk,
			Embedding:      embedding,
			CreatedAt:      time.Now(),
			SourceTaskID:   taskID,
		}

		if s.vectorRepo != nil {
			if err := s.vectorRepo.Upsert(ctx, record); err != nil {
				return fmt.Errorf("failed to upsert memory record: %w", err)
			}
		}
	}

	return nil
}

func (s *Service) ResolveConflicts(ctx context.Context, organizationID string) error {
	if s.vectorRepo == nil {
		return fmt.Errorf("vector repo is nil")
	}
	conflicts, err := s.vectorRepo.FindConflicts(ctx, organizationID, 0.05)
	if err != nil {
		return err
	}

	for _, conflict := range conflicts {
		prompt := fmt.Sprintf("Merge these two related or conflicting facts into one clear fact:\n1. %s\n2. %s", conflict.Content1, conflict.Content2)
		merged, err := s.llm.Reason(ctx, prompt)
		if err != nil {
			continue
		}

		if err := s.vectorRepo.DeleteMemories(ctx, []string{conflict.ID1, conflict.ID2}); err != nil {
			continue
		}

		emb, err := s.llm.GenerateEmbedding(ctx, merged)
		if err != nil {
			continue
		}

		record := &memory.EmbeddingRecord{
			ID:             conflict.ID1 + "-merged",
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
	if s.vectorRepo == nil {
		return fmt.Errorf("vector repo is nil")
	}
	cutoff := time.Now().Add(-olderThan)
	return s.vectorRepo.PruneOlderThan(ctx, organizationID, cutoff)
}

func (s *Service) StartWatcher(ctx context.Context) error {
	if s.watchDir == "" {
		return nil
	}

	if err := os.MkdirAll(s.watchDir, 0755); err != nil {
		return fmt.Errorf("failed to create watch directory: %w", err)
	}

	// Baseline sweep
	entries, err := os.ReadDir(s.watchDir)
	if err == nil {
		for _, entry := range entries {
			if !entry.IsDir() && (filepath.Ext(entry.Name()) == ".yml" || filepath.Ext(entry.Name()) == ".yaml") {
				_ = s.processMemoryFile(ctx, filepath.Join(s.watchDir, entry.Name()))
			}
		}
	}

	watcher, err := fsnotify.NewWatcher()
	if err != nil {
		return fmt.Errorf("failed to create watcher: %w", err)
	}

	go func() {
		defer watcher.Close()
		for {
			select {
			case <-ctx.Done():
				return
			case event, ok := <-watcher.Events:
				if !ok {
					return
				}
				if event.Op&fsnotify.Create == fsnotify.Create || event.Op&fsnotify.Write == fsnotify.Write {
					if filepath.Ext(event.Name) == ".yml" || filepath.Ext(event.Name) == ".yaml" {
						if err := s.processMemoryFile(ctx, event.Name); err != nil {
							fmt.Printf("Error processing memory file %s: %v\n", event.Name, err)
						}
					}
				}
			case err, ok := <-watcher.Errors:
				if !ok {
					return
				}
				fmt.Printf("Watcher error: %v\n", err)
			}
		}
	}()

	err = watcher.Add(s.watchDir)
	if err != nil {
		return fmt.Errorf("failed to add directory to watcher: %w", err)
	}

	return nil
}

func (s *Service) processMemoryFile(ctx context.Context, filepath string) error {
	data, err := os.ReadFile(filepath)
	if err != nil {
		return fmt.Errorf("failed to read file: %w", err)
	}

	var memFile MemoryFile
	if err := yaml.Unmarshal(data, &memFile); err != nil {
		return fmt.Errorf("failed to parse yaml: %w", err)
	}

	if memFile.TaskID == "" || memFile.Content == "" {
		return fmt.Errorf("invalid memory file: missing task_id or content")
	}

	orgID := "sys"
	claims := auth.ClaimsFromContext(ctx)
	if claims != nil && claims.OrganizationID != "" {
		orgID = claims.OrganizationID
	}

	chunks := chunkString(memFile.Content, 2000)

	for i, chunk := range chunks {
		embedding, err := s.llm.GenerateEmbedding(ctx, chunk)
		if err != nil {
			return fmt.Errorf("failed to generate embedding: %w", err)
		}

		record := &memory.EmbeddingRecord{
			ID:             fmt.Sprintf("%s-%d-%d", memFile.TaskID, time.Now().UnixNano(), i),
			OrganizationID: orgID,
			MemoryType:     memFile.AgentRole,
			Content:        chunk,
			Embedding:      embedding,
			CreatedAt:      time.Now(),
			SourceTaskID:   memFile.TaskID,
		}

		if s.vectorRepo != nil {
			if err := s.vectorRepo.Upsert(ctx, record); err != nil {
				return fmt.Errorf("failed to upsert memory record: %w", err)
			}
		}
	}

	return nil
}

func (s *Service) PushToCloud(ctx context.Context, dbProvider PushDBProvider) error {
	if dbProvider == nil {
		return fmt.Errorf("db provider is nil")
	}

	// Fetch unsynced records from consolidated_memory
	query := `SELECT id, organization_id, memory_type, content, embedding, created_at, source_task_id FROM consolidated_memory WHERE synced_to_cloud = false`

	rows, err := dbProvider.Query(ctx, query)
	if err != nil {
		// Table or column might not exist if migrations are behind, fail gracefully
		return fmt.Errorf("failed to query unsynced records: %w", err)
	}
	defer rows.Close()

	syncedCount := 0
	var idsToUpdate []string

	for rows.Next() {
		var id, orgID, memType, content, sourceTaskID string
		var embeddingJSON []byte
		var createdAt time.Time
		if err := rows.Scan(&id, &orgID, &memType, &content, &embeddingJSON, &createdAt, &sourceTaskID); err != nil {
			fmt.Printf("Error scanning row: %v\n", err)
			continue
		}

		var embedding []float32
		if err := json.Unmarshal(embeddingJSON, &embedding); err != nil {
			fmt.Printf("Error unmarshaling embedding: %v\n", err)
			continue
		}

		// Simulate pushing to cloud (e.g. pgvector on remote)
		// In a real implementation this would call a remote gRPC/REST API.

		idsToUpdate = append(idsToUpdate, id)
		syncedCount++
	}

	for _, id := range idsToUpdate {
		_, err := dbProvider.Exec(ctx, "UPDATE consolidated_memory SET synced_to_cloud = true WHERE id = $1", id)
		if err != nil {
			fmt.Printf("Error marking record %s as synced: %v\n", id, err)
		}
	}

	telemetry.RecordRAGRecordsSynced(ctx, int64(syncedCount))
	return nil
}
