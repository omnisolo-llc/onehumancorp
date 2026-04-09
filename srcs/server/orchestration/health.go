package orchestration

import (
	"context"
	"time"
)

// HybridHealthProbe details the system health across standalone and cloud modes.
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
	if h.sipDB != nil && h.sipDB.Provider() != nil {
		_, err := h.sipDB.Provider().Exec(ctx, "SELECT 1")
		probe.DBPing = time.Since(start)
		if err != nil {
			probe.Status = "degraded"
		} else {
			if !h.sipDB.Provider().IsSQLite() {
				probe.Mode = "cloud"
			}

			// Get sync backlog
			var count int
			err = h.sipDB.Provider().QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions WHERE status = 'PENDING' AND organization_id = $1", h.sipDB.orgID).Scan(&count)
			if err == nil {
				probe.SyncBacklog = count
			}
		}
	} else {
		probe.Status = "degraded"
	}

	if h.centrifugeNode != nil {
		probe.MeshActive = true
		_, err := h.centrifugeNode.node.Publish("mesh:health", []byte(`{"ping":"pong"}`))
		if err != nil {
			probe.MeshActive = false
			probe.Status = "degraded"
		}
	}

	return probe, nil
}
