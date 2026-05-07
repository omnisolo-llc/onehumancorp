package autodream_pipeline

import (
	"database/sql"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/lib/pricing"
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
	cache  *pricing.LocalEmbeddingCache
}

func NewAutoDreamWorker(db DB, api EmbeddingApi, memDir string, cache *pricing.LocalEmbeddingCache) *AutoDreamWorker {
	if memDir == "" {
		memDir = ".ohc/runtime/memory"
	}
	return &AutoDreamWorker{
		db:     db,
		api:    api,
		memDir: memDir,
		cache:  cache,
	}
}

func (w *AutoDreamWorker) SweepAndConsolidate() error {
	if _, err := os.Stat(w.memDir); os.IsNotExist(err) {
		return nil
	}

	retentionPeriod := 7 * 24 * time.Hour
	now := time.Now()

	err := filepath.WalkDir(w.memDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if d.IsDir() {
			return nil
		}

		info, err := d.Info()
		if err == nil && now.Sub(info.ModTime()) > retentionPeriod {
			_ = os.Remove(path)
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

		var embedding string
		var errEmb error
		var exists bool

		if w.cache != nil {
			embedding, exists = w.cache.Get(content)
		}

		if !exists {
			embedding, errEmb = w.api.GenerateEmbedding(content)
			if errEmb != nil {
				return fmt.Errorf("failed to generate embedding for %s: %w", path, errEmb)
			}
			if w.cache != nil {
				w.cache.Set(content, embedding)
			}
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
