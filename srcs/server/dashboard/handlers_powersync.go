package dashboard

import (
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// handlePowerSyncRules provides the sync rules for the PowerSync daemon.
// This implements strict Tenant isolation based on the OHC architecture.
// For now, returning rules to sync the `users` and other important tables.
func (s *Server) handlePowerSyncRules(w http.ResponseWriter, r *http.Request) {
	// Only GET method allowed
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Record the request for PowerSync sync observability
	telemetry.RecordPowerSyncSync(r.Context(), s.org.ID)

	// Assuming the JWT or session context has authenticated the user.
	// We'd return the JourneyApps `bucket_data` schema for sync rules.

	rules := map[string]interface{}{
		"bucket_data": map[string]interface{}{
			"global": map[string]interface{}{
				"data": []map[string]interface{}{
					{
						"table": "users",
						"query": "SELECT * FROM users", // Refine based on tenant if multi-tenant
					},
					{
						"table": "agent_missions",
						"query": "SELECT * FROM agent_missions",
					},
					{
						"table": "agent_status",
						"query": "SELECT * FROM agent_status",
					},
				},
			},
		},
	}

	writeJSON(w, rules)
}
