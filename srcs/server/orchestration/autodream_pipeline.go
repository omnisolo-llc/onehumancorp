package orchestration

import (
	"context"
	"fmt"
	"encoding/json"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
	"time"

	"gopkg.in/yaml.v3"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
	"net/http"
	"bytes"
)

// EmbeddingClient interface for dependency injection and testing
type EmbeddingClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// AutoDreamPipeline is the daemon process for long-term memory consolidation
type AutoDreamPipeline struct {
	db     db.Provider
	client EmbeddingClient
	done   chan struct{}
}

// NewAutoDreamPipeline creates a new pipeline instance
func NewAutoDreamPipeline(provider db.Provider) *AutoDreamPipeline {
	var client EmbeddingClient
	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	if minimaxKey != "" {
		client = NewCachedMinimaxClient(NewMinimaxClient(minimaxKey), provider, nil)
	}

	return &AutoDreamPipeline{
		db:     provider,
		client: client,
		done:   make(chan struct{}),
	}
}

// Start begins the background pipeline process
func (p *AutoDreamPipeline) Start(ctx context.Context) {
	ticker := time.NewTicker(5 * time.Minute) // run periodically
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			p.Stop()
			return
		case <-p.done:
			return
		case <-ticker.C:
			p.process(context.Background())
		}
	}
}

// Stop halts the pipeline
func (p *AutoDreamPipeline) Stop() {
	close(p.done)
}

// process performs a sweep to consolidate ephemeral memories from the DB.
// File-based ingestion via OHC_MEMORY_DIR is handled separately by
// AutoDreamWorker.ingestAgentMemories.
func (p *AutoDreamPipeline) process(ctx context.Context) {
	slog.Info("AutoDreamPipeline: starting memory consolidation sweep")

	memoryDir := os.Getenv("OHC_MEMORY_DIR")
	if memoryDir == "" {
		return // DB-backed memory only; no file processing
	}
	matches, err := filepath.Glob(filepath.Join(memoryDir, "*.yml"))
	if err != nil {
		slog.Error("AutoDreamPipeline: failed to glob memory files", "error", err)
		return
	}
	if len(matches) == 0 {
		return // nothing to process
	}

	limit := 500
	if len(matches) > limit {
		matches = matches[:limit]
	}

	for _, file := range matches {
		data, err := os.ReadFile(file)
		if err != nil {
			slog.Error("AutoDreamPipeline: failed to read memory file", "file", file, "error", err)
			continue
		}

		var memFile struct {
			AgentSessionData string `yaml:"agent_session_data"`
			Content          string `yaml:"content"`
		}

		if err := yaml.Unmarshal(data, &memFile); err != nil {
			slog.Error("AutoDreamPipeline: failed to unmarshal memory file", "file", file, "error", err)
			continue
		}

		contentToEmbed := memFile.AgentSessionData
		if contentToEmbed == "" {
			contentToEmbed = memFile.Content
		}
		if contentToEmbed == "" {
			os.Remove(file)
			continue
		}

		missionID := strings.TrimSuffix(filepath.Base(file), ".yml")

		embeddingStr := "[0.0, 0.0, 0.0]"
		if p.client != nil {
			ctxTimeout, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, err := p.client.GenerateEmbedding(ctxTimeout, contentToEmbed)
			cancel()
			if err == nil && len(resp) > 0 {
				if bytes, err := json.Marshal(resp); err == nil {
					embeddingStr = string(bytes)
				}
			} else if err != nil {
				slog.Warn("AutoDreamPipeline: failed to generate embedding", "error", err)
			}
		}

		var insertQuery string
		var insertArgs []interface{}

		memID := missionID

		isDuplicate := false
		if !p.db.IsSQLite() && embeddingStr != "[0.0, 0.0, 0.0]" {
			// Check for cosine similarity distance <= 0.1
			var distance float64
			query := `SELECT embedding <=> $1::vector FROM autodream_memories ORDER BY embedding <=> $1::vector LIMIT 1`
			err := p.db.QueryRow(ctx, query, embeddingStr).Scan(&distance)
			if err == nil && distance <= 0.1 {
				isDuplicate = true
				slog.Debug("AutoDreamPipeline: skipping duplicate memory", "id", memID, "distance", distance)
			}
		}

		if isDuplicate {
			os.Remove(file)
			continue
		}

		if p.db.IsSQLite() {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, created_at)
				VALUES (?, 'system', 'auto-dream-pipeline', ?, ?, 'memory_file', CURRENT_TIMESTAMP)
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
			`
			insertArgs = []interface{}{memID, contentToEmbed, embeddingStr}
		} else {
			insertQuery = `
				INSERT INTO autodream_memories (id, organization_id, agent_id, content, embedding, source_type, created_at)
				VALUES ($1, 'system', 'auto-dream-pipeline', $2, $3::vector, 'memory_file', NOW())
				ON CONFLICT(id) DO UPDATE SET content=EXCLUDED.content, embedding=EXCLUDED.embedding
			`
			insertArgs = []interface{}{memID, contentToEmbed, embeddingStr}
		}

		if _, err := p.db.Exec(ctx, insertQuery, insertArgs...); err != nil {
			slog.Warn("AutoDreamPipeline: failed to insert memory", "id", memID, "error", err)
		} else {
			slog.Debug("AutoDreamPipeline: consolidated memory", "id", memID)
			os.Remove(file)
			if telemetry.AutoDreamMemoriesIngestedCounter != nil {
				telemetry.AutoDreamMemoriesIngestedCounter.Add(ctx, 1, metric.WithAttributes(attribute.String("agent_id", "auto-dream-pipeline")))
			}
		}
	}

	slog.Info("AutoDreamPipeline: completed sweep", "processed", len(matches))
}

// Sync pushes local memories to the cloud pgvector store.
func (p *AutoDreamPipeline) Sync(ctx context.Context, cloudEndpoint string) error {
	if cloudEndpoint == "" {
		return nil
	}

	// Fetch un-synced memories (for simplicity, we'll fetch recently created ones or all if not tracked.
	// But let's fetch everything since this is an escalation push, or just recent ones.)
	// The problem doesn't specify which ones, just "export local memories to the cloud endpoint".
	query := "SELECT id, content, embedding FROM autodream_memories LIMIT 100"
	rows, err := p.db.Query(ctx, query)
	if err != nil {
		return err
	}
	defer rows.Close()

	type SyncPayload struct {
		ID        string `json:"id"`
		Content   string `json:"content"`
		Embedding string `json:"embedding"`
	}

	var payloads []SyncPayload
	for rows.Next() {
		var p SyncPayload
		if err := rows.Scan(&p.ID, &p.Content, &p.Embedding); err == nil {
			payloads = append(payloads, p)
		}
	}

	if len(payloads) == 0 {
		return nil
	}

	jsonData, err := json.Marshal(payloads)
	if err != nil {
		return err
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, cloudEndpoint, bytes.NewBuffer(jsonData))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")

	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 300 {
		return fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	return nil
}
