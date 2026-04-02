package agents

import (
	"context"
	"database/sql"
	"log"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// AutoDreamEngine runs the background memory consolidation pipeline.
type AutoDreamEngine struct {
	hub *orchestration.Hub
}

// NewAutoDreamEngine creates a new AutoDreamEngine
func NewAutoDreamEngine(hub *orchestration.Hub) *AutoDreamEngine {
	return &AutoDreamEngine{hub: hub}
}

// Start spawns the background goroutine to periodically sweep memory.
func (e *AutoDreamEngine) Start(ctx context.Context, tickDuration time.Duration) {
	ticker := time.NewTicker(tickDuration)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			e.ProcessMemoryConsolidation(ctx)
		}
	}
}

// ProcessMemoryConsolidation sweeps completed tasks and generates embeddings.
func (e *AutoDreamEngine) ProcessMemoryConsolidation(ctx context.Context) {
	if e.hub == nil || e.hub.GetSIPDB() == nil || e.hub.GetSIPDB().DB() == nil {
		return
	}

	db := e.hub.GetSIPDB().DB()
	apiKey := e.hub.MinimaxAPIKey()

	// Skip if no API key is available
	if apiKey == "" {
		return
	}

	minimaxClient := orchestration.NewMinimaxClient(apiKey)

	// In a real implementation, we would query `swarm_tasks` where status='COMPLETED'
	// and they haven't been consolidated yet, or query `swarm_memory`.
	// For this exercise we fetch some COMPLETED tasks to consolidate.
	query := "SELECT id, title, payload FROM swarm_tasks WHERE status = 'COMPLETED' ORDER BY updated_at DESC LIMIT 10"

	rows, err := db.Query(ctx, query)
	if err != nil {
		log.Printf("autodream: query error: %v", err)
		return
	}
	defer rows.Close()

	for rows.Next() {
		var id, title, payload string
		if err := rows.Scan(&id, &title, &payload); err != nil {
			continue
		}

		// Generate summary and embeddings
		prompt := "Summarize the architectural findings of this task: " + title + "\n" + payload
		summary, err := minimaxClient.Reason(ctx, prompt)
		if err != nil {
			log.Printf("autodream: LLM reasoning error for task %s: %v", id, err)
			continue
		}

		// Fake embedding for now since we don't have a real embedding API
		// Real system would call minimaxClient.Embed(summary)
		dummyEmbedding := "[0.1, 0.2, 0.3]"

		insertQuery := "INSERT INTO swarm_long_term_memory (topic, summary, embedding) VALUES ($1, $2, $3)"
		_, err = db.Exec(ctx, insertQuery, title, summary, dummyEmbedding)
		if err != nil {
			// Some databases might complain about VECTOR type strings if we just send string.
			// In production pgvector expects array syntax.
			// SQLite replacement changes VECTOR to BLOB or TEXT, so it usually succeeds.
			// We handle errors gracefully.
			log.Printf("autodream: insert memory error for task %s: %v", id, err)
		}

		// Mark task as consolidated (we could add a consolidated_at flag or change status)
		// To avoid infinite loops in this mock we might update status to 'CONSOLIDATED'
		db.Exec(ctx, "UPDATE swarm_tasks SET status = 'CONSOLIDATED' WHERE id = $1", id)
	}
}
