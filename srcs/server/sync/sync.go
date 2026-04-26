package sync

import (
	"context"
	"fmt"
	"log"
	"os"
	"time"
)

// SyncDelta represents a delta sync request payload.
type SyncDelta struct {
	TenantID  string    `json:"tenant_id"`
	EntityID  string    `json:"entity_id"`
	Data      string    `json:"data"`
	UpdatedAt time.Time `json:"updated_at"`
}

// Syncer is the interface for pushing deltas to the data store.
type Syncer interface {
	SyncDeltas(ctx context.Context, deltas []SyncDelta) error
}

// MCPSyncer implements Syncer.
type MCPSyncer struct {
	Store Store
}

// Store is an interface for database execution, so we can support both modes.
type Store interface {
	Exec(ctx context.Context, query string, args ...interface{}) error
	Driver() string
}

// SyncDeltas handles the sync operation
func (m *MCPSyncer) SyncDeltas(ctx context.Context, deltas []SyncDelta) error {
	standalone := os.Getenv("OHC_STANDALONE") == "true"
	telemetryEnabled := os.Getenv("OHC_TELEMETRY_ENABLED") == "true"

	if standalone && telemetryEnabled {
		log.Printf("Telemetry: Syncing %d deltas", len(deltas))
	}

	for _, d := range deltas {
		var query string
		if m.Store.Driver() == "postgres" {
			query = `INSERT INTO mcp_deltas (tenant_id, entity_id, data, updated_at)
VALUES ($1, $2, $3, $4)
ON CONFLICT (tenant_id, entity_id) DO UPDATE
SET data = EXCLUDED.data, updated_at = EXCLUDED.updated_at
WHERE mcp_deltas.updated_at < EXCLUDED.updated_at`
		} else if m.Store.Driver() == "sqlite3" || m.Store.Driver() == "sqlite" {
			query = `INSERT INTO mcp_deltas (tenant_id, entity_id, data, updated_at)
VALUES (?, ?, ?, ?)
ON CONFLICT (tenant_id, entity_id) DO UPDATE
SET data = EXCLUDED.data, updated_at = EXCLUDED.updated_at
WHERE mcp_deltas.updated_at < EXCLUDED.updated_at`
		} else {
			return fmt.Errorf("unsupported driver: %s", m.Store.Driver())
		}
		err := m.Store.Exec(ctx, query, d.TenantID, d.EntityID, d.Data, d.UpdatedAt)
		if err != nil {
			return err
		}
	}

	return nil
}

func NewMCPSyncer(store Store) *MCPSyncer {
	return &MCPSyncer{Store: store}
}
