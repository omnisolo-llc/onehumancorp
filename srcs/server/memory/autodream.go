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

	// Upsert into DB
	err = d.upsertMemory(ctx, filepath.Base(path), content, embeddingBytes)
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

func (d *AutoDreamDaemon) upsertMemory(ctx context.Context, id string, content string, embedding []byte) error {
	// The problem statement requires []byte for vector_embedding
	query := `
		INSERT INTO memory_embeddings (id, content, vector_embedding)
		VALUES (?, ?, ?)
		ON CONFLICT(id) DO UPDATE SET
			content = excluded.content,
			vector_embedding = excluded.vector_embedding
	`
	_, err := d.db.ExecContext(ctx, query, id, content, embedding)
	return err
}
