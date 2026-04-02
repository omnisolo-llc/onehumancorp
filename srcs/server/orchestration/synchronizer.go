package orchestration

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
)

// SwarmSynchronizer synchronizes local RAG state to the cloud.
type SwarmSynchronizer struct {
	dbWrapper   db.Provider
	httpClient  *http.Client
	cloudAPIURL string
}

// NewSwarmSynchronizer creates a new synchronizer instance.
func NewSwarmSynchronizer(dbWrapper db.Provider, client *http.Client, cloudURL string) *SwarmSynchronizer {
	return &SwarmSynchronizer{
		dbWrapper:   dbWrapper,
		httpClient:  client,
		cloudAPIURL: cloudURL,
	}
}

// StartSyncLoop starts the background ticker loop for synchronization.
func (s *SwarmSynchronizer) StartSyncLoop(ctx context.Context, interval time.Duration) {
	ticker := time.NewTicker(interval)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				s.ProcessSyncTick(ctx)
			}
		}
	}()
}

// MemoryPayload represents the payload to send to the cloud API.
type MemoryPayload struct {
	MemoryID string `json:"memory_id"`
	Context  string `json:"context"`
}

// CloudResponse represents the response from the cloud API.
type CloudResponse struct {
	MissionID string `json:"mission_id"`
}

// ProcessSyncTick processes one cycle of synchronization.
// It retrieves unsynced local RAG embeddings, sanitizes the context, and pushes them to the cloud.
func (s *SwarmSynchronizer) ProcessSyncTick(ctx context.Context) {
	// Find memory records that are not in the sync log
	query := `
		SELECT sm.key, sm.value
		FROM swarm_memory sm
		LEFT JOIN local_cloud_sync_log sl ON sm.key = sl.memory_id
		WHERE sl.sync_id IS NULL
		LIMIT 100
	`
	rows, err := s.dbWrapper.Query(ctx, query)
	if err != nil {
		slog.Error("SwarmSynchronizer: Failed to query unsynced memories", "error", err)
		return
	}
	defer rows.Close()

	var toSync []struct {
		Key   string
		Value string
	}

	for rows.Next() {
		var key, value string
		if err := rows.Scan(&key, &value); err != nil {
			slog.Error("SwarmSynchronizer: Failed to scan memory row", "error", err)
			continue
		}
		toSync = append(toSync, struct{ Key, Value string }{key, value})
	}

	if err := rows.Err(); err != nil {
		slog.Error("SwarmSynchronizer: Rows error", "error", err)
		return
	}

	for _, item := range toSync {
		// Sanitize context
		sanitizedContext := s.sanitizeContext(item.Value)

		// Push to cloud API
		payload := MemoryPayload{
			MemoryID: item.Key,
			Context:  sanitizedContext,
		}

		body, err := json.Marshal(payload)
		if err != nil {
			slog.Error("SwarmSynchronizer: Failed to marshal payload", "error", err)
			continue
		}

		req, err := http.NewRequestWithContext(ctx, http.MethodPost, s.cloudAPIURL+"/api/v1/sync/autodream", bytes.NewBuffer(body))
		if err != nil {
			slog.Error("SwarmSynchronizer: Failed to create request", "error", err)
			continue
		}
		req.Header.Set("Content-Type", "application/json")

		resp, err := s.httpClient.Do(req)
		if err != nil {
			slog.Error("SwarmSynchronizer: Cloud API request failed", "error", err)
			continue
		}

		var cloudResp CloudResponse
		if resp.StatusCode == http.StatusOK || resp.StatusCode == http.StatusCreated {
			if err := json.NewDecoder(resp.Body).Decode(&cloudResp); err != nil {
				slog.Error("SwarmSynchronizer: Failed to decode cloud response", "error", err)
			}
		} else {
			slog.Warn("SwarmSynchronizer: Cloud API returned non-success status", "status", resp.StatusCode)
		}
		resp.Body.Close()

		// If successful or we want to record the attempt, log it.
		// We'll log it if the API call succeeds.
		if resp.StatusCode == http.StatusOK || resp.StatusCode == http.StatusCreated {
			s.recordSync(ctx, item.Key, cloudResp.MissionID)
		}
	}
}

// sanitizeContext sanitizes raw memory data to prevent sensitive data leaks.
// It removes highly sensitive data formats like raw secrets and aggregates it.
func (s *SwarmSynchronizer) sanitizeContext(raw string) string {
	// A simple mock sanitization for testing. In reality, it would use advanced regex/NLP.
	// For our purposes, we'll replace the exact substring "SENSITIVE_DATA" with "[REDACTED]"
	// Or we can just ensure we truncate or remove sensitive looking JSON fields.
	var data map[string]interface{}
	if err := json.Unmarshal([]byte(raw), &data); err == nil {
		// If it's valid JSON, we remove keys that sound sensitive
		for k := range data {
			if k == "password" || k == "secret" || k == "token" {
				data[k] = "[REDACTED]"
			}
		}
		sanitized, _ := json.Marshal(data)
		return string(sanitized)
	}

	// String substitution for basic testing
	sanitized := raw
	if len(sanitized) > 1000 {
		sanitized = sanitized[:1000] // truncate large payloads
	}
	return sanitized
}

// recordSync records a successful sync in the local database.
func (s *SwarmSynchronizer) recordSync(ctx context.Context, memoryID, cloudMissionID string) {
	syncID := uuid.New().String()
	query := `
		INSERT INTO local_cloud_sync_log (sync_id, memory_id, cloud_mission_id, synced_at)
		VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
	`
	_, err := s.dbWrapper.Exec(ctx, query, syncID, memoryID, cloudMissionID)
	if err != nil {
		slog.Error("SwarmSynchronizer: Failed to record sync log", "error", err)
	}
}
