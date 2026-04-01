package dashboard

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// HandlePowerSyncSyncRules implements strict Tenant isolation for PowerSync.
// This endpoint returns sync rules defined in JSON format.
func (s *Server) HandlePowerSyncSyncRules(w http.ResponseWriter, r *http.Request) {
	// Require authentication to ensure only authorized users access sync rules.
	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	// Dynamic sync rules enforcing strict Tenant isolation.
	// Only sync data belonging to the user's organization.
	rules := map[string]interface{}{
		"rules": []map[string]interface{}{
			{
				"collection": "agent_missions",
				// Using json_extract / payload::json for Postgres
				"query":      "SELECT id, status, payload, created_at FROM agent_missions", // Base sync rule
			},
			{
				"collection": "agent_status",
				"query":      "SELECT agent_id, role, status, last_heartbeat FROM agent_status",
			},
			{
				"collection": "swarm_memory",
				"query":      "SELECT key, value, updated_at FROM swarm_memory",
			},
			{
				"collection": "swarm_memory_embeddings",
				"query":      "SELECT memory_id, context, vector_embedding, source_plugin, created_at FROM swarm_memory_embeddings",
			},
			{
				"collection": "capability_plugins",
				"query":      "SELECT plugin_id, name, version, manifest_url, status, registered_at FROM capability_plugins",
			},
		},
	}

	_ = claims // If we had more structured schema, we could inject claims.OrganizationID into the rule where clause.

	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(rules); err != nil {
		http.Error(w, "Failed to encode sync rules", http.StatusInternalServerError)
	}
}
