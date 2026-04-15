package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"gopkg.in/yaml.v3"
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

	matches, err := filepath.Glob(".agent-task/memory/*.yml")
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to glob memory files", "error", err)
		return
	}
	if len(matches) == 0 {
		return // nothing to process
	}

	limit := 500
	if len(matches) > limit {
		matches = matches[:limit]
	}

	for _, file := range matches {
		data, err := os.ReadFile(file)
		if err != nil {
			slog.Error("AutoDreamPipeline: failed to read memory file", "file", file, "error", err)
			continue
		}

		var memFile struct {
			AgentSessionData string `yaml:"agent_session_data"`
			Content          string `yaml:"content"`
		}

		if err := yaml.Unmarshal(data, &memFile); err != nil {
			slog.Error("AutoDreamPipeline: failed to unmarshal memory file", "file", file, "error", err)
			continue
		}

		contentToEmbed := memFile.AgentSessionData
		if contentToEmbed == "" {
			contentToEmbed = memFile.Content
		}
		if contentToEmbed == "" {
			os.Remove(file)
			continue
		}

		missionID := strings.TrimSuffix(filepath.Base(file), ".yml")

		embeddingStr := "[0.0, 0.0, 0.0]"
		if p.client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, err := p.client.GenerateEmbedding(ctxTimeout, contentToEmbed)
			cancel()
			if err == nil && len(resp) > 0 {
				if bytes, err := json.Marshal(resp); err == nil {
					embeddingStr = string(bytes)
				}
			} else if err != nil {
				slog.Warn("AutoDreamPipeline: failed to generate embedding", "error", err)
			}
		}

		var insertQuery string
		var insertArgs []interface{}

		memID := missionID

		insertQuery = `
			INSERT INTO autodream_memories (id, task_id, content, embedding, created_at)
			VALUES ($1, $2, $3, $4::vector, CURRENT_TIMESTAMP)
			ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
		`
		insertArgs = []interface{}{memID, missionID, contentToEmbed, embeddingStr}

		if _, err := p.db.Exec(ctx, insertQuery, insertArgs...); err != nil {
			slog.Warn("AutoDreamPipeline: failed to insert memory", "id", memID, "error", err)
		} else {
			slog.Debug("AutoDreamPipeline: consolidated memory", "id", memID)
			os.Remove(file)
		}
	}

	slog.Info("AutoDreamPipeline: completed sweep", "processed", len(matches))
}