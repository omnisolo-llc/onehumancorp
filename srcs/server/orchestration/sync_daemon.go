package orchestration

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
)

var (
	syncMeter        = otel.Meter("github.com/onehumancorp/mono/srcs/server/orchestration")
	syncCountMetric, _ = syncMeter.Int64Counter("ohc.sync.escalations.count")
)

// StartSyncDaemon monitors local SQLite for CLOUD_ESCALATION missions,
// sanitizes them, injects into cloud Postgres, polls for completion,
// and syncs results back to local SQLite.
func StartSyncDaemon(ctx context.Context, localDB db.Provider, cloudDB db.Provider) {
	if localDB == nil || cloudDB == nil {
		slog.Error("sync_daemon: localDB or cloudDB is nil")
		return
	}

	ticker := time.NewTicker(1 * time.Second)
	go func() {
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				processSyncTick(ctx, localDB, cloudDB)
			}
		}
	}()
}

func processSyncTick(ctx context.Context, localDB db.Provider, cloudDB db.Provider) {
	// 1. Monitor local SQLite for CLOUD_ESCALATION
	rows, err := localDB.Query(ctx, "SELECT id, payload FROM agent_missions WHERE status = 'CLOUD_ESCALATION' LIMIT 100")
	if err != nil {
		slog.Error("sync_daemon: failed to query localDB", "error", err)
		return
	}

	type mission struct {
		id      string
		payload string
	}
	var missionsToEscalate []mission

	for rows.Next() {
		var m mission
		if err := rows.Scan(&m.id, &m.payload); err != nil {
			slog.Error("sync_daemon: scan error", "error", err)
			continue
		}
		missionsToEscalate = append(missionsToEscalate, m)
	}
	if err := rows.Err(); err != nil {
		slog.Error("sync_daemon: rows error", "error", err)
	}
	rows.Close()

	for _, m := range missionsToEscalate {
		// Sanitize
		sanitized, err := SanitizePayload(m.payload)
		if err != nil {
			slog.Error("sync_daemon: sanitize error", "error", err)
			continue
		}

		// Inject into cloudDB
		_, err = cloudDB.Exec(ctx, "INSERT INTO agent_missions (id, status, payload) VALUES ($1, 'PENDING', $2) ON CONFLICT(id) DO UPDATE SET status = 'PENDING', payload = $2", m.id, sanitized)
		if err != nil {
			slog.Error("sync_daemon: cloud inject error", "error", err)
			continue
		}

		// Update local status to IN_CLOUD to avoid re-syncing
		_, err = localDB.Exec(ctx, "UPDATE agent_missions SET status = 'IN_CLOUD' WHERE id = $1", m.id)
		if err != nil {
			slog.Error("sync_daemon: local update error", "error", err)
			continue
		}

		if syncCountMetric != nil {
			syncCountMetric.Add(ctx, 1)
		}
	}

	// 2. Poll cloud database for completion (status = 'DONE')
	// For all local missions with 'IN_CLOUD', check cloud
	inCloudRows, err := localDB.Query(ctx, "SELECT id FROM agent_missions WHERE status = 'IN_CLOUD' LIMIT 100")
	if err != nil {
		slog.Error("sync_daemon: query IN_CLOUD error", "error", err)
		return
	}

	var inCloudIDs []string
	for inCloudRows.Next() {
		var id string
		if err := inCloudRows.Scan(&id); err != nil {
			continue
		}
		inCloudIDs = append(inCloudIDs, id)
	}
	if err := inCloudRows.Err(); err != nil {
		slog.Error("sync_daemon: inCloudRows error", "error", err)
	}
	inCloudRows.Close()

	if len(inCloudIDs) == 0 {
		return
	}

	// Fetch all DONE statuses from cloud in a single query
	var query string
	var args []any
	if cloudDB.IsSQLite() {
		placeholders := ""
		for i, id := range inCloudIDs {
			if i > 0 {
				placeholders += ","
			}
			placeholders += "?"
			args = append(args, id)
		}
		query = "SELECT id, payload FROM agent_missions WHERE id IN (" + placeholders + ") AND status = 'DONE'"
	} else {
		placeholders := ""
		for i, id := range inCloudIDs {
			if i > 0 {
				placeholders += ","
			}
			placeholders += "$" + fmt.Sprintf("%d", i+1)
			args = append(args, id)
		}
		query = "SELECT id, payload FROM agent_missions WHERE id IN (" + placeholders + ") AND status = 'DONE'"
	}

	cloudRows, err := cloudDB.Query(ctx, query, args...)
	if err != nil {
		slog.Error("sync_daemon: query cloud DONE error", "error", err)
		return
	}
	defer cloudRows.Close()

	type doneMission struct {
		id      string
		payload string
	}
	var doneMissions []doneMission

	for cloudRows.Next() {
		var dm doneMission
		if err := cloudRows.Scan(&dm.id, &dm.payload); err == nil {
			doneMissions = append(doneMissions, dm)
		}
	}
	if err := cloudRows.Err(); err != nil {
		slog.Error("sync_daemon: cloudRows error", "error", err)
	}

	for _, dm := range doneMissions {
		// Pull back to local
		_, err = localDB.Exec(ctx, "UPDATE agent_missions SET status = 'DONE', payload = $1 WHERE id = $2", dm.payload, dm.id)
		if err != nil {
			slog.Error("sync_daemon: update local DONE error", "error", err)
		}
	}
}
