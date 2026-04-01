package powersync

import (
	"encoding/json"
	"net/http"
)

// SyncRulesResponse represents the payload required by JourneyApps PowerSync
// for evaluating sync rules dynamically.
type SyncRulesResponse struct {
	BucketData map[string]interface{} `json:"bucket_data"`
}

// RulesHandler returns the PowerSync sync rules, filtering based on the tenant/organization.
func RulesHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// In a real application, extract tenant ID from the token claims context.
		tenantID := r.Header.Get("X-Tenant-ID")

		if tenantID == "" {
			tenantID = "default_tenant"
		}

		// Strictly use the JourneyApps bucket_data schema for sync rules.
		rules := SyncRulesResponse{
			BucketData: map[string]interface{}{
				"tenant_data": map[string]interface{}{
					"data": []map[string]interface{}{
						{
							"query": "SELECT * FROM agent_missions WHERE tenant_id = $1",
							"args":  []interface{}{tenantID},
						},
						{
							"query": "SELECT * FROM users WHERE organization_id = $1",
							"args":  []interface{}{tenantID},
						},
						{
							"query": "SELECT * FROM meetings WHERE organization_id = $1",
							"args":  []interface{}{tenantID},
						},
					},
				},
			},
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(rules)
	}
}
