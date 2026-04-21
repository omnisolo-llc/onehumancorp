package mcp

import (
	"context"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/sync"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// Syncer interface for Cross-Mode MCP Sync
type Syncer interface {
	SyncDeltas(ctx context.Context, deltas []sync.SyncDelta) error
}

type mcpSyncer struct {
	dbWrapper *db.DB
}

func NewSyncer(dbWrapper *db.DB) Syncer {
	return &mcpSyncer{
		dbWrapper: dbWrapper,
	}
}

func getTenantID(ctx context.Context) string {
	claims := auth.ClaimsFromContext(ctx)
	if claims != nil && claims.OrganizationID != "" {
		return claims.OrganizationID
	}
	return "sys"
}

func (s *mcpSyncer) SyncDeltas(ctx context.Context, deltas []sync.SyncDelta) error {
	if len(deltas) == 0 {
		return nil
	}

	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	telemetryEnabled := os.Getenv("OHC_TELEMETRY_ENABLED") == "true"

	recordTelemetry := true
	if isStandalone && !telemetryEnabled {
		recordTelemetry = false
	}

	tenantID := getTenantID(ctx)

	for _, delta := range deltas {
		if s.dbWrapper.IsSQLite() {
			// Standalone Mode: SQLite logs state changes (deltas).
			query := `
				INSERT INTO crdt_deltas (tenant_id, id, entity_id, data, updated_at, synced_to_cloud)
				VALUES ($1, $2, $3, $4, $5, false)
				ON CONFLICT(tenant_id, id) DO UPDATE SET
					data = excluded.data,
					updated_at = excluded.updated_at,
					synced_to_cloud = false
			`
			_, err := s.dbWrapper.Exec(ctx, query, tenantID, delta.ID, delta.EntityID, delta.Data, delta.UpdatedAt.Format(time.RFC3339))
			if err != nil {
				return err
			}
		} else {
			// Cloud Mode: Postgres handles ingestion and conflict resolution.
			query := `
				INSERT INTO crdt_deltas (tenant_id, id, entity_id, data, updated_at, synced_to_cloud)
				VALUES ($1, $2, $3, $4, $5, true)
				ON CONFLICT(tenant_id, id) DO UPDATE SET
					data = excluded.data,
					updated_at = excluded.updated_at,
					synced_to_cloud = true
				WHERE excluded.updated_at > crdt_deltas.updated_at
			`
			_, err := s.dbWrapper.Exec(ctx, query, tenantID, delta.ID, delta.EntityID, delta.Data, delta.UpdatedAt.Format(time.RFC3339))
			if err != nil {
				return err
			}
		}
	}

	if recordTelemetry && telemetry.SyncCompletedCount != nil {
		telemetry.SyncCompletedCount.Add(ctx, int64(len(deltas)))
	}

	return nil
}
