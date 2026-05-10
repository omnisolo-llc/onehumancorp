package mesh

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
)

type meshPayload struct {
	AgentID   string          `json:"agent_id"`
	Channel   string          `json:"channel"`
	EventType string          `json:"event_type"`
	Data      json.RawMessage `json:"data"`
}

func ValidationMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method == "POST" && r.URL.Path == "/api/mesh/broadcast" {
			bodyBytes, err := io.ReadAll(r.Body)
			if err != nil {
				http.Error(w, "Failed to read request body", http.StatusInternalServerError)
				return
			}
			r.Body = io.NopCloser(bytes.NewBuffer(bodyBytes))

			var payload map[string]interface{}
			if err := json.Unmarshal(bodyBytes, &payload); err != nil {
				http.Error(w, "Invalid JSON payload", http.StatusBadRequest)
				return
			}

			// Reject deprecated keys
			if _, hasAction := payload["action"]; hasAction {
				http.Error(w, "Deprecated key 'action' found in payload", http.StatusBadRequest)
				return
			}
			if _, hasStatus := payload["status"]; hasStatus {
				http.Error(w, "Deprecated key 'status' found in payload", http.StatusBadRequest)
				return
			}

			// Enforce strict quad-key requirement
			requiredKeys := []string{"agent_id", "channel", "event_type", "data"}
			for _, key := range requiredKeys {
				if _, ok := payload[key]; !ok {
					http.Error(w, "Missing required OHC-SIP field: "+key, http.StatusBadRequest)
					return
				}
			}

			// Verify they are the correct type if present via our struct mapping
			var strictPayload meshPayload
			if err := json.Unmarshal(bodyBytes, &strictPayload); err != nil {
				http.Error(w, "Invalid field types in payload", http.StatusBadRequest)
				return
			}

			if strictPayload.AgentID == "" || strictPayload.Channel == "" || strictPayload.EventType == "" || len(strictPayload.Data) == 0 {
				http.Error(w, "OHC-SIP fields cannot be empty strings or null", http.StatusBadRequest)
				return
			}

		}

		next.ServeHTTP(w, r)
	})
}
