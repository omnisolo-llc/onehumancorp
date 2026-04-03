package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// SyncDaemon is the background service that syncs local RAG RAG state to the cloud.
type SyncDaemon struct {
	dbProvider   db.Provider
	cloudURL     string
	cloudToken   string
	httpClient   *http.Client
}

// NewSyncDaemon creates a new SyncDaemon.
func NewSyncDaemon(dbProvider db.Provider, cloudURL string, cloudToken string) *SyncDaemon {
	return &SyncDaemon{
		dbProvider:   dbProvider,
		cloudURL:     cloudURL,
		cloudToken:   cloudToken,
		httpClient:   &http.Client{Timeout: 10 * time.Second},
	}
}

// Start runs the sync loop.
func (d *SyncDaemon) Start(ctx context.Context, tickDuration time.Duration) {
	ticker := time.NewTicker(tickDuration)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			d.ProcessSyncTick(ctx)
		}
	}
}

// MissionPayload struct represents the data we sync.
type MissionPayload struct {
	ID        string          `json:"id"`
	Status    string          `json:"status"`
	Payload   json.RawMessage `json:"payload"`
}

// ProcessSyncTick is the extracted loop body for easy testing.
func (d *SyncDaemon) ProcessSyncTick(ctx context.Context) {
	// Query the local SQLite agent_missions for items marked for cloud escalation.
	// We'll use status = 'ESCALATE' to identify such missions.
	// Using LIMIT 100 as per requirements to prevent memory explosion.
	query := `
		SELECT id, status, payload
		FROM agent_missions
		WHERE status = 'ESCALATE'
		ORDER BY created_at ASC
		LIMIT 100
	`

	rows, err := d.dbProvider.Query(ctx, query)
	if err != nil {
		log.Printf("sync_daemon: Failed to query local agent_missions: %v", err)
		return
	}
	defer rows.Close()

	var missions []MissionPayload
	for rows.Next() {
		var m MissionPayload
		if err := rows.Scan(&m.ID, &m.Status, &m.Payload); err != nil {
			log.Printf("sync_daemon: Failed to scan mission: %v", err)
			continue
		}
		missions = append(missions, m)
	}

	for _, m := range missions {
		// Sanitize payload using telemetry.RedactPII
		sanitizedPayloadStr := telemetry.RedactPII(string(m.Payload))

		reqBody, err := json.Marshal(map[string]interface{}{
			"id":      m.ID,
			"status":  "ESCALATED", // Change status locally later, but send as ESCALATED or PENDING to cloud
			"payload": json.RawMessage(sanitizedPayloadStr),
		})
		if err != nil {
			log.Printf("sync_daemon: Failed to marshal request: %v", err)
			continue
		}

		req, err := http.NewRequestWithContext(ctx, http.MethodPost, d.cloudURL+"/api/sync/missions", bytes.NewBuffer(reqBody))
		if err != nil {
			log.Printf("sync_daemon: Failed to create request: %v", err)
			continue
		}

		req.Header.Set("Content-Type", "application/json")
		if d.cloudToken != "" {
			req.Header.Set("Authorization", "Bearer "+d.cloudToken)
		}

		resp, err := d.httpClient.Do(req)
		if err != nil {
			log.Printf("sync_daemon: HTTP request failed: %v", err)
			continue
		}

		if resp.StatusCode < 200 || resp.StatusCode >= 300 {
			body, _ := io.ReadAll(resp.Body)
			resp.Body.Close()
			log.Printf("sync_daemon: Cloud rejected payload (status %d): %s", resp.StatusCode, body)
			continue
		}
		resp.Body.Close()

		// Update local status to SYNCED
		_, err = d.dbProvider.Exec(ctx, "UPDATE agent_missions SET status = 'SYNCED' WHERE id = $1", m.ID)
		if err != nil {
			log.Printf("sync_daemon: Failed to update local mission status: %v", err)
		}
	}
}
