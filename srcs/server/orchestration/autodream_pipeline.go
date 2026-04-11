package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"gopkg.in/yaml.v3"
)

// EmbeddingClient interface for dependency injection and testing
type EmbeddingClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// AutoDreamWorker is the daemon process for long-term memory consolidation
type AutoDreamWorker struct {
	db     db.Provider
	client EmbeddingClient
	done   chan struct{}
}

// NewAutoDreamWorker creates a new pipeline instance
func NewAutoDreamWorker(provider db.Provider) *AutoDreamWorker {
	var client EmbeddingClient
	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	if minimaxKey != "" {
		client = NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), provider, nil)
	}

	return &AutoDreamWorker{
		db:     provider,
		client: client,
		done:   make(chan struct{}),
	}
}

// Start begins the background pipeline process
func (w *AutoDreamWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(5 * time.Minute) // run periodically
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			w.Stop()
			return
		case <-w.done:
			return
		case <-ticker.C:
			w.process(context.Background())
		}
	}
}

// Stop halts the pipeline
func (w *AutoDreamWorker) Stop() {
	close(w.done)
}

func (w *AutoDreamWorker) processFiles(ctx context.Context) {
	matches, err := filepath.Glob(".agent-task/memory/*.yml")
	if err != nil || len(matches) == 0 {
		return
	}

	for _, file := range matches {
		data, err := os.ReadFile(file)
		if err != nil {
			continue
		}

		var memFile struct {
			AgentSessionData string `yaml:"agent_session_data"`
			Content          string `yaml:"content"`
		}
		if err := yaml.Unmarshal(data, &memFile); err != nil {
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

		embeddingStr := "[0.0, 0.0, 0.0]"

		if w.client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, err := w.client.GenerateEmbedding(ctxTimeout, contentToEmbed)
			cancel()
			if err == nil && len(resp) > 0 {
				if bytes, err := json.Marshal(resp); err == nil {
					embeddingStr = string(bytes)
				}
			}
		}

		memID := uuid.New().String()
		var insertQuery string
		var insertArgs []interface{}

		if w.db.IsSQLite() {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, created_at)
				VALUES (?, 'system', 'system', ?, ?, 'file', CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content
			`
			insertArgs = []interface{}{memID, contentToEmbed, embeddingStr}
		} else {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, created_at)
				VALUES ($1, 'system', 'system', $2, $3::vector, 'file', NOW())
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
			`
			insertArgs = []interface{}{memID, contentToEmbed, embeddingStr}
		}

		if _, err := w.db.Exec(ctx, insertQuery, insertArgs...); err == nil {
			os.Remove(file)
		}
	}
}

// process performs a sweep to consolidate ephemeral memories
func (w *AutoDreamWorker) process(ctx context.Context) {
	slog.Info("AutoDreamWorker: starting memory consolidation sweep")

	w.processFiles(ctx)

	limit := 500

	var query string
	var args []interface{}

	if w.db.IsSQLite() {
		query = `
			SELECT id, organization_id, agent_id, payload
			FROM shared_tasks
			WHERE status = 'COMPLETED'
			ORDER BY updated_at DESC LIMIT ?
		`
		args = append(args, limit)
	} else {
		query = `
			SELECT id, organization_id, agent_id, payload
			FROM shared_tasks
			WHERE status = 'COMPLETED'
			ORDER BY updated_at DESC LIMIT $1 FOR UPDATE SKIP LOCKED
		`
		args = append(args, limit)
	}

	rows, err := w.db.Query(ctx, query, args...)
	if err != nil {
		slog.Error("AutoDreamWorker: failed to query shared_tasks", "error", err)
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
		return
	}

	for _, m := range memories {
		embeddingStr := "[0.0, 0.0, 0.0]"

		if w.client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, err := w.client.GenerateEmbedding(ctxTimeout, m.payload)
			cancel()
			if err == nil && len(resp) > 0 {
				if bytes, err := json.Marshal(resp); err == nil {
					embeddingStr = string(bytes)
				}
			}
		}

		var insertQuery string
		var insertArgs []interface{}

		if w.db.IsSQLite() {
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

		w.db.Exec(ctx, insertQuery, insertArgs...)
	}

	slog.Info("AutoDreamWorker: completed sweep", "processed", len(memories))
}
