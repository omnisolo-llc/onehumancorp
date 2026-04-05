package dashboard

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func (s *Server) handleTelemetrySync(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var batch []struct {
		MetricType string `json:"metric_type"`
		Payload    string `json:"payload"`
	}

	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 5<<20)).Decode(&batch); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	for _, item := range batch {
		// Re-record the buffered metrics
		var payloadMap map[string]interface{}
		if err := json.Unmarshal([]byte(item.Payload), &payloadMap); err != nil {
			continue // Skip malformed payloads
		}

		ctx := r.Context()

		switch item.MetricType {
		case "token_usage":
			agentID, _ := payloadMap["agent_id"].(string)
			role, _ := payloadMap["role"].(string)
			model, _ := payloadMap["model"].(string)
			tokenType, _ := payloadMap["type"].(string)
			countFloat, _ := payloadMap["count"].(float64)
			telemetry.RecordTokenUsage(ctx, agentID, role, model, tokenType, int64(countFloat))
		case "agent_api_call":
			agentID, _ := payloadMap["agent_id"].(string)
			role, _ := payloadMap["role"].(string)
			api, _ := payloadMap["api"].(string)
			telemetry.RecordAgentApiCall(ctx, agentID, role, api)
		case "agent_api_error":
			agentID, _ := payloadMap["agent_id"].(string)
			role, _ := payloadMap["role"].(string)
			api, _ := payloadMap["api"].(string)
			telemetry.RecordAgentApiError(ctx, agentID, role, api)
		case "human_interaction":
			interactionType, _ := payloadMap["type"].(string)
			telemetry.RecordHumanInteraction(ctx, interactionType)
		case "meeting_event":
			eventType, _ := payloadMap["type"].(string)
			telemetry.RecordMeetingEvent(ctx, eventType)
		case "swarm_task_completed":
			missionID, _ := payloadMap["mission_id"].(string)
			telemetry.RecordSwarmTaskCompleted(ctx, missionID)
		}
	}

	writeJSON(w, map[string]interface{}{"status": "ok", "synced": len(batch)})
}
