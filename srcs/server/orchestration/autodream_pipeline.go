package orchestration

import (
	"context"
	"encoding/json"
	"log/slog"
	"os"
	"path/filepath"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"gopkg.in/yaml.v3"
)

type EmbeddingClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamPipeline struct {
	db     db.Provider
	client EmbeddingClient
	done   chan struct{}
}

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

func (w *AutoDreamPipeline) Start(ctx context.Context) {
	ticker := time.NewTicker(5 * time.Minute)
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

func (w *AutoDreamPipeline) Stop() {
	close(w.done)
}

func (w *AutoDreamPipeline) process(ctx context.Context) {
	w.processSessionData(ctx)
	w.processMemoryFiles(ctx)
}

func (w *AutoDreamPipeline) processSessionData(ctx context.Context) {
	limit := 500
	var query string
	var args []interface{}

	if w.db.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data LIMIT ?"
		args = append(args, limit)
	} else {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data LIMIT $1 FOR UPDATE SKIP LOCKED"
		args = append(args, limit)
	}

	rows, err := w.db.Query(ctx, query, args...)
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to query agent_session_data", "error", err)
		return
	}
	defer rows.Close()

	type sessionData struct {
		id      string
		agentID string
		context string
	}
	var items []sessionData
	for rows.Next() {
		var item sessionData
		if err := rows.Scan(&item.id, &item.agentID, &item.context); err == nil {
			items = append(items, item)
		}
	}
	rows.Close()

	for _, item := range items {
		embeddingStr := "[0.0, 0.0, 0.0]"
		if w.client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, err := w.client.GenerateEmbedding(ctxTimeout, item.context)
			cancel()
			if err == nil && len(resp) > 0 {
				if bytes, err := json.Marshal(resp); err == nil {
					embeddingStr = string(bytes)
				}
			}
		}

		var insertQuery string
		var insertArgs []interface{}

		memID := uuid.New().String()
		if w.db.IsSQLite() {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, created_at)
				VALUES (?, 'default', ?, ?, ?, 'agent_session_data', CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content
			`
			insertArgs = []interface{}{memID, item.agentID, item.context, embeddingStr}
		} else {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, created_at)
				VALUES ($1, 'default', $2, $3, $4::vector, 'agent_session_data', NOW())
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
			`
			insertArgs = []interface{}{memID, item.agentID, item.context, embeddingStr}
		}

		if _, err := w.db.Exec(ctx, insertQuery, insertArgs...); err == nil {
			if w.db.IsSQLite() {
				w.db.Exec(ctx, "DELETE FROM agent_session_data WHERE session_id = ?", item.id)
			} else {
				w.db.Exec(ctx, "DELETE FROM agent_session_data WHERE session_id = $1", item.id)
			}
		}
	}
}

func (w *AutoDreamPipeline) processMemoryFiles(ctx context.Context) {
	matches, err := filepath.Glob(".agent-task/memory/*.yml")
	if err != nil || len(matches) == 0 {
		return
	}

	limit := 500
	if len(matches) > limit {
		matches = matches[:limit]
	}

	for _, file := range matches {
		data, err := os.ReadFile(file)
		if err != nil { continue }

		type MemoryFile struct {
			AgentSessionData string `yaml:"agent_session_data"`
			Content          string `yaml:"content"`
		}
		var memFile MemoryFile
		if err := yaml.Unmarshal(data, &memFile); err != nil { continue }

		contentToEmbed := memFile.AgentSessionData
		if contentToEmbed == "" { contentToEmbed = memFile.Content }
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

		var insertQuery string
		var insertArgs []interface{}

		memID := uuid.New().String()
		if w.db.IsSQLite() {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, created_at)
				VALUES (?, 'default', 'system', ?, ?, 'memory_file', CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content
			`
			insertArgs = []interface{}{memID, contentToEmbed, embeddingStr}
		} else {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, created_at)
				VALUES ($1, 'default', 'system', $2, $3::vector, 'memory_file', NOW())
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
			`
			insertArgs = []interface{}{memID, contentToEmbed, embeddingStr}
		}

		if _, err := w.db.Exec(ctx, insertQuery, insertArgs...); err == nil {
			os.Remove(file)
		}
	}
}
