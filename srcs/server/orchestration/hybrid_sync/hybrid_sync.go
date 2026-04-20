package hybrid_sync

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

// HybridSyncDaemon bridges the local SQLite single-user instance
// and the Postgres multi-tenant database for Omni-Context synchronization.
type HybridSyncDaemon struct {
	dbWrapper   *db.DB
	ticker      *time.Ticker
	quit        chan struct{}
	cloudAPIURL string
}

func NewHybridSyncDaemon(dbWrapper *db.DB, pollInterval time.Duration, cloudAPIURL string) *HybridSyncDaemon {
	if cloudAPIURL == "" {
		cloudAPIURL = os.Getenv("OHC_CORE_URL")
	}
	if cloudAPIURL == "" {
		cloudAPIURL = "http://localhost:8080"
	}

	return &HybridSyncDaemon{
		dbWrapper:   dbWrapper,
		ticker:      time.NewTicker(pollInterval),
		quit:        make(chan struct{}),
		cloudAPIURL: cloudAPIURL,
	}
}

func (d *HybridSyncDaemon) Start(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		// Only run in standalone/SQLite mode
		slog.Debug("hybrid_sync: HybridSyncDaemon disabled (not in standalone SQLite mode)")
		return
	}

	go func() {
		for {
			select {
			case <-d.ticker.C:
				d.ProcessSync(ctx)
				d.ProcessCRDTSync(ctx)
			case <-d.quit:
				d.ticker.Stop()
				return
			case <-ctx.Done():
				d.ticker.Stop()
				return
			}
		}
	}()
}

func (d *HybridSyncDaemon) Stop() {
	close(d.quit)
}

type SyncPayload struct {
	MemoryID        string `json:"memory_id"`
	Context         string `json:"context"`
	VectorEmbedding []byte `json:"vector_embedding,omitempty"`
}

func (d *HybridSyncDaemon) ProcessSync(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		return
	}

	tx, err := d.dbWrapper.Begin(ctx)
	if err != nil {
		slog.Error("hybrid_sync: failed to begin transaction", "error", err)
		return
	}
	defer tx.Rollback(ctx)

	// In swarm_memory_embeddings, we look for items where json_extract(context, '$.escalation_required') = 1 or true.
	query := "SELECT memory_id, context FROM swarm_memory_embeddings WHERE json_extract(context, '$.escalation_required') = 1 OR json_extract(context, '$.escalation_required') = 'true' LIMIT 100"

	rows, err := tx.Query(ctx, query)
	if err != nil {
		slog.Error("hybrid_sync: failed to query swarm_memory_embeddings", "error", err)
		return
	}
	defer rows.Close()

	var payloads []SyncPayload
	var ids []string

	for rows.Next() {
		var memoryID, contextData string
		if err := rows.Scan(&memoryID, &contextData); err != nil {
			slog.Error("hybrid_sync: failed to scan swarm_memory_embeddings", "error", err)
			continue
		}

		var parsedContext map[string]interface{}
		if err := json.Unmarshal([]byte(contextData), &parsedContext); err == nil {
			// Sanitize the context using PII redaction
			redactedInterface := telemetry.RedactInterfacePII(parsedContext)
			if redactedMap, ok := redactedInterface.(map[string]interface{}); ok {
				parsedContext = redactedMap
			}

			if redactedBytes, err := json.Marshal(parsedContext); err == nil {
				contextData = string(redactedBytes)
			}
		} else {
			contextData = telemetry.RedactPII(contextData)
		}

		payloads = append(payloads, SyncPayload{
			MemoryID: memoryID,
			Context:  contextData,
		})
		ids = append(ids, memoryID)
	}

	if len(payloads) == 0 {
		return
	}

	if err := d.sendToCloud(ctx, payloads); err != nil {
		slog.Error("hybrid_sync: failed to send escalations to cloud", "error", err)
		return
	}

	telemetry.RecordSyncEscalation(ctx, int64(len(payloads)))
	for range payloads {
		telemetry.RecordRagEscalation(ctx)
	}

	// Update the local SQLite database to mark as no longer requiring escalation
	if len(ids) > 0 {
		for _, id := range ids {
			// Update the JSON to set escalation_required to false
			updateQuery := "UPDATE swarm_memory_embeddings SET context = json_set(context, '$.escalation_required', false) WHERE memory_id = $1"
			_, err := tx.Exec(ctx, updateQuery, id)
			if err != nil {
				slog.Error("hybrid_sync: failed to update escalation_required status", "error", err)
				return
			}
		}
	}

	if err := tx.Commit(ctx); err != nil {
		slog.Error("hybrid_sync: failed to commit transaction", "error", err)
		return
	}

	slog.Debug("hybrid_sync: successfully synced and escalated RAG vectors", "count", len(payloads))
}

func (d *HybridSyncDaemon) sendToCloud(ctx context.Context, payloads []SyncPayload) error {
	jsonData, err := json.Marshal(payloads)
	if err != nil {
		return fmt.Errorf("marshal payloads: %w", err)
	}

	syncEndpoint := fmt.Sprintf("%s/api/sync/escalation", d.cloudAPIURL)

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, syncEndpoint, bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("create request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

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

type CRDTDelta struct {
	ID        string `json:"id"`
	EntityID  string `json:"entity_id"`
	Data      string `json:"data"`
	UpdatedAt string `json:"updated_at"`
}

func (d *HybridSyncDaemon) ProcessCRDTSync(ctx context.Context) {
	if !d.dbWrapper.IsSQLite() {
		return
	}

	// Phase 1: Read under one transaction
	tx, err := d.dbWrapper.Begin(ctx)
	if err != nil {
		slog.Error("hybrid_sync: failed to begin transaction for crdt sync read", "error", err)
		return
	}

	query := "SELECT id, entity_id, data, updated_at FROM crdt_deltas WHERE synced_to_cloud = false LIMIT 100"
	rows, err := tx.Query(ctx, query)
	if err != nil {
		slog.Error("hybrid_sync: failed to query crdt_deltas", "error", err)
		tx.Rollback(ctx)
		return
	}

	var deltas []CRDTDelta
	var ids []string

	for rows.Next() {
		var id, entityID, data string
		var updatedAt time.Time
		if err := rows.Scan(&id, &entityID, &data, &updatedAt); err != nil {
			slog.Error("hybrid_sync: failed to scan crdt_deltas", "error", err)
			continue
		}

		deltas = append(deltas, CRDTDelta{
			ID:        id,
			EntityID:  entityID,
			Data:      data,
			UpdatedAt: updatedAt.Format(time.RFC3339),
		})
		ids = append(ids, id)
	}
	rows.Close()
	tx.Rollback(ctx) // Finished reading, close tx before network I/O

	if len(deltas) == 0 {
		return
	}

	// Phase 2: Network I/O without holding a DB lock
	if err := d.sendCRDTToCloud(ctx, deltas); err != nil {
		slog.Error("hybrid_sync: failed to send crdt deltas to cloud", "error", err)
		return
	}

	// Phase 3: Write under a new transaction
	if len(ids) > 0 {
		updateTx, err := d.dbWrapper.Begin(ctx)
		if err != nil {
			slog.Error("hybrid_sync: failed to begin transaction for crdt sync update", "error", err)
			return
		}
		defer updateTx.Rollback(ctx)

		for _, id := range ids {
			updateQuery := "UPDATE crdt_deltas SET synced_to_cloud = true WHERE id = $1"
			_, err := updateTx.Exec(ctx, updateQuery, id)
			if err != nil {
				slog.Error("hybrid_sync: failed to update crdt_deltas status", "error", err)
				return
			}
		}

		if err := updateTx.Commit(ctx); err != nil {
			slog.Error("hybrid_sync: failed to commit transaction for crdt sync update", "error", err)
			return
		}
	}

	slog.Debug("hybrid_sync: successfully synced crdt deltas", "count", len(deltas))
}

func (d *HybridSyncDaemon) sendCRDTToCloud(ctx context.Context, deltas []CRDTDelta) error {
	payload := map[string]interface{}{
		"deltas": deltas,
	}
	jsonData, err := json.Marshal(payload)
	if err != nil {
		return fmt.Errorf("marshal crdt payload: %w", err)
	}

	syncEndpoint := fmt.Sprintf("%s/api/v1/sync/mcp-deltas", d.cloudAPIURL)

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, syncEndpoint, bytes.NewBuffer(jsonData))
	if err != nil {
		return fmt.Errorf("create crdt request: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")

	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return fmt.Errorf("do crdt request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 300 {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("unexpected status %d: %s", resp.StatusCode, string(body))
	}

	return nil
}
