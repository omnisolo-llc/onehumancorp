package webhooks

import (
	"encoding/json"
	"net/http"
	"bytes"
	"io"
)

// WebhookPayload represents the expected input for a webhook, keeping JSON raw
type WebhookPayload struct {
	AgentID *string `json:"agent_id"`
	Action  *string `json:"action"`
	Status  *string `json:"status"`
}

// Receiver validates incoming webhook requests for OHC-SIP compliance.
func Receiver(w http.ResponseWriter, r *http.Request) {
	// Limit payload to 1MB to prevent memory exhaustion
	bodyBytes, err := io.ReadAll(io.LimitReader(r.Body, 1024*1024))
	if err != nil {
		http.Error(w, "Internal server error", http.StatusInternalServerError)
		return
	}
	r.Body = io.NopCloser(bytes.NewBuffer(bodyBytes))

	var payload WebhookPayload
	if err := json.Unmarshal(bodyBytes, &payload); err != nil {
		http.Error(w, "Bad Request: invalid JSON", http.StatusBadRequest)
		return
	}

	if payload.AgentID == nil || payload.Action == nil || payload.Status == nil {
		http.Error(w, "Bad Request: missing required OHC-SIP fields (agent_id, action, status)", http.StatusBadRequest)
		return
	}

	w.WriteHeader(http.StatusOK)
}
