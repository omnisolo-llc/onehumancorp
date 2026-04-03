package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// MCPRAGSynchronizer synchronizes local SQLite RAG state to the cloud Postgres orchestration engine.
type MCPRAGSynchronizer struct {
	dbProvider db.Provider
	httpClient *http.Client
}

// NewMCPRAGSynchronizer creates a new MCPRAGSynchronizer.
func NewMCPRAGSynchronizer(dbProvider db.Provider) *MCPRAGSynchronizer {
	return &MCPRAGSynchronizer{
		dbProvider: dbProvider,
		httpClient: &http.Client{Timeout: 10 * time.Second},
	}
}

// Start runs the sync daemon in the background.
func (s *MCPRAGSynchronizer) Start(ctx context.Context, tickDuration time.Duration) {
	ticker := time.NewTicker(tickDuration)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			s.ProcessSyncTick(ctx)
		}
	}
}

// ProcessSyncTick performs a single tick of the sync loop. It finds agent missions
// requiring cloud escalation, sanitizes them, and pushes them to the cloud.
func (s *MCPRAGSynchronizer) ProcessSyncTick(ctx context.Context) {
	// Only run this in Standalone mode or if DATABASE_URL is an sqlite DB
	if os.Getenv("OHC_STANDALONE") != "true" && os.Getenv("DATABASE_URL") != "" {
		return
	}

	cloudURL := os.Getenv("OHC_CORE_URL")
	if cloudURL == "" {
		cloudURL = "http://localhost:8080" // Fallback
	}
	syncEndpoint := cloudURL + "/api/sync/missions"

	// Fetch up to 100 missions that are marked for cloud escalation and not yet synced
	query := `
		SELECT id, payload
		FROM agent_missions
		WHERE status = 'ESCALATED' OR status = 'PENDING'
		ORDER BY created_at ASC
		LIMIT 100
	`

	rows, err := s.dbProvider.Query(ctx, query)
	if err != nil {
		slog.Error("failed to query agent_missions for sync", "error", err)
		return
	}
	defer rows.Close()

	type missionRecord struct {
		id      string
		payload string
	}
	var missions []missionRecord

	for rows.Next() {
		var m missionRecord
		if err := rows.Scan(&m.id, &m.payload); err != nil {
			slog.Error("failed to scan mission record", "error", err)
			continue
		}
		missions = append(missions, m)
	}

	for _, m := range missions {
		var rawData interface{}
		if err := json.Unmarshal([]byte(m.payload), &rawData); err != nil {
			slog.Warn("Failed to unmarshal mission payload for sanitization, skipping", "mission_id", m.id)
			continue
		}

		if payloadData, ok := rawData.(map[string]interface{}); ok {
			// Ensure minimal local context payload. Sanitize rag_context if any.
			delete(payloadData, "rag_context")
			payloadData["id"] = m.id
		}

		// Apply redaction to prevent raw sensitive user data from being pushed
		rawData = s.sanitizeRecursively(rawData)

		sanitizedBytes, err := json.Marshal(rawData)
		if err != nil {
			slog.Warn("Failed to marshal sanitized mission payload, skipping sync", "mission_id", m.id)
			continue
		}

		req, err := http.NewRequestWithContext(ctx, "POST", syncEndpoint, bytes.NewReader(sanitizedBytes))
		if err != nil {
			slog.Error("failed to create sync request", "error", err)
			continue
		}
		req.Header.Set("Content-Type", "application/json")
		req.Header.Set("Authorization", "Bearer "+os.Getenv("OHC_CLOUD_API_KEY"))

		resp, err := s.httpClient.Do(req)
		if err != nil {
			slog.Error("failed to send sync request", "mission_id", m.id, "error", err)
			continue
		}

		if (resp.StatusCode >= 200 && resp.StatusCode < 300) || resp.StatusCode == http.StatusConflict {
			// Mark as synced to avoid duplicate syncing
			_, updateErr := s.dbProvider.Exec(ctx, "UPDATE agent_missions SET status = 'SYNCED' WHERE id = $1", m.id)
			if updateErr != nil {
				slog.Error("failed to update mission status after sync", "mission_id", m.id, "error", updateErr)
			}
		} else {
			slog.Warn("cloud sync returned non-success status", "mission_id", m.id, "status", resp.StatusCode)
		}
		resp.Body.Close()
	}
}

// sanitizeRecursively traverses the payload to apply PII redaction.
func (s *MCPRAGSynchronizer) sanitizeRecursively(data interface{}) interface{} {
	switch v := data.(type) {
	case string:
		return telemetry.RedactPII(v)
	case map[string]interface{}:
		for key, val := range v {
			v[key] = s.sanitizeRecursively(val)
		}
		return v
	case []interface{}:
		for i, val := range v {
			v[i] = s.sanitizeRecursively(val)
		}
		return v
	default:
		return v
	}
}
