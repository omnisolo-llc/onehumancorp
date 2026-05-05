package orchestration

import (
	"database/sql"
	"io/fs"
	"log"
	"os"
	"path/filepath"
	"time"
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
			log.Printf("autodream daemon: Error accessing path %s: %v", path, err)
			return nil // continue
		}
		if d.IsDir() {
			return nil
		}
		if filepath.Ext(path) != ".yml" && filepath.Ext(path) != ".yaml" {
			return nil
		}

		contentBytes, err := os.ReadFile(path)
		if err != nil {
			log.Printf("autodream daemon: Error reading file %s: %v", path, err)
			return nil // continue
		}
		content := string(contentBytes)

		embedding, err := w.api.GenerateEmbedding(content)
		if err != nil {
			log.Printf("autodream daemon: Error generating embedding for %s: %v", path, err)
			return nil // continue
		}

		orgID := "system"

		if w.db.IsSQLite() {
			query := `INSERT INTO autodream_memories
				(organization_id, content, embedding)
				VALUES (?, ?, ?)`
			_, err = w.db.Exec(query, orgID, content, embedding)
		} else {
			query := `INSERT INTO autodream_memories
				(organization_id, content, embedding)
				VALUES ($1, $2, $3::vector)`
			_, err = w.db.Exec(query, orgID, content, embedding)
		}

		if err != nil {
			log.Printf("autodream daemon: Error inserting memory for %s: %v", path, err)
			return nil // continue
		}

		err = os.Remove(path)
		if err != nil {
			log.Printf("autodream daemon: Error removing file %s: %v", path, err)
		}
		return nil
	})

	return err
}

func (w *AutoDreamWorker) StartDaemon(interval time.Duration) {
	go func() {
		for {
			err := w.SweepAndConsolidate()
			if err != nil {
				log.Printf("autodream daemon sweep error: %v", err)
			}
			time.Sleep(interval)
		}
	}()
}
