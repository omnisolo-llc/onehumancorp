package memory

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"gopkg.in/yaml.v3"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type AutoDreamDaemon struct {
	db   db.Provider
	done chan struct{}
	once sync.Once
}

func NewAutoDreamDaemon(provider db.Provider) *AutoDreamDaemon {
	return &AutoDreamDaemon{
		db:   provider,
		done: make(chan struct{}),
	}
}

func (d *AutoDreamDaemon) Start(ctx context.Context) {
	slog.Info("Starting AutoDreamDaemon")
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			d.Stop()
			return
		case <-d.done:
			return
		case <-ticker.C:
			d.processDirectory(ctx, ".agent-task/memory/")
			d.processDirectory(ctx, ".agent-task/missions/")
		}
	}
}

func (d *AutoDreamDaemon) Stop() {
	d.once.Do(func() {
		close(d.done)
	})
}

func (d *AutoDreamDaemon) processDirectory(ctx context.Context, dir string) {
	files, err := os.ReadDir(dir)
	if err != nil {
		if !os.IsNotExist(err) {
			slog.Error("AutoDreamDaemon: failed to read directory", "dir", dir, "error", err)
		}
		return
	}

	for _, file := range files {
		if file.IsDir() || !strings.HasSuffix(file.Name(), ".yml") {
			continue
		}

		path := filepath.Join(dir, file.Name())
		data, err := os.ReadFile(path)
		if err != nil {
			slog.Error("AutoDreamDaemon: failed to read file", "file", path, "error", err)
			continue
		}

		var memFile struct {
			Status  string `yaml:"status"`
			Content string `yaml:"content"`
		}

		if err := yaml.Unmarshal(data, &memFile); err != nil {
			slog.Error("AutoDreamDaemon: failed to unmarshal memory file", "file", path, "error", err)
			continue
		}

		if memFile.Status != "DONE" {
			continue
		}

		// Mock embedding generation
		embedding := make([]float32, 1536)
		embedding[0] = 0.1

		var embStr string
		if d.db.IsSQLite() {
			embBytes, _ := json.Marshal(embedding)
			embStr = string(embBytes)
		} else {
			strs := make([]string, len(embedding))
			for i, v := range embedding {
				strs[i] = fmt.Sprintf("%f", v)
			}
			embStr = "[" + strings.Join(strs, ",") + "]"
		}

		memID := uuid.New().String()
		tx, err := d.db.Begin(ctx)
		if err != nil {
			slog.Error("AutoDreamDaemon: failed to begin tx", "error", err)
			continue
		}

		var insertQuery string
		if d.db.IsSQLite() {
			insertQuery = `INSERT INTO memory_embeddings (id, content, vector_embedding) VALUES (?, ?, ?)`
		} else {
			insertQuery = `INSERT INTO memory_embeddings (id, content, vector_embedding) VALUES ($1, $2, $3::vector)`
		}

		_, err = tx.Exec(ctx, insertQuery, memID, memFile.Content, embStr)
		if err != nil {
			slog.Error("AutoDreamDaemon: failed to insert memory embedding", "error", err)
			tx.Rollback(ctx)
			continue
		}

		if err := tx.Commit(ctx); err != nil {
			slog.Error("AutoDreamDaemon: failed to commit tx", "error", err)
		} else {
			slog.Debug("AutoDreamDaemon: processed memory file", "file", path)
			telemetry.RecordAutoDreamProcessedMemory(ctx, 1)
			os.Remove(path) // Remove processed file
		}
	}
}
