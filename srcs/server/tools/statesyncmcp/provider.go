package statesyncmcp

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"time"
	"errors"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// StateSyncProvider defines the interface for synchronizing state
type StateSyncProvider interface {
	SyncUp(ctx context.Context, claims *auth.Claims) error
	SyncDown(ctx context.Context, claims *auth.Claims) error
	GetStatus(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
}

// SyncStateTransition represents a state change to sync
type SyncStateTransition struct {
	EntityID   string    `json:"entity_id"`
	EntityType string    `json:"entity_type"`
	FromState  string    `json:"from_state"`
	ToState    string    `json:"to_state"`
	AgentID    string    `json:"agent_id"`
	Reason     string    `json:"reason"`
	OccurredAt time.Time `json:"occurred_at"`
}

// DefaultStateSyncProvider implements StateSyncProvider
type DefaultStateSyncProvider struct {
	db     db.Provider
	client *http.Client
}

// NewDefaultStateSyncProvider creates a new default provider
func NewDefaultStateSyncProvider(database db.Provider) *DefaultStateSyncProvider {
	return &DefaultStateSyncProvider{
		db: database,
		client: &http.Client{
			Timeout: 10 * time.Second,
		},
	}
}

func (p *DefaultStateSyncProvider) isCloudMode() bool {
	return os.Getenv("OHC_MULTITENANT") == "true"
}

func (p *DefaultStateSyncProvider) getCloudURL() string {
	url := os.Getenv("OHC_CORE_URL")
	if url == "" {
		url = "http://localhost:8080"
	}
	return url
}

// setupSyncCursorTable creates the sync cursor table if it doesn't exist
func (p *DefaultStateSyncProvider) setupSyncCursorTable(ctx context.Context) error {
	query := `
		CREATE TABLE IF NOT EXISTS state_sync_cursors (
			id TEXT PRIMARY KEY,
			last_synced_at TIMESTAMPTZ NOT NULL
		);
	`
	_, err := p.db.Exec(ctx, query)
	return err
}

// getLastSyncedAt retrieves the last synced timestamp
func (p *DefaultStateSyncProvider) getLastSyncedAt(ctx context.Context) (time.Time, error) {
	if err := p.setupSyncCursorTable(ctx); err != nil {
		return time.Time{}, err
	}

	var lastSyncedAt time.Time
	err := p.db.QueryRow(ctx, "SELECT last_synced_at FROM state_sync_cursors WHERE id = 'sync_up'").Scan(&lastSyncedAt)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) || err.Error() == "sql: no rows in result set" {
			return time.Time{}, nil // Return zero time if no cursor exists
		}
		return time.Time{}, err
	}
	return lastSyncedAt, nil
}

// updateLastSyncedAt updates the sync cursor
func (p *DefaultStateSyncProvider) updateLastSyncedAt(ctx context.Context, t time.Time) error {
	query := `
		INSERT INTO state_sync_cursors (id, last_synced_at)
		VALUES ('sync_up', $1)
		ON CONFLICT (id) DO UPDATE SET last_synced_at = EXCLUDED.last_synced_at
	`
	if p.db.IsSQLite() {
		query = `
			INSERT INTO state_sync_cursors (id, last_synced_at)
			VALUES ('sync_up', $1)
			ON CONFLICT (id) DO UPDATE SET last_synced_at = EXCLUDED.last_synced_at
		`
	}
	_, err := p.db.Exec(ctx, query, t)
	return err
}

// SyncUp pushes local changes to the cloud
func (p *DefaultStateSyncProvider) SyncUp(ctx context.Context, claims *auth.Claims) error {
	if p.isCloudMode() {
		return nil // No-op in cloud mode
	}

	lastSyncedAt, err := p.getLastSyncedAt(ctx)
	if err != nil {
		return fmt.Errorf("failed to get last synced at: %w", err)
	}

	query := `
		SELECT entity_id, entity_type, from_state, to_state, agent_id, reason, occurred_at
		FROM state_machine_transitions
		WHERE occurred_at > $1
		ORDER BY occurred_at ASC
	`

	rows, err := p.db.Query(ctx, query, lastSyncedAt)
	if err != nil {
		return fmt.Errorf("failed to query local transitions: %w", err)
	}
	defer rows.Close()

	var transitions []SyncStateTransition
	var latestTime time.Time
	for rows.Next() {
		var t SyncStateTransition
		var agentID sql.NullString
		var reason sql.NullString

		err := rows.Scan(&t.EntityID, &t.EntityType, &t.FromState, &t.ToState, &agentID, &reason, &t.OccurredAt)
		if err != nil {
			return fmt.Errorf("failed to scan transition: %w", err)
		}

		if agentID.Valid {
			t.AgentID = agentID.String
		}
		if reason.Valid {
			t.Reason = reason.String
		}

		transitions = append(transitions, t)
		if t.OccurredAt.After(latestTime) {
			latestTime = t.OccurredAt
		}
	}

	if len(transitions) == 0 {
		return nil
	}

	payload, err := json.Marshal(transitions)
	if err != nil {
		return fmt.Errorf("failed to marshal transitions: %w", err)
	}

	req, err := http.NewRequestWithContext(ctx, "POST", p.getCloudURL()+"/api/sync/up", bytes.NewBuffer(payload))
	if err != nil {
		return fmt.Errorf("failed to create sync request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")
	if claims != nil {
		// Provide authorization using a mock token containing the org ID
		req.Header.Set("Authorization", fmt.Sprintf("Bearer sync-token-%s", claims.OrganizationID))
		req.Header.Set("X-Organization-ID", claims.OrganizationID)
	}

	resp, err := p.client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to send sync request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("sync failed with status %d: %s", resp.StatusCode, string(body))
	}

	// Update cursor on success
	if err := p.updateLastSyncedAt(ctx, latestTime); err != nil {
		return fmt.Errorf("failed to update sync cursor: %w", err)
	}

	return nil
}

// SyncDown fetches cloud changes to local
func (p *DefaultStateSyncProvider) SyncDown(ctx context.Context, claims *auth.Claims) error {
	if p.isCloudMode() {
		return nil // No-op in cloud mode
	}

	req, err := http.NewRequestWithContext(ctx, "GET", p.getCloudURL()+"/api/sync/down", nil)
	if err != nil {
		return fmt.Errorf("failed to create sync down request: %w", err)
	}

	if claims != nil {
		req.Header.Set("Authorization", fmt.Sprintf("Bearer sync-token-%s", claims.OrganizationID))
		req.Header.Set("X-Organization-ID", claims.OrganizationID)
	}

	resp, err := p.client.Do(req)
	if err != nil {
		return fmt.Errorf("failed to send sync down request: %w", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("sync down failed with status %d", resp.StatusCode)
	}

	var transitions []SyncStateTransition
	if err := json.NewDecoder(resp.Body).Decode(&transitions); err != nil {
		return fmt.Errorf("failed to decode transitions: %w", err)
	}

	// Apply transitions to local DB
	for _, t := range transitions {
		// We use a simple strategy: last write wins for state machine transitions.
		// In a real scenario, this would integrate deeply with the TaskManager to safely update task status.
		if t.EntityType == "SHARED_TASK" {
			updateQuery := `
				UPDATE shared_tasks
				SET status = $1, agent_id = $2, updated_at = $3
				WHERE id = $4 AND updated_at < $3
			`
			_, err := p.db.Exec(ctx, updateQuery, t.ToState, t.AgentID, t.OccurredAt, t.EntityID)
			if err != nil {
				return fmt.Errorf("failed to apply transition for entity %s: %w", t.EntityID, err)
			}
		}
	}

	return nil
}

// GetStatus returns the sync status
func (p *DefaultStateSyncProvider) GetStatus(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	if p.isCloudMode() {
		return map[string]interface{}{
			"mode": "cloud",
			"status": "active",
		}, nil
	}

	lastSyncedAt, err := p.getLastSyncedAt(ctx)
	var syncTime string
	if err != nil {
		syncTime = "unknown"
	} else if lastSyncedAt.IsZero() {
		syncTime = "never"
	} else {
		syncTime = lastSyncedAt.Format(time.RFC3339)
	}

	return map[string]interface{}{
		"mode": "standalone",
		"status": "synced",
		"last_synced_at": syncTime,
	}, nil
}
