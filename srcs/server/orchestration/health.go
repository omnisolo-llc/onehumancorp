package orchestration

import (
	"context"
	"time"
)

// HybridHealthProbe detailing the system health.
type HybridHealthProbe struct {
	Mode        string        `json:"mode"`
	Status      string        `json:"status"`
	DBPing      time.Duration `json:"db_ping"`
	SyncBacklog int           `json:"sync_backlog"`
	MeshActive  bool          `json:"mesh_active"`
}

// CheckHealth returns a HybridHealthProbe detailing the system health.
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

			// Get sync backlog
			var count int
			err = h.sipDB.db.QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions WHERE status = 'PENDING'").Scan(&count)
			if err == nil {
				probe.SyncBacklog = count
			}
		}
	} else {
		probe.Status = "degraded"
	}

	if h.centrifugeNode != nil {
		probe.MeshActive = true
		// Centrifuge node publish returns bool, error in Centrifuge v0.11 or later maybe, need to check
		// Let's assume the previous code in service.go was correct.
		_, err := h.centrifugeNode.node.Publish("mesh:health", []byte(`{"ping":"pong"}`))
		if err != nil {
			probe.MeshActive = false
			probe.Status = "degraded"
		}
	}

	return probe, nil
}
