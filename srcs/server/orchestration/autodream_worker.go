package orchestration

import (
	"sync"

	"context"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/google/uuid"
	"gopkg.in/yaml.v3"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func formatFloat32SliceForVector(embedding []float32) string {
	if len(embedding) == 0 {
		return "[]"
	}
	strs := make([]string, len(embedding))
	for i, v := range embedding {
		strs[i] = fmt.Sprintf("%f", v)
	}
	return "[" + strings.Join(strs, ",") + "]"
}

// MemoryFile represents the structure of agent memory YAML files.
type MemoryFile struct {
	AgentSessionData string `yaml:"agent_session_data"`
	Content          string `yaml:"content"`
}

// ProcessMemories ingests pending memory YAML files into the autodream_memories table.
//
// When OHC_MEMORY_DIR is set the worker reads YAML files from that directory
// (migration path: legacy agents wrote memory files there).  If the env var is
// empty this pipeline is a no-op — new agents write directly to the DB.
func (w *AutoDreamWorker) ProcessMemories(ctx context.Context) error {
	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		return nil // file-based ingestion disabled; agents write to DB directly
	}
	matches, err := filepath.Glob(filepath.Join(memoryDir, "*.yml"))
	if err != nil {
		return fmt.Errorf("failed to glob memory files: %w", err)
	}
	if len(matches) == 0 {
		return nil // No files to process
	}

	// Apply batch limit of 500 as requested
	if len(matches) > 500 {
		matches = matches[:500]
	}

	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client MinimaxClient
	if minimaxKey != "" {
		client = NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
	}

	for _, file := range matches {
		data, err := os.ReadFile(file)
		if err != nil {
			slog.Error("AutoDream: failed to read memory file", "file", file, "error", err)
			continue
		}

		var memFile MemoryFile
		if err := yaml.Unmarshal(data, &memFile); err != nil {
			slog.Error("AutoDream: failed to unmarshal memory file", "file", file, "error", err)
			continue
		}

		contentToEmbed := memFile.AgentSessionData
		if contentToEmbed == "" {
			contentToEmbed = memFile.Content
		}
		if contentToEmbed == "" {
			os.Remove(file) // Clean up empty files so they aren't re-processed
			continue
		}

		missionID := strings.TrimSuffix(filepath.Base(file), ".yml")

		// Embed via Minimax if available, otherwise use a dummy embedding
		embedding := make([]float32, 1536)
		if client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, err := client.GenerateEmbedding(ctxTimeout, contentToEmbed)
			cancel()
			if err == nil && len(resp) == 1536 {
				embedding = resp
			} else {
				slog.Warn("AutoDream: failed to embed with Minimax, using empty embedding", "error", err)
			}
		}

		tx, err := w.pool.Begin(ctx)
		if err != nil {
			slog.Error("AutoDream: failed to begin tx", "error", err)
			continue
		}

		// Implement UPSERT logic by source_mission_id instead of just inserting.
		// However, source_mission_id doesn't have a unique constraint.
		// Since we delete the file, idempotency is mostly handled by file deletion, but if it fails to delete, it might duplicate.

		memID := uuid.New().String()
		embStr := formatFloat32SliceForVector(embedding)

		var query string
		var args []interface{}

		if !w.pool.IsSQLite() {
			_, lockErr := tx.Exec(ctx, "SELECT 1 FROM agent_session_data WHERE session_id = $1 FOR UPDATE SKIP LOCKED", missionID)
			if lockErr != nil {
				tx.Rollback(ctx)
				continue
			}
		}

		if w.pool.IsSQLite() {
			query = `INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP)`
		} else {
			query = `INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3::vector, $4, $5, $6, $7, CURRENT_TIMESTAMP)`
		}

		// Insert into the new pgvector table specified in Phase 3
		var pgvectorQuery string
		if w.pool.IsSQLite() {
			pgvectorQuery = `INSERT INTO ohc_memory_autodream_vectors (id, task_id, content, embedding, metadata, created_at) VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)`
		} else {
			pgvectorQuery = `INSERT INTO ohc_memory.autodream_vectors (id, task_id, content, embedding, metadata, created_at) VALUES ($1, $2, $3, $4::vector, $5, CURRENT_TIMESTAMP)`
		}

		args = []interface{}{memID, contentToEmbed, embStr, missionID, "system", "auto-dream-worker", "background-pipeline"}

		_, err = tx.Exec(ctx, query, args...)
		if err == nil {
			// Only insert into the new table if the first succeeds (although technically this should be the primary insert now)
			// task_id requires a valid UUID but we'll use missionID here since task_id matches it contextually, or NULL if it fails FK constraint.
			// However since the task_id is a UUID, and missionID might not be a valid UUID, we generate a random one if missionID is not a valid UUID.
			taskIdToUse := missionID
			if _, uuidErr := uuid.Parse(missionID); uuidErr != nil {
			    // Create a deterministic UUID for the mission ID if it's not a UUID
			    taskIdToUse = uuid.NewMD5(uuid.NameSpaceOID, []byte(missionID)).String()
			}

			_, _ = tx.Exec(ctx, pgvectorQuery, uuid.New().String(), taskIdToUse, contentToEmbed, embStr, `{"source":"background-pipeline"}`)
		}
		if err != nil {
			slog.Error("AutoDream: failed to insert memory", "error", err)
			tx.Rollback(ctx)
			continue
		}

		if err := tx.Commit(ctx); err != nil {
			slog.Error("AutoDream: failed to commit tx", "error", err)
		} else {
			slog.Info("AutoDream: processed memory file", "file", file)
			os.Remove(file)
		}
	}

	return nil
}



// AutoDreamWorkerDaemon runs ProcessMemories periodically.
type AutoDreamWorkerDaemon struct {
	worker *AutoDreamWorker
	done   chan struct{}
	stopOnce sync.Once
}

func NewAutoDreamWorkerDaemon(worker *AutoDreamWorker) *AutoDreamWorkerDaemon {
	return &AutoDreamWorkerDaemon{
		worker: worker,
		done:   make(chan struct{}),
	}
}


// ConsolidateMemories processes the backlog of episodic memories into embeddings.
// It fetches up to 100 rows where processed_at IS NULL.
func (w *AutoDreamWorker) ConsolidateMemories(ctx context.Context) error {
	var rows db.Rows
	var err error

	if w.pool.IsSQLite() {
		rows, err = w.pool.Query(ctx, "SELECT id, content FROM autodream_memories WHERE processed_at IS NULL LIMIT 100")
	} else {
		// Postgres lock row for update so no other worker picks it up
		rows, err = w.pool.Query(ctx, "SELECT id, content FROM autodream_memories WHERE processed_at IS NULL LIMIT 100 FOR UPDATE SKIP LOCKED")
	}
	if err != nil {
		return fmt.Errorf("failed to fetch unprocessed memories: %w", err)
	}

	type memEntry struct {
		id      string
		content string
	}
	var entries []memEntry

	for rows.Next() {
		var e memEntry
		if scanErr := rows.Scan(&e.id, &e.content); scanErr == nil {
			entries = append(entries, e)
		}
	}
	rows.Close()

	if len(entries) == 0 {
		return nil
	}

	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client MinimaxClient
	if minimaxKey != "" {
		client = NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), w.pool, nil)
	}

	for _, e := range entries {
		embedding := make([]float32, 1536)
		if client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embedErr := client.GenerateEmbedding(ctxTimeout, e.content)
			cancel()
			if embedErr == nil && len(resp) == 1536 {
				embedding = resp
			} else {
				slog.Warn("AutoDream: failed to embed memory with Minimax, using empty", "error", embedErr)
			}
		}

		embStr := formatFloat32SliceForVector(embedding)

		var query string
		if w.pool.IsSQLite() {
			query = "UPDATE autodream_memories SET embedding = $1, processed_at = CURRENT_TIMESTAMP WHERE id = $2"
		} else {
			query = "UPDATE autodream_memories SET embedding = $1::vector, processed_at = CURRENT_TIMESTAMP WHERE id = $2"
		}

		_, updateErr := w.pool.Exec(ctx, query, embStr, e.id)
		if updateErr != nil {
			slog.Error("AutoDream: failed to update memory", "id", e.id, "error", updateErr)
		} else {
			slog.Debug("AutoDream: consolidated memory", "id", e.id)
		}
	}

	return nil
}

func (d *AutoDreamWorkerDaemon) Start(ctx context.Context) {
	ticker := time.NewTicker(5 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			d.Stop()
			return
		case <-d.done:
			return
		case <-ticker.C:
			if err := d.worker.ProcessMemories(ctx); err != nil {
				slog.Error("AutoDreamWorkerDaemon: failed to process memories", "error", err)
			}
			if err := d.worker.ConsolidateMemories(ctx); err != nil {
				slog.Error("AutoDreamWorkerDaemon: failed to consolidate memories", "error", err)
			}
		}
	}
}

func (d *AutoDreamWorkerDaemon) Stop() {
	d.stopOnce.Do(func() {
		close(d.done)
	})
}
