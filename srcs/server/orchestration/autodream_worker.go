package orchestration

import (
	"context"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type Database interface {
	ExecContext(ctx context.Context, query string, args ...any) error
	IsSQLite() bool
}

type AutoDreamWorker struct {
	db        Database
	memDir    string
	embedder  func(string) (string, error)
	processed metric.Int64Counter
}

func NewAutoDreamWorker(db Database, memDir string, embedder func(string) (string, error)) *AutoDreamWorker {
	meter := otel.Meter("autodream_worker")
	processed, _ := meter.Int64Counter("autodream.memories.processed")
	if memDir == "" { memDir = ".agent-task/memory" }
	return &AutoDreamWorker{db: db, memDir: memDir, embedder: embedder, processed: processed}
}

func (w *AutoDreamWorker) RunSweep(ctx context.Context) error {
	if _, err := os.Stat(w.memDir); os.IsNotExist(err) { return nil }
	return filepath.WalkDir(w.memDir, func(path string, d fs.DirEntry, err error) error {
		if err != nil || d.IsDir() || !strings.HasSuffix(path, ".yml") { return nil }

		contentBytes, err := os.ReadFile(path)
		if err != nil {
			return nil
		}

		content := string(contentBytes)
		embedding, err := w.embedder(content)
		if err != nil {
			return nil
		}

		orgID := "system"

		var query string
		if w.db.IsSQLite() {
			query = `INSERT INTO agent_memories (organization_id, content, embedding) VALUES ($1, $2, $3)`
		} else {
			query = `INSERT INTO agent_memories (organization_id, content, embedding) VALUES ($1, $2, $3::vector)`
		}

		err = w.db.ExecContext(ctx, query, orgID, content, embedding)
		if err != nil {
			return nil
		}

		w.processed.Add(ctx, 1)
		return os.Remove(path)
	})
}
