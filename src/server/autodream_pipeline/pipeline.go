package autodream_pipeline

import (
	"database/sql"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"time"

	"github.com/google/uuid"
)

type EmbeddingApi interface {
	GenerateEmbedding(text string) (string, error)
}

type DB interface {
	IsSQLite() bool
	Exec(query string, args ...any) (sql.Result, error)
}

type AutoDreamWorker struct {
	db     DB
	api    EmbeddingApi
	memDir string
}

func NewAutoDreamWorker(db DB, api EmbeddingApi, memDir string) *AutoDreamWorker {
	if memDir == "" {
		memDir = ".agent-task/memory"
	}
	return &AutoDreamWorker{
		db:     db,
		api:    api,
		memDir: memDir,
	}
}

func (w *AutoDreamWorker) SweepAndConsolidate() error {
	if _, err := os.Stat(w.memDir); os.IsNotExist(err) {
		return nil
	}

	err := filepath.WalkDir(w.memDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			return nil
		}
		if filepath.Ext(path) != ".yml" {
			return nil
		}

		contentBytes, err := os.ReadFile(path)
		if err != nil {
			return err
		}
		content := string(contentBytes)

		embedding, err := w.api.GenerateEmbedding(content)
		if err != nil {
			return fmt.Errorf("failed to generate embedding for %s: %w", path, err)
		}

		memID := uuid.New().String()

		if w.db.IsSQLite() {
			query := `INSERT INTO autodream_memories
				(id, organization_id, agent_id, task_id, content, embedding, source_type)
				VALUES (?, ?, ?, ?, ?, ?, ?)`
			_, err = w.db.Exec(query, memID, "system", "system_agent", "agent-task", content, embedding, "TASK_SUMMARY")
		} else {
			query := `INSERT INTO autodream_memories
				(id, organization_id, agent_id, task_id, content, embedding, source_type)
				VALUES ($1, $2, $3, $4, $5, $6::vector, $7)`
			_, err = w.db.Exec(query, memID, "system", "system_agent", "agent-task", content, embedding, "TASK_SUMMARY")
		}

		if err != nil {
			return fmt.Errorf("failed to insert memory for %s: %w", path, err)
		}

		return os.Remove(path)
	})

	return err
}

func (w *AutoDreamWorker) StartDaemon(interval time.Duration) {
	go func() {
		for {
			_ = w.SweepAndConsolidate()
			time.Sleep(interval)
		}
	}()
}
