package agents

import (
	"context"
	"database/sql"
	"encoding/json"
	"log/slog"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// AutoDreamManager manages memory consolidation sweeps
type AutoDreamManager struct {
	dbProvider db.Provider
	provider   Provider
	stopChan   chan struct{}
}

// NewAutoDreamManager creates a new autoDream service
func NewAutoDreamManager(dbProvider db.Provider, provider Provider) *AutoDreamManager {
	return &AutoDreamManager{
		dbProvider: dbProvider,
		provider:   provider,
		stopChan:   make(chan struct{}),
	}
}

// Start begins the autoDream background process
func (adm *AutoDreamManager) Start() {
	go func() {
		// Run once on startup, then every 1 hour (or fast for test)
		interval := 1 * time.Hour
		if os.Getenv("OHC_TEST_MODE") == "true" {
			interval = 500 * time.Millisecond
		}

		ticker := time.NewTicker(interval)
		defer ticker.Stop()

		adm.sweep(context.Background())

		for {
			select {
			case <-adm.stopChan:
				return
			case <-ticker.C:
				adm.sweep(context.Background())
			}
		}
	}()
}

// Stop halts the background process
func (adm *AutoDreamManager) Stop() {
	close(adm.stopChan)
}

func (adm *AutoDreamManager) sweep(ctx context.Context) {
	// 1. Sweep completed tasks
	var tasks []struct {
		ID      string
		Content string
	}

	query := "SELECT id, title || ': ' || description FROM shared_tasks WHERE status = 'COMPLETED' AND updated_at > NOW() - INTERVAL '24 HOURS'"
	if adm.dbProvider.IsSQLite() {
		query = "SELECT id, title || ': ' || description FROM shared_tasks WHERE status = 'COMPLETED' AND updated_at > datetime('now', '-24 hours')"
	}

	rows, err := adm.dbProvider.Query(ctx, query)
	if err != nil {
		if err != sql.ErrNoRows && err.Error() != "no such table: shared_tasks" {
			slog.Error("AutoDream sweep failed to fetch completed tasks", "error", err)
		}
	} else {
		defer rows.Close()
		for rows.Next() {
			var id, content string
			if err := rows.Scan(&id, &content); err == nil {
				tasks = append(tasks, struct{ID, Content string}{id, content})
			}
		}
	}

	// For each, generate embedding and store
	for _, task := range tasks {
		embeddingBytes := adm.generateEmbedding(ctx, task.Content)

		insertQuery := "INSERT INTO autodream_memories (content, embedding, source_mission_id) VALUES ($1, $2, $3) ON CONFLICT (source_mission_id) DO NOTHING"
		if adm.dbProvider.IsSQLite() {
			insertQuery = "INSERT INTO autodream_memories (id, content, embedding, source_mission_id) VALUES (hex(randomblob(16)), ?, ?, ?) ON CONFLICT (source_mission_id) DO NOTHING"
		}

		_, err := adm.dbProvider.Exec(ctx, insertQuery, task.Content, embeddingBytes, task.ID)
		if err != nil {
			slog.Warn("AutoDream failed to store memory", "error", err)
		}
	}
}

func (adm *AutoDreamManager) generateEmbedding(ctx context.Context, text string) []byte {
	// If provider exists and has generate functionality, we'd use it.
	// For OHC-SIP, we serialize a dummy vector if missing real provider to maintain structure.

	vector := make([]float32, 1536) // OpenAI style default size

	// Create some variation based on text length to avoid completely empty vectors
	if len(text) > 0 {
		vector[0] = float32(len(text)) / 1000.0
	}

	// In a real system, we'd call adm.provider.GenerateEmbedding(text)

	b, _ := json.Marshal(vector)

	// If it's pgvector, we need it as a string for cast e.g. '[0.1, 0.2, ...]'
	// The DB provider handles string casting to vector(1536) if it's setup right,
	// or we just save it as text/blob
	return b
}
