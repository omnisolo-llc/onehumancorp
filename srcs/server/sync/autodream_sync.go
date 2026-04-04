package sync

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type AutoDreamPayload struct {
	Type     string `json:"type"` // "embedding" or "mission"
	ID       string `json:"id"`
	Data     string `json:"data"`
	Metadata string `json:"metadata"`
}

type AutoDreamSyncEngine struct {
	dbWrapper   *db.DB
	ticker      *time.Ticker
	quit        chan struct{}
	cloudAPIURL string
}

func NewAutoDreamSyncEngine(dbWrapper *db.DB, pollInterval time.Duration, cloudAPIURL string) *AutoDreamSyncEngine {
	return &AutoDreamSyncEngine{
		dbWrapper:   dbWrapper,
		ticker:      time.NewTicker(pollInterval),
		quit:        make(chan struct{}),
		cloudAPIURL: cloudAPIURL,
	}
}

func (e *AutoDreamSyncEngine) Start(ctx context.Context) {
	if !e.dbWrapper.IsSQLite() {
		// Only run sync engine in standalone/SQLite mode
		slog.Debug("sync: AutoDreamSyncEngine disabled (not in standalone SQLite mode)")
		return
	}

	go func() {
		for {
			select {
			case <-e.ticker.C:
				e.ProcessForecastTick(ctx)
			case <-e.quit:
				e.ticker.Stop()
				return
			case <-ctx.Done():
				e.ticker.Stop()
				return
			}
		}
	}()
}

func (e *AutoDreamSyncEngine) Stop() {
	close(e.quit)
}

// ProcessForecastTick is used synchronously for tests and run via the ticker loop in production.
func (e *AutoDreamSyncEngine) ProcessForecastTick(ctx context.Context) {
	if !e.dbWrapper.IsSQLite() {
		return
	}

	// 1. Synthesize memory for AutoDream vector cache (runs the background epoch cluster)
	e.synthesizeMemory(ctx)

	// 2. Sync Embedding Cache
	e.syncEmbeddingCache(ctx)

	// 3. Sync Agent Missions
	e.syncAgentMissions(ctx)
}

func (e *AutoDreamSyncEngine) synthesizeMemory(ctx context.Context) {
	// Periodic background worker that processes DONE tasks and synthesizes them into long-term vector embeddings.
	// As this is a sync engine, it should only do this if it's standalone, which we verified above.
	// We'll delegate to the AutoDreamWorker to cluster an epoch if there's no worker active.
	// AutoDream Worker is usually started in main(), but here we ensure periodic sync synthesis is called.

	// A simple call to gather DONE agent missions that aren't synced, and generate a memory.
	// For now, let's keep it simple: we trigger a "synthesize" query.
	// Since full vector ops aren't in SQLite, we just update local cache.
	rows, err := e.dbWrapper.Query(ctx, "SELECT id, payload FROM agent_missions WHERE status = 'DONE' AND synced_to_cloud = false LIMIT 10")
	if err != nil {
		return
	}
	defer rows.Close()

	var doneMissions []string
	for rows.Next() {
		var id, payload string
		if err := rows.Scan(&id, &payload); err == nil {
			doneMissions = append(doneMissions, fmt.Sprintf("ID: %s, Payload: %s", id, payload))
		}
	}

	if len(doneMissions) > 0 {
		slog.Debug("sync: autoDream memory synthesis triggered for pending DONE missions", "count", len(doneMissions))
		// In a full implementation, we'd call MinimaxClient to create an embedding and insert it into swarm_truth_embeddings.
	}
}

func (e *AutoDreamSyncEngine) syncEmbeddingCache(ctx context.Context) {
	rows, err := e.dbWrapper.Query(ctx, "SELECT content_hash, embedding FROM embedding_cache WHERE synced_to_cloud = false LIMIT 50")
	if err != nil {
		slog.Error("sync: failed to query embedding_cache", "error", err)
		return
	}
	defer rows.Close()

	var payloads []AutoDreamPayload
	var hashes []string

	for rows.Next() {
		var hash, embedding string
		if err := rows.Scan(&hash, &embedding); err != nil {
			slog.Error("sync: failed to scan embedding_cache", "error", err)
			continue
		}

		payloads = append(payloads, AutoDreamPayload{
			Type:     "embedding",
			ID:       hash,
			Data:     embedding,
			Metadata: "",
		})
		hashes = append(hashes, hash)
	}

	if len(payloads) == 0 {
		return
	}

	if err := e.sendToCloud(ctx, payloads); err != nil {
		if telemetry.SyncFailedCount != nil {
			telemetry.SyncFailedCount.Add(ctx, int64(len(payloads)))
		}
		slog.Error("sync: failed to send embedding_cache to cloud", "error", err)
		return
	}

	// Mark as synced
	for _, h := range hashes {
		_, err := e.dbWrapper.Exec(ctx, "UPDATE embedding_cache SET synced_to_cloud = true WHERE content_hash = $1", h)
		if err != nil {
			slog.Error("sync: failed to update embedding_cache status", "hash", h, "error", err)
		}
	}

	if telemetry.SyncCompletedCount != nil {
		telemetry.SyncCompletedCount.Add(ctx, int64(len(payloads)))
	}
	slog.Debug("sync: successfully synced embeddings", "count", len(payloads))
}

func (e *AutoDreamSyncEngine) syncAgentMissions(ctx context.Context) {
	rows, err := e.dbWrapper.Query(ctx, "SELECT id, status, payload FROM agent_missions WHERE synced_to_cloud = false LIMIT 50")
	if err != nil {
		slog.Error("sync: failed to query agent_missions", "error", err)
		return
	}
	defer rows.Close()

	var payloads []AutoDreamPayload
	var ids []string

	for rows.Next() {
		var id, status, payloadData string
		if err := rows.Scan(&id, &status, &payloadData); err != nil {
			slog.Error("sync: failed to scan agent_missions", "error", err)
			continue
		}

		var parsedPayload map[string]interface{}
		if err := json.Unmarshal([]byte(payloadData), &parsedPayload); err == nil {
			parsedIface := telemetry.RedactInterfacePII(parsedPayload)
			if redactedBytes, err := json.Marshal(parsedIface); err == nil {
				payloadData = string(redactedBytes)
			}
		}

		payloads = append(payloads, AutoDreamPayload{
			Type:     "mission",
			ID:       id,
			Data:     payloadData,
			Metadata: status,
		})
		ids = append(ids, id)
	}

	if len(payloads) == 0 {
		return
	}

	if err := e.sendToCloud(ctx, payloads); err != nil {
		if telemetry.SyncFailedCount != nil {
			telemetry.SyncFailedCount.Add(ctx, int64(len(payloads)))
		}
		slog.Error("sync: failed to send agent_missions to cloud", "error", err)
		return
	}

	// Mark as synced
	for _, id := range ids {
		_, err := e.dbWrapper.Exec(ctx, "UPDATE agent_missions SET synced_to_cloud = true WHERE id = $1", id)
		if err != nil {
			slog.Error("sync: failed to update agent_missions status", "id", id, "error", err)
		}
	}

	if telemetry.SyncCompletedCount != nil {
		telemetry.SyncCompletedCount.Add(ctx, int64(len(payloads)))
	}
	slog.Debug("sync: successfully synced agent_missions", "count", len(payloads))
}

func (e *AutoDreamSyncEngine) sendToCloud(ctx context.Context, payloads []AutoDreamPayload) error {
	// If cloudAPIURL is empty (e.g. testing), mock success
	if e.cloudAPIURL == "" {
		return nil
	}

	jsonData, err := json.Marshal(payloads)
	if err != nil {
		return fmt.Errorf("marshal payloads: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, e.cloudAPIURL, bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	// Set SPIFFE authentication token header if identity token is provided in environment variables.
	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("do request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 300 {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("unexpected status %d: %s", resp.StatusCode, string(body))
	}

	return nil
}
