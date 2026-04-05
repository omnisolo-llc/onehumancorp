package dashboard

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
)

type syncedMetric struct {
	MetricType string `json:"metric_type"`
	AgentID    string `json:"agent_id,omitempty"`
	Role       string `json:"role,omitempty"`
	API        string `json:"api,omitempty"`
	Model      string `json:"model,omitempty"`
	Type       string `json:"type,omitempty"`
	Count      int64  `json:"count,omitempty"`
	MissionID  string `json:"mission_id,omitempty"`
}

func (s *Server) handleTelemetrySync(w http.ResponseWriter, r *http.Request) {
	ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/dashboard").Start(r.Context(), "handleTelemetrySync")
	defer span.End()

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	r.Body = http.MaxBytesReader(w, r.Body, 5<<20)

	var metrics []map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&metrics); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	for _, m := range metrics {
		metricType, _ := m["metric_type"].(string)

		switch metricType {
		case "token_usage":
			agentID, _ := m["agent_id"].(string)
			role, _ := m["role"].(string)
			model, _ := m["model"].(string)
			tokenType, _ := m["type"].(string)
			var count int64
			if c, ok := m["count"].(float64); ok {
				count = int64(c)
			}
			telemetry.RecordTokenUsage(ctx, agentID, role, model, tokenType, count)
		case "agent_api_call":
			agentID, _ := m["agent_id"].(string)
			role, _ := m["role"].(string)
			api, _ := m["api"].(string)
			telemetry.RecordAgentApiCall(ctx, agentID, role, api)
		case "agent_api_error":
			agentID, _ := m["agent_id"].(string)
			role, _ := m["role"].(string)
			api, _ := m["api"].(string)
			telemetry.RecordAgentApiError(ctx, agentID, role, api)
		case "human_interaction":
			t, _ := m["type"].(string)
			telemetry.RecordHumanInteraction(ctx, t)
		case "meeting_event":
			t, _ := m["type"].(string)
			telemetry.RecordMeetingEvent(ctx, t)
		case "swarm_task_completed":
			missionID, _ := m["mission_id"].(string)
			telemetry.RecordSwarmTaskCompleted(ctx, missionID)
		}
	}

	writeJSON(w, map[string]string{"status": "success", "message": "metrics ingested"})
}
