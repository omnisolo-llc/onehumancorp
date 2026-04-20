package api

import (
	"encoding/json"
	"log/slog"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
)

// HandleHybridSyncMissions handles receiving synced local missions from HybridMCPRAGDaemon.
func HandleHybridSyncMissions(hub *orchestration.Hub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/api").Start(r.Context(), "HandleHybridSyncMissions")
		defer span.End()

		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		r.Body = http.MaxBytesReader(w, r.Body, 5<<20) // 5MB limit

		var payloads []struct {
			ID      string `json:"id"`
			Status  string `json:"status"`
			Payload string `json:"payload"`
		}

		if err := json.NewDecoder(r.Body).Decode(&payloads); err != nil {
			http.Error(w, "invalid JSON payload array", http.StatusBadRequest)
			return
		}

		if len(payloads) == 0 {
			writeJSON(w, map[string]string{"status": "success", "message": "no missions to sync"})
			return
		}

		for i := range payloads {
			var parsedPayload interface{}
			if err := json.Unmarshal([]byte(payloads[i].Payload), &parsedPayload); err == nil {
				redactedPayload := telemetry.RedactInterfacePII(parsedPayload)
				if redactedBytes, err := json.Marshal(redactedPayload); err == nil {
					payloads[i].Payload = string(redactedBytes)
				}
			}
		}

		syncedCount := 0
		for _, p := range payloads {
			if p.ID == "" {
				continue // Skip invalid items
			}

			status := p.Status
			if status == "" {
				status = "PENDING"
			}

			forceLocal := r.Header.Get("X-OHC-Conflict-Resolution") == "force-local"

			// Use the UpsertMission method to store in Postgres
			if hub.SIPDB() != nil {
				err := hub.SIPDB().UpsertMission(ctx, p.ID, status, p.Payload, forceLocal)
				if err != nil {
					slog.Error("failed to upsert mission from sync daemon", "id", p.ID, "error", err)
					// continue syncing the rest
				} else {
					syncedCount++
					telemetry.RecordSyncConflictResolved(ctx)
					telemetry.RecordOmniContextBytes(ctx, int64(len(p.Payload)))

					// Publish to Teammate Mesh
					if cnNode := hub.CentrifugeNode(); cnNode != nil {
						var payloadMap map[string]interface{}
						if err := json.Unmarshal([]byte(p.Payload), &payloadMap); err != nil {
							payloadMap = map[string]interface{}{}
						}
						payloadMap["status"] = status
						cnNode.PublishTaskBroadcast(p.ID, payloadMap)
					}
				}
			}
		}

		writeJSON(w, map[string]interface{}{
			"status":       "success",
			"message":      "missions synced successfully",
			"synced_count": syncedCount,
		})
	}
}

func writeJSON(w http.ResponseWriter, data interface{}) {
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(data)
}
