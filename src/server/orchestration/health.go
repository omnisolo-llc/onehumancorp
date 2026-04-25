package orchestration

import (
	"context"
	"database/sql"
	"net/http"
	"os"
	"time"

	agentgrpc "github.com/onehumancorp/mono/src/server/agents/grpc"
)

// HybridHealthProbe details the system health across standalone and cloud modes.
type HybridHealthProbe struct {
	Mode               string        `json:"mode"`
	Status             string        `json:"status"`
	DBPing             time.Duration `json:"db_ping"`
	SyncBacklog        int           `json:"sync_backlog"`
	StuckMissions      int           `json:"stuck_missions"`
	LastSyncTime       time.Time     `json:"last_sync_time"`
	MeshActive         bool          `json:"mesh_active"`
	CloudConnected     bool          `json:"cloud_connected"`
	BuiltinAgentActive bool          `json:"builtin_agent_active"`
}

// CheckHealth returns a HybridHealthProbe detailing the system health.
func (h *Hub) CheckHealth(ctx context.Context) (HybridHealthProbe, error) {
	probe := HybridHealthProbe{
		Mode:               "standalone",
		Status:             "healthy",
		MeshActive:         false,
		CloudConnected:     true, // Default to true for cloud mode
		BuiltinAgentActive: false,
		SyncBacklog:        0,
		StuckMissions:      0,
		LastSyncTime:       time.Time{},
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

			// Get last sync time
			var lastSync sql.NullTime
			err = h.sipDB.Provider().QueryRow(ctx, "SELECT MAX(updated_at) FROM agent_missions WHERE status = 'SYNCED'").Scan(&lastSync)
			if err == nil && lastSync.Valid {
				probe.LastSyncTime = lastSync.Time
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

	// Health Guardianship: Implement cloud connectivity check for Standalone mode
	if os.Getenv("OHC_STANDALONE") == "true" {
		cloudURL := os.Getenv("OHC_CORE_URL")
		if cloudURL == "" {
			cloudURL = "https://core.onehumancorp.com"
		}

		client := &http.Client{Timeout: 2 * time.Second}
		resp, err := client.Get(cloudURL + "/health")
		if err != nil || resp.StatusCode != http.StatusOK {
			probe.CloudConnected = false
		} else {
			probe.CloudConnected = true
		}
		if resp != nil && resp.Body != nil {
			_ = resp.Body.Close()
		}
	}

	// Cross-Mode Health Monitoring: Ping the builtin agent
	h.agentClientMu.Lock()
	if h.agentClient == nil {
		client, err := agentgrpc.NewClient(agentgrpc.AddressFromEnv(), agentgrpc.ClientOptionsFromEnv())
		if err == nil {
			h.agentClient = client
		}
	}
	client := h.agentClient
	h.agentClientMu.Unlock()

	if client != nil {
		pingCtx, cancel := context.WithTimeout(ctx, 2*time.Second)
		defer cancel()
		_, pingErr := client.Ping(pingCtx)
		if pingErr == nil {
			probe.BuiltinAgentActive = true
		}
	}

	return probe, nil

}
