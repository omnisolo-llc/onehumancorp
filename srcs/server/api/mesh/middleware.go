package mesh

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
)

// meshPayload is used to verify OHC-SIP compliance.
type meshPayload struct {
	AgentID *string `json:"agent_id"`
	Action  *string `json:"action"`
	Status  *string `json:"status"`
}

// ValidationMiddleware enforces OHC-SIP compliance for Teammate Mesh unified gateway requests.
// It verifies that agent_id, action, and status exist at the root of the JSON payload.
func ValidationMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Only validate POST requests (like POST /api/mesh/broadcast)
		if r.Method != http.MethodPost {
			next.ServeHTTP(w, r)
			return
		}

		bodyBytes, err := io.ReadAll(r.Body)
		if err != nil {
			http.Error(w, "failed to read body", http.StatusInternalServerError)
			return
		}

		// Restore the body for downstream handlers
		r.Body = io.NopCloser(bytes.NewBuffer(bodyBytes))

		var payload meshPayload
		if err := json.Unmarshal(bodyBytes, &payload); err != nil {
			http.Error(w, "invalid json payload", http.StatusBadRequest)
			return
		}

		if payload.AgentID == nil || payload.Action == nil || payload.Status == nil {
			http.Error(w, "OHC-SIP compliance failed: missing agent_id, action, or status", http.StatusBadRequest)
			return
		}

		next.ServeHTTP(w, r)
	})
}
