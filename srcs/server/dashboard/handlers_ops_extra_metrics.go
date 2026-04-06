package dashboard

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func (s *Server) handleTelemetrySync(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var batch []struct {
		ID         int64  `json:"id"`
		MetricType string `json:"metric_type"`
		Payload    string `json:"payload"`
	}
	if err := json.NewDecoder(r.Body).Decode(&batch); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	for _, metric := range batch {
		if telemetry.SyncCompletedCount != nil {
			telemetry.SyncCompletedCount.Add(r.Context(), 1)
		}

		switch metric.MetricType {
		case "token_usage":
			var p struct {
				AgentID string `json:"agent_id"`
				Role    string `json:"role"`
				Model   string `json:"model"`
				Type    string `json:"type"`
				Count   int64  `json:"count"`
			}
			if err := json.Unmarshal([]byte(metric.Payload), &p); err == nil {
				telemetry.RecordTokenUsage(r.Context(), p.AgentID, p.Role, p.Model, p.Type, p.Count)
			}
		case "agent_api_call":
			var p struct {
				AgentID string `json:"agent_id"`
				Role    string `json:"role"`
				API     string `json:"api"`
			}
			if err := json.Unmarshal([]byte(metric.Payload), &p); err == nil {
				telemetry.RecordAgentApiCall(r.Context(), p.AgentID, p.Role, p.API)
			}
		case "agent_api_error":
			var p struct {
				AgentID string `json:"agent_id"`
				Role    string `json:"role"`
				API     string `json:"api"`
			}
			if err := json.Unmarshal([]byte(metric.Payload), &p); err == nil {
				telemetry.RecordAgentApiError(r.Context(), p.AgentID, p.Role, p.API)
			}
		case "human_interaction":
			var p struct {
				Type string `json:"type"`
			}
			if err := json.Unmarshal([]byte(metric.Payload), &p); err == nil {
				telemetry.RecordHumanInteraction(r.Context(), p.Type)
			}
		case "meeting_event":
			var p struct {
				Type string `json:"type"`
			}
			if err := json.Unmarshal([]byte(metric.Payload), &p); err == nil {
				telemetry.RecordMeetingEvent(r.Context(), p.Type)
			}
		case "swarm_task_completed":
			var p struct {
				MissionID string `json:"mission_id"`
			}
			if err := json.Unmarshal([]byte(metric.Payload), &p); err == nil {
				telemetry.RecordSwarmTaskCompleted(r.Context(), p.MissionID)
			}
		}
	}

	w.WriteHeader(http.StatusOK)
	_, _ = w.Write([]byte("ok"))
}
