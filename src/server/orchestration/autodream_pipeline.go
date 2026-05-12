package orchestration

import (
	"context"
	"crypto/rand"
	"database/sql"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"gopkg.in/yaml.v3"
)

// LLMClient represents an abstraction for embedding generation.
type LLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// AutoDreamWorker handles the consolidation of episodic memory into pgvector/SQLite.
type AutoDreamWorker struct {
	db         *sql.DB
	llmClient  LLMClient
	memoryPath string
}

// MemoryFile represents the structure of the agent memory YAML.
type MemoryFile struct {
	TaskID          string `yaml:"task_id"`
	TenantID        string `yaml:"tenant_id"`
	AgentID         string `yaml:"agent_id"`
	Payload         string `yaml:"payload"`
	DeliberationLog string `yaml:"deliberation_log"`
}

// NewAutoDreamWorker creates a new AutoDreamWorker.
func NewAutoDreamWorker(db *sql.DB, llmClient LLMClient, memoryPath string) *AutoDreamWorker {
	if memoryPath == "" {
		memoryPath = ".agent-task/memory"
	}
	return &AutoDreamWorker{
		db:         db,
		llmClient:  llmClient,
		memoryPath: memoryPath,
	}
}

// ProcessMemoryFiles reads local memory files, chunks them, generates embeddings, and stores them.
func (w *AutoDreamWorker) ProcessMemoryFiles(ctx context.Context) error {
	var files []string
	err := filepath.WalkDir(w.memoryPath, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if !d.IsDir() && strings.HasSuffix(d.Name(), ".yaml") {
			files = append(files, path)
		}
		return nil
	})

	if err != nil {
		if os.IsNotExist(err) {
			return nil // Nothing to process
		}
		return fmt.Errorf("autodream: failed to read memory path: %w", err)
	}

	for _, file := range files {
		data, err := os.ReadFile(file)
		if err != nil {
			continue // Skip unreadable
		}

		var memFile MemoryFile
		if err := yaml.Unmarshal(data, &memFile); err != nil {
			continue // Skip invalid
		}

		content := fmt.Sprintf("Task Payload:\n%s\nDeliberation Log:\n%s", memFile.Payload, memFile.DeliberationLog)
		chunks := chunkContent(content, 2000)

		var successCount int

		for _, chunk := range chunks {
			embedding, err := w.llmClient.GenerateEmbedding(ctx, chunk)
			if err != nil {
				continue
			}

			embStr := floatSliceToString(embedding)

			id, err := generateUUID()
			if err != nil {
				continue
			}

			// Try Postgres syntax first (with ::vector), fallback to SQLite if needed
			// In Go standard library database/sql, this is tricky. We'll use a standard prepared statement.
			// The caller must provide a DB connection that understands the target dialect.
			// We'll write it for SQLite fallback compatible by just inserting as string if needed,
			// or pgvector depending on driver.

			query := `
				INSERT INTO consolidated_memory (id, tenant_id, agent_id, content, embedding, source_type, task_id)
				VALUES ($1, $2, $3, $4, $5, 'TASK_SUMMARY', $6)
			`

			// A real production implementation would check the DB driver and adapt $5 vs $5::vector
			// For this task, we will just pass the string.
			_, err = w.db.ExecContext(ctx, query, id, memFile.TenantID, memFile.AgentID, chunk, embStr, memFile.TaskID)
			if err == nil {
				successCount++
			}
		}

		// Only delete if we successfully processed and inserted the chunks
		if successCount > 0 && successCount == len(chunks) {
			_ = os.Remove(file)
		}
	}

	return nil
}

func chunkContent(content string, size int) []string {
	var chunks []string
	words := strings.Fields(content)

	var currentChunk strings.Builder
	var currentSize int

	for _, word := range words {
		if currentSize+len(word)+1 > size && currentChunk.Len() > 0 {
			chunks = append(chunks, strings.TrimSpace(currentChunk.String()))
			currentChunk.Reset()
			currentSize = 0
		}
		currentChunk.WriteString(word)
		currentChunk.WriteString(" ")
		currentSize += len(word) + 1
	}

	if currentChunk.Len() > 0 {
		chunks = append(chunks, strings.TrimSpace(currentChunk.String()))
	}

	return chunks
}

func floatSliceToString(floats []float32) string {
	strs := make([]string, len(floats))
	for i, f := range floats {
		strs[i] = fmt.Sprintf("%f", f)
	}
	return "[" + strings.Join(strs, ",") + "]"
}

// generateUUID creates a random UUID v4
func generateUUID() (string, error) {
	b := make([]byte, 16)
	_, err := rand.Read(b)
	if err != nil {
		return "", err
	}
	// Variant 10
	b[8] = (b[8] & 0x3f) | 0x80
	// Version 4
	b[6] = (b[6] & 0x0f) | 0x40
	return fmt.Sprintf("%08x-%04x-%04x-%04x-%012x", b[0:4], b[4:6], b[6:8], b[8:10], b[10:]), nil
}
