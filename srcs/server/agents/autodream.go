package agents

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// AutoDreamEngine manages the background consolidation of memories.
type AutoDreamEngine struct {
	db        db.Provider
	llmClient *orchestration.MinimaxClient
	ticker    *time.Ticker
	quit      chan struct{}
}

// NewAutoDreamEngine initializes the autoDream engine.
func NewAutoDreamEngine(db db.Provider, llmClient *orchestration.MinimaxClient) *AutoDreamEngine {
	return &AutoDreamEngine{
		db:        db,
		llmClient: llmClient,
		quit:      make(chan struct{}),
	}
}

// Start runs the autoDream background process.
func (ae *AutoDreamEngine) Start(ctx context.Context, interval time.Duration) {
	ae.ticker = time.NewTicker(interval)
	go func() {
		for {
			select {
			case <-ae.ticker.C:
				if err := ae.consolidate(ctx); err != nil {
					slog.Error("autodream: consolidation failed", "err", err)
				}
			case <-ae.quit:
				ae.ticker.Stop()
				return
			case <-ctx.Done():
				ae.ticker.Stop()
				return
			}
		}
	}()
}

// Stop halts the background process.
func (ae *AutoDreamEngine) Stop() {
	close(ae.quit)
}

func (ae *AutoDreamEngine) consolidate(ctx context.Context) error {
	// 1. Sweep completed shared_tasks that have not been consolidated
	// In a real implementation we'd track which tasks were consolidated via a flag or join.
	// For now, we select completed tasks.
	query := `
		SELECT id, mission_id, title, description
		FROM shared_tasks
		WHERE status = 'COMPLETED'
		ORDER BY updated_at DESC LIMIT 10
	`
	rows, err := ae.db.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query tasks: %w", err)
	}
	defer rows.Close()

	var consolidatedCount int

	for rows.Next() {
		var id, missionID, title, desc string
		if err := rows.Scan(&id, &missionID, &title, &desc); err != nil {
			slog.Warn("autodream: skip row due to scan error", "err", err)
			continue
		}

		// Check if already consolidated (basic check)
		var exists int
		checkQuery := "SELECT 1 FROM autodream_memories WHERE source_mission_id = $1 LIMIT 1"
		// handle both pg/sqlite placeholder differences gracefully or assume generic
		if ae.db.IsSQLite() {
			checkQuery = "SELECT 1 FROM autodream_memories WHERE source_mission_id = ? LIMIT 1"
		}
		_ = ae.db.QueryRow(ctx, checkQuery, missionID).Scan(&exists)
		if exists == 1 {
			continue
		}

		content := fmt.Sprintf("Task %s (Mission: %s): %s", title, missionID, desc)

		// 2. Generate embedding (if llmClient provided)
		var embedding []float32
		if ae.llmClient != nil {
			embedding, err = ae.llmClient.GenerateEmbedding(ctx, content)
			if err != nil {
				slog.Error("autodream: failed to generate embedding", "err", err)
				continue
			}
		} else {
			// Mock embedding for tests
			embedding = make([]float32, 1536)
		}

		// Convert embedding to postgres pgvector format string or sqlite BLOB
		var vectorStr interface{}
		if ae.db.IsSQLite() {
			vectorStr = fmt.Sprintf("%v", embedding) // basic string repr as text fallback
		} else {
			// pgvector format '[0.1, 0.2, ...]'
			vectorStr = formatVector(embedding)
		}

		// 3. Store in autodream_memories
		insertQuery := `
			INSERT INTO autodream_memories (content, embedding, source_mission_id)
			VALUES ($1, $2, $3)
		`
		if ae.db.IsSQLite() {
			// Use generated uuid in db layer wrapper or just standard sqlite syntax without default uuid?
			// The migration changed UUID DEFAULT gen_random_uuid() to TEXT PRIMARY KEY.
			// We need to provide the ID for sqlite.
			insertQuery = `
				INSERT INTO autodream_memories (id, content, embedding, source_mission_id)
				VALUES (?, ?, ?, ?)
			`
			id := fmt.Sprintf("%d", time.Now().UnixNano())
			_, err = ae.db.Exec(ctx, insertQuery, id, content, vectorStr, missionID)
		} else {
			_, err = ae.db.Exec(ctx, insertQuery, content, vectorStr, missionID)
		}

		if err != nil {
			slog.Error("autodream: failed to store memory", "err", err)
			continue
		}
		consolidatedCount++
	}

	if consolidatedCount > 0 {
		slog.Debug("autodream: consolidated memories", "count", consolidatedCount)
	}

	return nil
}

func formatVector(v []float32) string {
	b := []byte("[")
	for i, f := range v {
		if i > 0 {
			b = append(b, []byte(",")...)
		}
		b = append(b, []byte(fmt.Sprintf("%f", f))...)
	}
	b = append(b, []byte("]")...)
	return string(b)
}
