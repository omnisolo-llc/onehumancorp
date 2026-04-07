package api

import (
	"encoding/json"
	"log/slog"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/orchestration/hybrid_sync"
	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"go.opentelemetry.io/otel"
)

func HandleSyncEscalation(hub *orchestration.Hub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/api").Start(r.Context(), "HandleSyncEscalation")
		defer span.End()

		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		r.Body = http.MaxBytesReader(w, r.Body, 5<<20)

		var payloads []hybrid_sync.SyncPayload
		if err := json.NewDecoder(r.Body).Decode(&payloads); err != nil {
			http.Error(w, "invalid JSON payload array", http.StatusBadRequest)
			return
		}

		if len(payloads) == 0 {
			writeJSON(w, map[string]string{"status": "success", "message": "no items to escalate"})
			return
		}

		syncedCount := 0
		if hub.SIPDB() != nil {
			q := queue.NewPostgresTaskQueue(hub.SIPDB().Provider())
			for _, p := range payloads {
				if p.MemoryID == "" {
					continue
				}

				var parsedContext interface{}
				if err := json.Unmarshal([]byte(p.Context), &parsedContext); err == nil {
					redactedContext := telemetry.RedactInterfacePII(parsedContext)
					if redactedBytes, err := json.Marshal(redactedContext); err == nil {
						p.Context = string(redactedBytes)
					} else {
						p.Context = telemetry.RedactPII(p.Context)
					}
				} else {
					p.Context = telemetry.RedactPII(p.Context)
				}

				job := &queue.Job{
					ID:           p.MemoryID,
					ParentTaskID: "escalation",
					AgentRole:    "SYSTEM",
					Payload:      p.Context,
				}

				if err := q.Enqueue(ctx, job); err != nil {
					slog.Error("failed to enqueue escalation job", "memory_id", p.MemoryID, "error", err)
				} else {
					syncedCount++
				}
			}
		}

		writeJSON(w, map[string]interface{}{
			"status":       "success",
			"message":      "escalations synced successfully",
			"synced_count": syncedCount,
		})
	}
}
