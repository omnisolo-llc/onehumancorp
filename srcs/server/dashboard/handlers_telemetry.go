package dashboard

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type SyncMetricPayload struct {
	ID         int64  `json:"id"`
	MetricType string `json:"metric_type"`
	Payload    string `json:"payload"`
}

func (s *Server) handleTelemetrySync(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "forbidden: missing or invalid claims", http.StatusForbidden)
		return
	}

	var metrics []SyncMetricPayload
	if err := json.NewDecoder(r.Body).Decode(&metrics); err != nil {
		http.Error(w, "bad request: "+err.Error(), http.StatusBadRequest)
		return
	}

	// Insert synced metrics into the cloud telemetry_buffer or directly process them.
	// For now, we will simply accept them.
	for _, m := range metrics {
		_ = s.hub.SIPDB().BufferMetric(r.Context(), m.MetricType, m.Payload)
	}

	w.WriteHeader(http.StatusOK)
}
