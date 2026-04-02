package api

import (
	"encoding/json"
	"net/http"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func RegisterSyncRulesHandlers(mux *http.ServeMux) {
	mux.HandleFunc("/api/sync_rules", handleSyncRules)
}

func handleSyncRules(w http.ResponseWriter, r *http.Request) {
	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"

	if isMultiTenant {
		orgID := auth.OrganizationIDFromContext(r.Context())
		if orgID == "" {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		// Enforce strict multi-tenant isolation by applying WHERE clauses
		// matched to the user's JWT claims (e.g., token.jwt_ext_organization_id
		// or token_parameters.claim_name).
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"rules": []map[string]interface{}{
				{
					"name":  "agent_missions",
					"query": "SELECT * FROM agent_missions WHERE organization_id = request.jwt.claims ->> 'organization_id'",
				},
				{
					"name":  "swarm_memory",
					"query": "SELECT * FROM swarm_memory WHERE organization_id = request.jwt.claims ->> 'organization_id'",
				},
			},
		})
	} else {
		// Single tenant, no constraints
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"rules": []map[string]interface{}{
				{
					"name":  "agent_missions",
					"query": "SELECT * FROM agent_missions",
				},
				{
					"name":  "swarm_memory",
					"query": "SELECT * FROM swarm_memory",
				},
			},
		})
	}
}
