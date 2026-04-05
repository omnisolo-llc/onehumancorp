package orchestration

import (
	"context"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// HybridHealthProbe defines the structure for health check probe metrics
// returned by the orchestrator Hub.
type HybridHealthProbe struct {
	Mode        string        `json:"mode"`
	Status      string        `json:"status"`
	DBPing      time.Duration `json:"db_ping"`
	SyncBacklog int           `json:"sync_backlog"`
	MeshActive  bool          `json:"mesh_active"`
}

// CheckHealth executes health verifications across the hybrid architecture,
// inspecting the database connection, checking the sync backlog, and verifying
// the Mesh (Centrifuge) pub/sub connectivity.
func (h *Hub) CheckHealth(ctx context.Context) (HybridHealthProbe, error) {
	probe := HybridHealthProbe{
		Mode:        "standalone",
		Status:      "healthy",
		MeshActive:  false,
		SyncBacklog: 0,
	}

	start := time.Now()
	if h.sipDB != nil && h.sipDB.db != nil {
		_, err := h.sipDB.db.Exec(ctx, "SELECT 1")
		probe.DBPing = time.Since(start)
		if err != nil {
			probe.Status = "degraded"
		} else {
			if !h.sipDB.db.IsSQLite() {
				probe.Mode = "cloud"
			}

			// In Standalone Mode, local-to-cloud missions that haven't been synced might be marked differently
			// Let's count pending missions or sync queue. Assuming agent_missions with synced_to_cloud = false/0
			var count int
			query := "SELECT COUNT(*) FROM agent_missions WHERE synced_to_cloud = false"
			if h.sipDB.db.IsSQLite() {
				query = "SELECT COUNT(*) FROM agent_missions WHERE synced_to_cloud = 0"
			}
			err = h.sipDB.db.QueryRow(ctx, query).Scan(&count)
			if err == nil {
				probe.SyncBacklog = count
			} else {
				// Fallback to checking pending missions
				err2 := h.sipDB.db.QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions WHERE status = 'PENDING'").Scan(&count)
				if err2 == nil {
					probe.SyncBacklog = count
				}
			}
		}
	} else {
		probe.Status = "degraded"
	}

	// Verify mesh channel connectivity
	if h.centrifugeNode != nil {
		probe.MeshActive = true
		// "mesh:health" check
		payload, _ := json.Marshal(map[string]string{"ping": "pong"})
		_, err := h.centrifugeNode.node.Publish("mesh:health", payload)
		if err != nil {
			probe.MeshActive = false
			probe.Status = "degraded"
		}
	}

	telemetry.RecordAgentApiCall(ctx, "system", "health", "check")

	return probe, nil
}
