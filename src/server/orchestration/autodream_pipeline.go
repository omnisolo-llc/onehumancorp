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

	"gopkg.in/yaml.v3"

	"github.com/google/uuid"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/telemetry"
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

// process performs a sweep to consolidate ephemeral memories from the DB.
// File-based ingestion via OHC_MEMORY_DIR is handled separately by
// AutoDreamWorker.ingestAgentMemories.
func (p *AutoDreamPipeline) process(ctx context.Context) {
	slog.Info("AutoDreamPipeline: starting memory consolidation sweep")

	// 1. Extraction: Poll recent agent_session_data
	threshold := time.Now().Add(-1 * time.Hour).UTC()
	var query string
	if p.db.IsSQLite() {
		query = "SELECT session_id, agent_id, context_data, organization_id FROM agent_session_data WHERE last_accessed < ? LIMIT 50"
	} else {
		query = "SELECT session_id, agent_id, context_data, organization_id FROM agent_session_data WHERE last_accessed < $1 LIMIT 50 FOR UPDATE SKIP LOCKED"
	}

	rows, err := p.db.Query(ctx, query, threshold)
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to fetch stale sessions", "error", err)
		return
	}

	type Session struct {
		ID             string
		AgentID        string
		ContextData    string
		OrganizationID string
	}

	var sessions []Session
	for rows.Next() {
		var s Session
		if err := rows.Scan(&s.ID, &s.AgentID, &s.ContextData, &s.OrganizationID); err == nil {
			sessions = append(sessions, s)
		}
	}
	rows.Close()

	if len(sessions) > 0 {
		for _, s := range sessions {
			summary := s.ContextData
			var embeddingStr string
			if p.client != nil {
				ctxTimeout, cancel := context.WithTimeout(ctx, 15*time.Second)
				embedding, embedErr := p.client.GenerateEmbedding(ctxTimeout, summary)
				cancel()
				if embedErr == nil && len(embedding) > 0 {
					if bytes, err := json.Marshal(embedding); err == nil {
						embeddingStr = string(bytes)
					}
				}
			}

			if embeddingStr == "" {
				var vec []string
				for i := 0; i < 1536; i++ {
					vec = append(vec, "0.0")
				}
				embeddingStr = "[" + strings.Join(vec, ",") + "]"
			}

			err = func() error {
				tx, err := p.db.Begin(ctx)
				if err != nil {
					return err
				}
				defer tx.Rollback(ctx)

				var insertQuery string
				if p.db.IsSQLite() {
					insertQuery = "INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES (?, ?, ?, ?, ?, 'session_compression')"
					_, err = tx.Exec(ctx, insertQuery, s.ID, s.OrganizationID, s.AgentID, summary, embeddingStr)
					if err == nil {
						_, err = tx.Exec(ctx, "INSERT INTO swarm_long_term_memory (id, topic, summary, embedding) VALUES (?, ?, ?, ?)", uuid.New().String(), "Session Compression: "+s.ID, summary, embeddingStr)
					}
				} else {
					insertQuery = "INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type) VALUES ($1, $2, $3, $4, $5::vector, 'session_compression')"
					_, err = tx.Exec(ctx, insertQuery, s.ID, s.OrganizationID, s.AgentID, summary, embeddingStr)
					if err == nil {
						_, err = tx.Exec(ctx, "INSERT INTO swarm_long_term_memory (id, topic, summary, embedding) VALUES ($1, $2, $3, $4::vector)", uuid.New().String(), "Session Compression: "+s.ID, summary, embeddingStr)
					}
				}

				if err != nil {
					return err
				}

				var delQuery string
				if p.db.IsSQLite() {
					delQuery = "DELETE FROM agent_session_data WHERE session_id = ?"
				} else {
					delQuery = "DELETE FROM agent_session_data WHERE session_id = $1"
				}
				_, err = tx.Exec(ctx, delQuery, s.ID)
				if err != nil {
					return err
				}

				return tx.Commit(ctx)
			}()
			if err != nil {
				slog.Error("AutoDreamPipeline: failed to consolidate DB memory", "error", err)
			}
		}
	}

	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		return // DB-backed memory only; no file processing
	}
	matches, err := filepath.Glob(filepath.Join(memoryDir, "*.yml"))
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
			OrganizationID   string `yaml:"organization_id"`
		}

		if err := yaml.Unmarshal(data, &memFile); err != nil {
			slog.Error("AutoDreamPipeline: failed to unmarshal memory file", "file", file, "error", err)
			continue
		}

		contentToEmbed := memFile.AgentSessionData
		if contentToEmbed == "" {
			contentToEmbed = memFile.Content
		}

		orgID := memFile.OrganizationID
		if orgID == "" {
			orgID = "system"
		}
		if contentToEmbed == "" {
			os.Remove(file)
			continue
		}

		chunks := chunkText(contentToEmbed, 8000)
		missionID := strings.TrimSuffix(filepath.Base(file), ".yml")

		success := true
		for i, chunk := range chunks {
			memID := missionID
			if len(chunks) > 1 {
				memID = fmt.Sprintf("%s-chunk%d", missionID, i)
			}

			var vec []string
			for i := 0; i < 1536; i++ {
				vec = append(vec, "0.0")
			}
			embeddingStr := "[" + strings.Join(vec, ",") + "]"

			if p.client != nil {
				ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
				resp, err := p.client.GenerateEmbedding(ctxTimeout, chunk)
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

			if p.db.IsSQLite() {
				insertQuery = `
					INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at)
					VALUES (?, ?, 'auto-dream-pipeline', ?, ?, 'memory_file', CURRENT_TIMESTAMP)
					ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
				`
				insertArgs = []interface{}{memID, orgID, chunk, embeddingStr}
				if _, err := p.db.Exec(ctx, insertQuery, insertArgs...); err != nil {
					slog.Warn("AutoDreamPipeline: failed to insert memory chunk", "id", memID, "error", err)
					success = false
				} else {
					_, err = p.db.Exec(ctx, "INSERT INTO swarm_long_term_memory (id, topic, summary, embedding) VALUES (?, ?, ?, ?)", uuid.New().String(), "Memory File: "+memID, chunk, embeddingStr)
					if err != nil {
						slog.Warn("AutoDreamPipeline: failed to insert into swarm_long_term_memory", "id", memID, "error", err)
						success = false
					} else {
						slog.Debug("AutoDreamPipeline: consolidated memory chunk", "id", memID)
					}
				}
			} else {
				insertQuery = `
					INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at)
					VALUES ($1, $2, 'auto-dream-pipeline', $3, $4::vector, 'memory_file', NOW())
					ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
				`
				insertArgs = []interface{}{memID, orgID, chunk, embeddingStr}
				if _, err := p.db.Exec(ctx, insertQuery, insertArgs...); err != nil {
					slog.Warn("AutoDreamPipeline: failed to insert memory chunk", "id", memID, "error", err)
					success = false
				} else {
					_, err = p.db.Exec(ctx, "INSERT INTO swarm_long_term_memory (id, topic, summary, embedding) VALUES ($1, $2, $3, $4::vector)", uuid.New().String(), "Memory File: "+memID, chunk, embeddingStr)
					if err != nil {
						slog.Warn("AutoDreamPipeline: failed to insert into swarm_long_term_memory", "id", memID, "error", err)
						success = false
					} else {
						slog.Debug("AutoDreamPipeline: consolidated memory chunk", "id", memID)
					}
				}
			}
		}
		if success {
			os.Remove(file)
		}
	}

	telemetry.RecordAutoDreamConsolidation(ctx, "autodream_pipeline")
	slog.Info("AutoDreamPipeline: completed sweep", "processed", len(matches))
}

// chunkText splits a string into chunks of a given maximum size (in runes).
func chunkText(text string, chunkSize int) []string {
	if chunkSize <= 0 {
		return []string{text}
	}
	runes := []rune(text)
	var chunks []string
	for len(runes) > 0 {
		if len(runes) < chunkSize {
			chunks = append(chunks, string(runes))
			break
		}
		chunks = append(chunks, string(runes[:chunkSize]))
		runes = runes[chunkSize:]
	}
	return chunks
}
