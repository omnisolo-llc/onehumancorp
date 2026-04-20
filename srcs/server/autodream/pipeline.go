package autodream

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type LLMClient interface {
	Reason(ctx context.Context, prompt string) (string, error)
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamPipeline struct {
	db     db.Provider
	llm    LLMClient
	worker *AutoDreamWorker
	done   chan struct{}
	once   sync.Once
}

func NewAutoDreamPipeline(dbProvider db.Provider, llm LLMClient, worker *AutoDreamWorker) *AutoDreamPipeline {
	return &AutoDreamPipeline{
		db:     dbProvider,
		llm:    llm,
		worker: worker,
		done:   make(chan struct{}),
	}
}

func (p *AutoDreamPipeline) Start(ctx context.Context) {
	ticker := time.NewTicker(5 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			p.Stop()
			return
		case <-p.done:
			return
		case <-ticker.C:
			p.process(ctx)
		}
	}
}

func (p *AutoDreamPipeline) Stop() {
	p.once.Do(func() {
		close(p.done)
	})
}

func (p *AutoDreamPipeline) process(ctx context.Context) {
	slog.Info("AutoDreamPipeline: starting memory consolidation sweep")

	p.processDB(ctx)
	p.processFS(ctx)

	slog.Info("AutoDreamPipeline: completed sweep")
}

func (p *AutoDreamPipeline) processDB(ctx context.Context) {
	var query string
	if p.db.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data LIMIT 50"
	} else {
		// Use SKIP LOCKED within a quick transaction to fetch IDs, then update them to a processing state
		// Or fetch normally for simplicity. We will just fetch rows quickly without locking,
		// and then process them one by one in isolated transactions.
		query = "SELECT session_id, agent_id, context_data FROM agent_session_data LIMIT 50"
	}

	rows, err := p.db.Query(ctx, query)
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to fetch stale sessions", "error", err)
		return
	}

	type Session struct {
		ID          string
		AgentID     string
		ContextData string
	}

	var sessions []Session
	for rows.Next() {
		var s Session
		if err := rows.Scan(&s.ID, &s.AgentID, &s.ContextData); err == nil {
			sessions = append(sessions, s)
		}
	}
	rows.Close()

	for _, s := range sessions {
		summary, err := p.llm.Reason(ctx, fmt.Sprintf("Summarize the following agent session context: %s", s.ContextData))
		if err != nil {
			slog.Warn("AutoDreamPipeline: failed to summarize memory", "error", err)
			continue
		}

		embedding, err := p.llm.GenerateEmbedding(ctx, summary)
		if err != nil {
			slog.Warn("AutoDreamPipeline: failed to generate embedding", "error", err)
			continue
		}

		// Individual transaction for each memory insertion
		err = func() error {
			tx, err := p.db.Begin(ctx)
			if err != nil {
				return err
			}
			defer tx.Rollback(ctx)

			var insertQuery string
			var args []interface{}
			embeddingStr := formatEmbedding(embedding)

			if p.db.IsSQLite() {
				insertQuery = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type)
					VALUES (?, 'system', ?, ?, ?, 'db_sweep')`
				args = []interface{}{s.ID, s.AgentID, summary, embeddingStr}
			} else {
				insertQuery = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type)
					VALUES ($1, 'system', $2, $3, $4::vector, 'db_sweep')`
				args = []interface{}{s.ID, s.AgentID, summary, embeddingStr}
			}

			if _, err := tx.Exec(ctx, insertQuery, args...); err != nil {
				return fmt.Errorf("insert failed: %w", err)
			}

			delQuery := "DELETE FROM agent_session_data WHERE session_id = $1"
			if p.db.IsSQLite() {
				delQuery = "DELETE FROM agent_session_data WHERE session_id = ?"
			}

			if _, err := tx.Exec(ctx, delQuery, s.ID); err != nil {
				return fmt.Errorf("delete failed: %w", err)
			}

			return tx.Commit(ctx)
		}()

		if err != nil {
			slog.Error("AutoDreamPipeline: failed to consolidate DB memory", "id", s.ID, "error", err)
		}
	}
}

func (p *AutoDreamPipeline) processFS(ctx context.Context) {
	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		return
	}

	matches, err := filepath.Glob(filepath.Join(memoryDir, "*.yml"))
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to glob memory files", "error", err)
		return
	}

	for _, file := range matches {
		data, err := os.ReadFile(file)
		if err != nil {
			continue
		}

		content := string(data)
		summary, err := p.llm.Reason(ctx, fmt.Sprintf("Summarize the following agent session file context: %s", content))
		if err != nil {
			slog.Warn("AutoDreamPipeline: failed to summarize file memory", "error", err)
			continue
		}

		embedding, err := p.llm.GenerateEmbedding(ctx, summary)
		if err != nil {
			slog.Warn("AutoDreamPipeline: failed to generate embedding for file", "error", err)
			continue
		}

		memID := filepath.Base(file)
		embeddingStr := formatEmbedding(embedding)

		var insertQuery string
		var args []interface{}

		if p.db.IsSQLite() {
			insertQuery = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type)
				VALUES (?, 'system', 'file-agent', ?, ?, 'fs_sweep')`
			args = []interface{}{memID, summary, embeddingStr}
		} else {
			insertQuery = `INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type)
				VALUES ($1, 'system', 'file-agent', $2, $3::vector, 'fs_sweep')`
			args = []interface{}{memID, summary, embeddingStr}
		}

		if _, err := p.db.Exec(ctx, insertQuery, args...); err != nil {
			slog.Error("AutoDreamPipeline: failed to consolidate FS memory", "error", err)
		} else {
			os.Remove(file)
		}
	}
}

func formatEmbedding(v []float32) string {
	var vec []string
	for _, val := range v {
		vec = append(vec, fmt.Sprintf("%f", val))
	}
	return "[" + strings.Join(vec, ",") + "]"
}
