package pipeline

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"time"

	"gopkg.in/yaml.v3"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type AutoDreamPipeline struct {
	pool db.Provider
}

func NewAutoDreamPipeline(pool db.Provider) *AutoDreamPipeline {
	return &AutoDreamPipeline{pool: pool}
}

func (p *AutoDreamPipeline) Start(ctx context.Context) {
	slog.Info("Starting AutoDream pipeline loop")
	ticker := time.NewTicker(10 * time.Minute)
	defer ticker.Stop()

	// Run once initially
	if err := p.Run(ctx); err != nil {
		slog.Error("AutoDreamPipeline initial run failed", "error", err)
	}

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if err := p.Run(ctx); err != nil {
				slog.Error("AutoDreamPipeline run failed", "error", err)
			}
		}
	}
}

func (p *AutoDreamPipeline) Run(ctx context.Context) error {
	// First run the orchestration AutoDream worker steps to consolidate
	worker := orchestration.NewAutoDreamWorker(p.pool)
	if err := worker.ConsolidateEpoch(ctx); err != nil {
		slog.Error("AutoDreamPipeline: ConsolidateEpoch failed", "error", err)
	}

	// 1. Extraction: Poll recent agent_session_data and .agent-task/memory/*.yml
	// We'll read the agent_memories table or the swarm_dream_epochs table.
	// But let's also directly scan the database for completed tasks or agent sessions.
	var query string
	if p.pool.IsSQLite() {
		query = `
			SELECT 'session-' || session_id, context_data FROM agent_session_data ORDER BY last_accessed DESC LIMIT 50
		`
	} else {
		query = `
			SELECT 'session-' || session_id, CAST(context_data AS TEXT) FROM agent_session_data ORDER BY last_accessed DESC LIMIT 50
		`
	}

	rows, err := p.pool.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query agent_session_data: %w", err)
	}
	defer rows.Close()

	var memories []string
	for rows.Next() {
		var id, contextStr string
		if err := rows.Scan(&id, &contextStr); err == nil {
			memories = append(memories, contextStr)
		}
	}

	// Read .agent-task/memory/*.yml files
	matches, _ := filepath.Glob(".agent-task/memory/*.yml")
	for _, file := range matches {
		data, err := os.ReadFile(file)
		if err == nil {
			var memFile struct {
				AgentSessionData string `yaml:"agent_session_data"`
				Content          string `yaml:"content"`
			}
			if yaml.Unmarshal(data, &memFile) == nil {
				if memFile.AgentSessionData != "" {
					memories = append(memories, memFile.AgentSessionData)
				} else if memFile.Content != "" {
					memories = append(memories, memFile.Content)
				}
			}
		}
	}

	if len(memories) == 0 {
		return nil
	}

	// 2. Consolidation: Group them
	combined := strings.Join(memories, "\n---\n")

	// 3. Embedding: Summarize and generate embedding
	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client orchestration.MinimaxClient


	summary := combined
	var embedding []float32

	if minimaxKey != "" {
		client = orchestration.NewMinimaxClient(minimaxKey)
		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		resp, err := client.Reason(ctxTimeout, "Summarize these memories compactly:\n"+combined)
		cancel()
		if err == nil {
			summary = resp
		}

		ctxTimeout2, cancel2 := context.WithTimeout(ctx, 30*time.Second)
		embResp, err := client.GenerateEmbedding(ctxTimeout2, summary)
		cancel2()
		if err == nil && len(embResp) == 1536 {
			embedding = embResp
		}
	} else {
		// Just a fallback
		embedding = make([]float32, 1536)
	}

	if len(embedding) == 0 {
		embedding = make([]float32, 1536)
	}

	// Format vector string
	embStrs := make([]string, len(embedding))
	for i, v := range embedding {
		embStrs[i] = fmt.Sprintf("%f", v)
	}
	vectorStr := "[" + strings.Join(embStrs, ",") + "]"

	// 4. Loading: Upsert into consolidated_memory
	memID := uuid.New().String()
	if p.pool.IsSQLite() {
		insertQuery := `
			INSERT INTO consolidated_memory (id, organization_id, agent_id, content, source_type, created_at)
			VALUES (?, 'system', 'system', ?, 'pipeline', CURRENT_TIMESTAMP)
		`
		_, err = p.pool.Exec(ctx, insertQuery, memID, summary)
	} else {
		insertQuery := `
			INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at)
			VALUES ($1, 'system', 'system', $2, $3::vector, 'pipeline', NOW())
		`
		_, err = p.pool.Exec(ctx, insertQuery, memID, summary, vectorStr)
	}

	if err != nil {
		return fmt.Errorf("failed to insert consolidated memory: %w", err)
	}

	slog.Info("AutoDreamPipeline: Successfully consolidated memory", "id", memID)
	return nil
}
