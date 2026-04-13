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
	StuckMissions int         `json:"stuck_missions"`
	MeshActive  bool          `json:"mesh_active"`
}

// CheckHealth returns a HybridHealthProbe detailing the system health.
func (h *Hub) CheckHealth(ctx context.Context) (HybridHealthProbe, error) {
	probe := HybridHealthProbe{
		Mode:        "standalone",
		Status:      "healthy",
		MeshActive:  false,
		SyncBacklog: 0,
		StuckMissions: 0,
	}

	start := time.Now()
	if h.sipDB != nil && h.sipDB.Provider() != nil {
		err := h.sipDB.Provider().Ping(ctx)
		probe.DBPing = time.Since(start)
		if err != nil {
			probe.Status = "degraded"
		} else {
			if !h.sipDB.Provider().IsSQLite() {
				probe.Mode = "cloud"
			}

			// Get sync backlog
			var count int
			err = h.sipDB.Provider().QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions WHERE status = 'PENDING'").Scan(&count)
			if err == nil {
				probe.SyncBacklog = count
			}

			// Get stuck missions
			var stuckCount int
			err = h.sipDB.Provider().QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions WHERE status = 'STUCK' OR status = 'FAILED'").Scan(&stuckCount)
			if err == nil {
				probe.StuckMissions = stuckCount
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
