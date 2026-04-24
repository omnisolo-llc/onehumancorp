package api

import (
	"encoding/json"
	"log/slog"
	"net/http"

	"github.com/onehumancorp/mono/src/server/orchestration"
	"github.com/onehumancorp/mono/src/server/orchestration/hybrid_sync"
	"github.com/onehumancorp/mono/src/server/orchestration/queue"
	"go.opentelemetry.io/otel"
)

func HandleSyncEscalation(hub *orchestration.Hub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx, span := otel.Tracer("github.com/onehumancorp/mono/src/server/api").Start(r.Context(), "HandleSyncEscalation")
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
		if hub.TaskManager() != nil {
			for _, p := range payloads {
				if p.MemoryID == "" {
					continue
				}

				job := &queue.Job{
					ID:           p.MemoryID,
					ParentTaskID: "escalation",
					AgentRole:    "SYSTEM",
					Payload:      p.Context,
				}

				// We need to enqueue this to the TaskQueue which we get from the provider if we can't directly.
				// However, if we don't have access to tm.taskQueue directly because it's unexported:
				// we could use a custom postgres_queue or enqueue directly to the database here, since we have the DB.

				// Let's create a Postgres queue right here with hub.SIPDB().Provider() if available.
				if hub.SIPDB() != nil {
					q := queue.NewPostgresTaskQueue(hub.SIPDB().Provider())
					if err := q.Enqueue(ctx, job); err != nil {
						slog.Error("failed to enqueue escalation job", "memory_id", p.MemoryID, "error", err)
					} else {
						syncedCount++
					}
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
