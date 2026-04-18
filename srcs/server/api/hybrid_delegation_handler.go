package api

import (
	"github.com/google/uuid"
	"encoding/json"
	"log/slog"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"go.opentelemetry.io/otel"
)

type DelegationRequest struct {
	OriginalQuery string `json:"original_query"`
	Content       string `json:"content"`
}

type DelegationResponse struct {
	Status    string `json:"status"`
	MissionID string `json:"mission_id"`
}

func HandleHybridDelegation(hub *orchestration.Hub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/api").Start(r.Context(), "HandleHybridDelegation")
		defer span.End()

		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		r.Body = http.MaxBytesReader(w, r.Body, 5<<20)

		var reqPayload DelegationRequest
		if err := json.NewDecoder(r.Body).Decode(&reqPayload); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}

		if reqPayload.OriginalQuery == "" {
			http.Error(w, "original_query is required", http.StatusBadRequest)
			return
		}

		missionData, _ := json.Marshal(map[string]interface{}{
			"role":  "delegated_cloud_agent",
			"query": reqPayload.OriginalQuery,
			"context": reqPayload.Content,
		})

		missionID := uuid.New().String()

		if hub.SIPDB() == nil {
			http.Error(w, "SIPDB not initialized", http.StatusInternalServerError)
			return
		}

		err := hub.SIPDB().UpsertMission(ctx, missionID, "PENDING", string(missionData), false)
		if err != nil {
			slog.Error("failed to create mission from delegation", "id", missionID, "error", err)
			http.Error(w, "failed to delegate mission", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(DelegationResponse{
			Status:    "success",
			MissionID: missionID,
		})
	}
}
