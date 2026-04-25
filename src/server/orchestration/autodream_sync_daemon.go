package orchestration

import (
	"context"
	"encoding/json"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"sync"
	"time"

	"gopkg.in/yaml.v3"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/db"
)

type EmbeddingClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

type AutoDreamSyncDaemon struct {
	db        db.Provider
	client    EmbeddingClient
	done      chan struct{}
	stopOnce  sync.Once
	memoryDir string
}

func NewAutoDreamSyncDaemon(provider db.Provider, client EmbeddingClient) *AutoDreamSyncDaemon {
	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		memoryDir = ".agent-task/memory"
	}
	return &AutoDreamSyncDaemon{
		db:        provider,
		client:    client,
		done:      make(chan struct{}),
		memoryDir: memoryDir,
	}
}

func (d *AutoDreamSyncDaemon) Start(ctx context.Context) {
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
			d.processFiles(ctx)
		}
	}
}

func (d *AutoDreamSyncDaemon) Stop() {
	d.stopOnce.Do(func() {
		close(d.done)
	})
}

func (d *AutoDreamSyncDaemon) processFiles(ctx context.Context) {
	matches, err := filepath.Glob(filepath.Join(d.memoryDir, "*.yml"))
	if err != nil {
		slog.Error("AutoDreamSyncDaemon: failed to glob memory files", "error", err)
		return
	}

	for _, file := range matches {
		data, err := os.ReadFile(file)
		if err != nil {
			slog.Error("AutoDreamSyncDaemon: failed to read memory file", "file", file, "error", err)
			continue
		}

		var memFile map[string]interface{}
		if err := yaml.Unmarshal(data, &memFile); err != nil {
			slog.Error("AutoDreamSyncDaemon: failed to unmarshal memory file", "file", file, "error", err)
			continue
		}

		contentBytes, _ := json.Marshal(memFile)
		content := string(contentBytes)

		var embeddingStr string
		if d.client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			embedding, embedErr := d.client.GenerateEmbedding(ctxTimeout, content)
			cancel()
			if embedErr == nil && len(embedding) > 0 {
				if bytes, err := json.Marshal(embedding); err == nil {
					embeddingStr = string(bytes)
				}
			}
		}

		if embeddingStr == "" {
			var vec []string
			for i := 0; i < 1536; i++ {
				vec = append(vec, "0.0")
			}
			embeddingStr = "[" + strings.Join(vec, ",") + "]"
		}

		id := uuid.New().String()
		topic := "Agent Memory"
		if t, ok := memFile["topic"].(string); ok {
			topic = t
		}

		tenantID := "default_tenant"
		if org, ok := memFile["organization_id"].(string); ok {
		    tenantID = org
		}

		var query string
		if d.db.IsSQLite() {
			query = "INSERT INTO autodream_memories (id, tenant_id, topic, content, embedding) VALUES (?, ?, ?, ?, ?)"
		} else {
			query = "INSERT INTO autodream_memories (id, tenant_id, topic, content, embedding) VALUES ($1, $2, $3, $4, $5::vector)"
		}

		_, err = d.db.Exec(ctx, query, id, tenantID, topic, content, embeddingStr)
		if err != nil {
			slog.Error("AutoDreamSyncDaemon: failed to insert memory into DB", "file", file, "error", err)
			continue
		}

		os.Remove(file)
		slog.Info("AutoDreamSyncDaemon: successfully processed and deleted memory file", "file", file)
	}
}
