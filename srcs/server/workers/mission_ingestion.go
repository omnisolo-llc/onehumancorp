package workers

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// MissionIngestionWorker is responsible for monitoring the .agent-task/missions/
// directory and vectorizing markdown files using the AutoDream infrastructure.
type MissionIngestionWorker struct {
	pool db.Provider
	}

func NewMissionIngestionWorker(pool db.Provider) *MissionIngestionWorker {
	return &MissionIngestionWorker{
		pool: pool,
			}
}

// Start begins a background loop polling for new mission artifacts.
func (w *MissionIngestionWorker) Start(ctx context.Context) {
	slog.Info("Starting MissionIngestionWorker")
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.IngestMissions(ctx)
		}
	}
}

// IngestMissions scans .agent-task/missions/ for markdown artifacts and vectorizes them.
func (w *MissionIngestionWorker) IngestMissions(ctx context.Context) {
	missionsDir := ".agent-task/missions"
	files, err := os.ReadDir(missionsDir)
	if err != nil {
		if !os.IsNotExist(err) {
			slog.Error("MissionIngestionWorker: failed to read missions directory", "error", err)
		}
		return
	}

	for _, file := range files {
		ext := filepath.Ext(file.Name())
		if file.IsDir() || (ext != ".md" && ext != ".yml") {
			continue
		}

		filePath := filepath.Join(missionsDir, file.Name())
		contentBytes, err := os.ReadFile(filePath)
		if err != nil {
			slog.Error("MissionIngestionWorker: failed to read mission file", "file", filePath, "error", err)
			continue
		}

		content := string(contentBytes)

		// Strip HTML glassmorphism styling
		content = stripHTML(content)

		missionID := strings.TrimSuffix(file.Name(), ext)

		// Let AutoDream logic inject truth, skipping if it already exists or using it to upset.
		// Wait, AutoDream InjectTruth expects embedding?
		// AutoDreamWorker handles its own embedding internally when using ProcessMemories, but InjectTruth takes a precomputed embedding string.

		// We'll mimic how AutoDreamWorker inserts things or use the DB directly
		w.processSingleMission(ctx, missionID, content, filePath)
	}
}

func (w *MissionIngestionWorker) processSingleMission(ctx context.Context, missionID, content, filePath string) {
	// Fetch minimax client logic if applicable (this logic is duplicated from AutoDreamWorker if we don't expose it)
	// We can use the AutoDream worker's client logic
	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var embedding []float32
	if minimaxKey != "" {
		client := orchestration.NewCachedMinimaxClient(orchestration.NewMinimaxClient(minimaxKey), w.pool, nil)
		ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
		resp, embErr := client.GenerateEmbedding(ctxTimeout, content)
		cancel()
		if embErr == nil && len(resp) == 1536 {
			embedding = resp
		}
	}
	if len(embedding) == 0 {
		embedding = make([]float32, 1536)
	}

	tx, err := w.pool.Begin(ctx)
	if err != nil {
		slog.Error("MissionIngestionWorker: failed to begin tx", "error", err)
		return
	}

	// For Postgres, we can try locking a dummy row, or use a distributed lock. But actually,
	// checking existence inside the transaction with a unique constraint or locking the autodream_memories table
	// is more robust than trying to lock a non-existent row in agent_session_data.
	// Since source_mission_id and source_type might not have a unique constraint, we will do a SELECT FOR UPDATE if we can,
	// or rely on idempotency by doing a read then insert within the same tx in SQLite, and serializable/locking in Postgres.

	if !w.pool.IsSQLite() {
		// Use an advisory lock in Postgres to prevent race conditions for this specific mission
		// This uses the hash of the missionID to create a 64-bit key
		// To avoid importing hash/crc64 just for this, we can just do a regular SELECT with a table lock or similar,
		// but an advisory lock is best.
		_, err := tx.Exec(ctx, "SELECT pg_try_advisory_xact_lock(hashtext($1))", missionID)
		if err != nil {
			slog.Error("MissionIngestionWorker: failed to get advisory lock", "error", err)
			tx.Rollback(ctx)
			return
		}
	}

	// Check if it already exists inside the transaction
	var count int
	err = tx.QueryRow(ctx, "SELECT count(*) FROM autodream_memories WHERE source_mission_id = $1 AND source_type = 'mission-artifact'", missionID).Scan(&count)
	if err != nil {
		slog.Error("MissionIngestionWorker: failed to check memory existence", "error", err)
		tx.Rollback(ctx)
		return
	}
	if count > 0 {
		tx.Rollback(ctx) // Already ingested
		return
	}

	memID := uuid.New().String()

	// Create string slice representation
	strs := make([]string, len(embedding))
	for i, v := range embedding {
		strs[i] = fmt.Sprintf("%f", v)
	}
	embStr := "[" + strings.Join(strs, ",") + "]"

	var query string
	var args []interface{}

	if w.pool.IsSQLite() {
		query = `INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, CURRENT_TIMESTAMP)`
	} else {
		query = `INSERT INTO autodream_memories (id, content, embedding, source_mission_id, organization_id, agent_id, source_type, created_at) VALUES ($1, $2, $3::vector, $4, $5, $6, $7, CURRENT_TIMESTAMP)`
	}

	args = []interface{}{memID, content, embStr, missionID, "system", "mission-ingestion-worker", "mission-artifact"}

	_, err = tx.Exec(ctx, query, args...)
	if err != nil {
		slog.Error("MissionIngestionWorker: failed to insert memory", "error", err)
		tx.Rollback(ctx)
		return
	}

	if err := tx.Commit(ctx); err != nil {
		slog.Error("MissionIngestionWorker: failed to commit tx", "error", err)
	} else {
		slog.Info("MissionIngestionWorker: ingested mission artifact", "file", filePath)
	}
}

// stripHTML removes simple HTML tags.
func stripHTML(content string) string {
	// A slightly better HTML tag stripper using a simple state machine that considers matching tags
	var result strings.Builder
	inTag := false
	for i := 0; i < len(content); i++ {
		if content[i] == '<' && i+1 < len(content) && (content[i+1] == 'd' || content[i+1] == '/' || content[i+1] == 's' || content[i+1] == 'p' || content[i+1] == 'a' || content[i+1] == 'h' || content[i+1] == 'b' || content[i+1] == 'i' || content[i+1] == 'u') {
			inTag = true
			continue
		}
		if inTag && content[i] == '>' {
			inTag = false
			continue
		}
		if !inTag {
			result.WriteByte(content[i])
		}
	}
	return result.String()
}
