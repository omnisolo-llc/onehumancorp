package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// EmbeddingClient interface for dependency injection and testing
type EmbeddingClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// AutoDreamPipeline is the daemon process for long-term memory consolidation
type AutoDreamPipeline struct {
	db     db.Provider
	client EmbeddingClient
	done   chan struct{}
}

// NewAutoDreamPipeline creates a new pipeline instance
func NewAutoDreamPipeline(provider db.Provider) *AutoDreamPipeline {
	var client EmbeddingClient
	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	if minimaxKey != "" {
		client = NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), provider, nil)
	}

	return &AutoDreamPipeline{
		db:     provider,
		client: client,
		done:   make(chan struct{}),
	}
}

// Start begins the background pipeline process
func (p *AutoDreamPipeline) Start(ctx context.Context) {
	ticker := time.NewTicker(5 * time.Minute) // run periodically
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			p.Stop()
			return
		case <-p.done:
			return
		case <-ticker.C:
			p.process(context.Background())
		}
	}
}

// Stop halts the pipeline
func (p *AutoDreamPipeline) Stop() {
	close(p.done)
}

// process performs a sweep to consolidate ephemeral memories
func (p *AutoDreamPipeline) process(ctx context.Context) {
	slog.Info("AutoDreamPipeline: starting memory consolidation sweep")

	files, err := os.ReadDir(".agent-task/memory/")
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to read memory directory", "error", err)
		return
	}

	for _, file := range files {
		if file.IsDir() {
			continue
		}

		path := ".agent-task/memory/" + file.Name()
		data, err := os.ReadFile(path)
		if err != nil {
			continue
		}

		embeddingStr := "[0.0, 0.0, 0.0]"
		if p.client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, err := p.client.GenerateEmbedding(ctxTimeout, string(data))
			cancel()
			if err == nil && len(resp) > 0 {
				if bytes, err := json.Marshal(resp); err == nil {
					embeddingStr = string(bytes)
				}
			}
		}

		var insertQuery string
		if p.db.IsSQLite() {
			insertQuery = `
				INSERT INTO autodream_memories_master_master (id, content, embedding)
				VALUES (?, ?, ?)
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content
			`
		} else {
			insertQuery = `
				INSERT INTO autodream_memories_master_master (id, content, embedding)
				VALUES ($1, $2, $3::vector)
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
			`
		}

		if _, err := p.db.Exec(ctx, insertQuery, file.Name(), string(data), embeddingStr); err != nil {
			slog.Warn("AutoDreamPipeline: failed to insert memory", "id", file.Name(), "error", err)
		} else {
			slog.Debug("AutoDreamPipeline: consolidated memory", "id", file.Name())
			os.Remove(path) // Delete file after processing to prevent infinite loop
		}
	}

	slog.Info("AutoDreamPipeline: completed sweep")
}
