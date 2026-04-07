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

	// Limit to process batches and prevent unbound queue growth
	limit := 500

	// 1. Fetch recently completed shared_tasks
	var query string
	var args []interface{}

	if p.db.IsSQLite() {
		// SQLite degradation mode
		query = `
			SELECT id, organization_id, agent_id, payload
			FROM shared_tasks
			WHERE status = 'COMPLETED'
			ORDER BY updated_at DESC LIMIT ?
		`
		args = append(args, limit)
	} else {
		// Postgres mode using SKIP LOCKED for concurrent worker safety
		query = `
			SELECT id, organization_id, agent_id, payload
			FROM shared_tasks
			WHERE status = 'COMPLETED'
			ORDER BY updated_at DESC LIMIT $1 FOR UPDATE SKIP LOCKED
		`
		args = append(args, limit)
	}

	rows, err := p.db.Query(ctx, query, args...)
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to query shared_tasks", "error", err)
		return
	}
	defer rows.Close()

	type taskMem struct {
		id      string
		orgID   string
		agentID string
		payload string
	}

	var memories []taskMem
	for rows.Next() {
		var m taskMem
		if err := rows.Scan(&m.id, &m.orgID, &m.agentID, &m.payload); err == nil {
			memories = append(memories, m)
		}
	}
	rows.Close()

	if len(memories) == 0 {
		return // nothing to process
	}

	for _, m := range memories {
		embeddingStr := "[0.0, 0.0, 0.0]"

		if p.client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, err := p.client.GenerateEmbedding(ctxTimeout, m.payload)
			cancel()
			if err == nil && len(resp) > 0 {
				if bytes, err := json.Marshal(resp); err == nil {
					embeddingStr = string(bytes)
				}
			} else if err != nil {
				slog.Warn("AutoDreamPipeline: failed to generate embedding", "error", err)
			}
		}

		// 2. Load into autodream_memories
		var insertQuery string
		var insertArgs []interface{}

		if p.db.IsSQLite() {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, created_at)
				VALUES (?, ?, ?, ?, ?, 'shared_task', CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content
			`
			insertArgs = []interface{}{m.id, m.orgID, m.agentID, m.payload, embeddingStr}
		} else {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, created_at)
				VALUES ($1, $2, $3, $4, $5::vector, 'shared_task', NOW())
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
			`
			insertArgs = []interface{}{m.id, m.orgID, m.agentID, m.payload, embeddingStr}
		}

		if _, err := p.db.Exec(ctx, insertQuery, insertArgs...); err != nil {
			slog.Warn("AutoDreamPipeline: failed to insert memory", "id", m.id, "error", err)
		} else {
			slog.Debug("AutoDreamPipeline: consolidated memory", "id", m.id)
		}
	}

	slog.Info("AutoDreamPipeline: completed sweep", "processed", len(memories))
}
