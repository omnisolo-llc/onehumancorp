package memory

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"io/fs"
	"log"
	"os"
	"path/filepath"
	"strings"
	"time"
	"regexp"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

// LLMClient represents a minimal interface for generating embeddings
type LLMClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// AutoDreamDaemon represents the background worker
type AutoDreamDaemon struct {
	db              *sql.DB
	llmClient       LLMClient
	memoryDir       string
	missionsDir     string
	pollInterval    time.Duration
	memoriesCounter metric.Int64Counter
}

// NewAutoDreamDaemon initializes the AutoDream background pipeline
func NewAutoDreamDaemon(db *sql.DB, llmClient LLMClient, memoryDir, missionsDir string, pollInterval time.Duration) (*AutoDreamDaemon, error) {
	meter := otel.Meter("autodream")
	counter, err := meter.Int64Counter(
		"autodream.processed_memories",
		metric.WithDescription("Number of memories processed by AutoDream"),
	)
	if err != nil {
		return nil, fmt.Errorf("failed to create metric counter: %w", err)
	}

	return &AutoDreamDaemon{
		db:              db,
		llmClient:       llmClient,
		memoryDir:       memoryDir,
		missionsDir:     missionsDir,
		pollInterval:    pollInterval,
		memoriesCounter: counter,
	}, nil
}

// Run starts the daemon
func (d *AutoDreamDaemon) Run(ctx context.Context) {
	ticker := time.NewTicker(d.pollInterval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			log.Println("AutoDreamDaemon stopping...")
			return
		case <-ticker.C:
			d.processDirectories(ctx)
		}
	}
}

func (d *AutoDreamDaemon) processDirectories(ctx context.Context) {
	dirs := []string{d.memoryDir, d.missionsDir}
	for _, dir := range dirs {
		err := filepath.WalkDir(dir, func(path string, info fs.DirEntry, err error) error {
			if err != nil {
				return nil // Skip on error
			}
			if info.IsDir() {
				return nil
			}

			// Process only markdown or json files typically used
			if strings.HasSuffix(path, ".md") || strings.HasSuffix(path, ".json") {
				d.processFile(ctx, path)
			}
			return nil
		})
		if err != nil {
			log.Printf("Error walking directory %s: %v", dir, err)
		}
	}
}

func (d *AutoDreamDaemon) processFile(ctx context.Context, path string) {
	contentBytes, err := os.ReadFile(path)
	if err != nil {
		log.Printf("Failed to read file %s: %v", path, err)
		return
	}
	content := string(contentBytes)

	// Check if file contains status: DONE (case-sensitive as requested, or adjust if needed)
	if !strings.Contains(content, "status: DONE") {
		return
	}

	// Generate embedding
	embedding, err := d.llmClient.GenerateEmbedding(ctx, content)
	if err != nil {
		log.Printf("Failed to generate embedding for %s: %v", path, err)
		return
	}

	// Convert []float32 to []byte (JSON encoding for simplicity, actual pgvector would use specific byte format or pg type)
	embeddingBytes, err := json.Marshal(embedding)
	if err != nil {
		log.Printf("Failed to marshal embedding for %s: %v", path, err)
		return
	}

	// Extract metadata
	orgID := "system"
	agentID := "system"
	taskID := "system"

	// Try extracting from content
	orgRe := regexp.MustCompile("(?i)(?:tenant_id|organization_id):\\s*([a-zA-Z0-9_-]+)")
	if matches := orgRe.FindStringSubmatch(content); len(matches) > 1 {
		orgID = matches[1]
	}
	agentRe := regexp.MustCompile("(?i)agent_id:\\s*([a-zA-Z0-9_-]+)")
	if matches := agentRe.FindStringSubmatch(content); len(matches) > 1 {
		agentID = matches[1]
	}
	taskRe := regexp.MustCompile("(?i)task_id:\\s*([a-zA-Z0-9_-]+)")
	if matches := taskRe.FindStringSubmatch(content); len(matches) > 1 {
		taskID = matches[1]
	}

	// Try extracting from filename fallback (e.g., agent-123_task-456_tenant-789.md)
	base := filepath.Base(path)
	if orgID == "system" {
		if matches := regexp.MustCompile("tenant-([a-zA-Z0-9_-]+)").FindStringSubmatch(base); len(matches) > 1 {
			orgID = matches[1]
		}
	}

	// Upsert into DB
	err = d.upsertMemory(ctx, base, orgID, agentID, taskID, content, embeddingBytes)
	if err != nil {
		log.Printf("Failed to upsert memory for %s: %v", path, err)
		return
	}

	// Increment metric
	d.memoriesCounter.Add(ctx, 1)

	// Optionally mark as processed to avoid reprocessing, here we can rename or delete.
	// For now, we rename it by adding .processed
	os.Rename(path, path+".processed")
}

func (d *AutoDreamDaemon) upsertMemory(ctx context.Context, id string, orgID string, agentID string, taskID string, content string, embedding []byte) error {
	tx, err := d.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	_, err = tx.ExecContext(ctx, "SELECT set_config('app.current_tenant', $1, true)", orgID)
	if err != nil {
		// Ignore syntax errors or function not found for sqlite testing fallback, if any
		if !strings.Contains(err.Error(), "syntax error") && !strings.Contains(err.Error(), "no such function: set_config") {
			return err
		}
	}

	query := `
		INSERT INTO autodream_memories (id, organization_id, agent_id, task_id, content, embedding, source_type)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
		ON CONFLICT(id) DO UPDATE SET
			content = excluded.content,
			embedding = excluded.embedding
	`
	_, err = tx.ExecContext(ctx, query, id, orgID, agentID, taskID, content, string(embedding), "autodream")
	if err != nil {
		return err
	}

	return tx.Commit()
}
