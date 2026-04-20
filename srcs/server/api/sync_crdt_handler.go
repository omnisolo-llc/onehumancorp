package api

import (
	"encoding/json"
	"log/slog"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"go.opentelemetry.io/otel"
)

type CRDTDelta struct {
	ID        string `json:"id"`
	EntityID  string `json:"entity_id"`
	Data      string `json:"data"`
	UpdatedAt string `json:"updated_at"`
}

func HandleCRDTSync(hub *orchestration.Hub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/api").Start(r.Context(), "HandleCRDTSync")
		defer span.End()

		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		r.Body = http.MaxBytesReader(w, r.Body, 5<<20) // 5MB limit

		var payload struct {
			Deltas []CRDTDelta `json:"deltas"`
		}

		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}

		if len(payload.Deltas) == 0 {
			writeJSON(w, map[string]interface{}{"status": "success", "message": "no deltas to sync"})
			return
		}

		syncedCount := 0
		if hub.SIPDB() != nil {
			for _, delta := range payload.Deltas {
				updatedAt, err := time.Parse(time.RFC3339, delta.UpdatedAt)
				if err != nil {
					slog.Error("failed to parse updated_at", "error", err)
					continue
				}

				_, err = hub.SIPDB().Provider().Exec(ctx,
					"INSERT INTO crdt_deltas (id, entity_id, data, updated_at, synced_to_cloud) VALUES ($1, $2, $3, $4, true) ON CONFLICT(id) DO UPDATE SET data = EXCLUDED.data, updated_at = EXCLUDED.updated_at, synced_to_cloud = true WHERE EXCLUDED.updated_at > crdt_deltas.updated_at",
					delta.ID, delta.EntityID, delta.Data, updatedAt)
				if err != nil {
					slog.Error("failed to sync crdt delta", "id", delta.ID, "error", err)
				} else {
					syncedCount++
				}
			}
		}

		writeJSON(w, map[string]interface{}{
			"status":       "success",
			"message":      "deltas synced successfully",
			"synced_count": syncedCount,
		})
	}
}

func HandleCRDTPull(hub *orchestration.Hub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/api").Start(r.Context(), "HandleCRDTPull")
		defer span.End()

		if r.Method != http.MethodGet {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var deltas []CRDTDelta

		if hub.SIPDB() != nil {

			// Support optional query parameter for pagination
			after := r.URL.Query().Get("after")
			var rows db.Rows
			var err error

			if after != "" {
				parsedTime, timeErr := time.Parse(time.RFC3339, after)
				if timeErr == nil {
					rows, err = hub.SIPDB().Provider().Query(ctx, "SELECT id, entity_id, data, updated_at FROM crdt_deltas WHERE updated_at > $1 ORDER BY updated_at ASC LIMIT 100", parsedTime)
				} else {
					rows, err = hub.SIPDB().Provider().Query(ctx, "SELECT id, entity_id, data, updated_at FROM crdt_deltas ORDER BY updated_at ASC LIMIT 100")
				}
			} else {
				rows, err = hub.SIPDB().Provider().Query(ctx, "SELECT id, entity_id, data, updated_at FROM crdt_deltas ORDER BY updated_at ASC LIMIT 100")
			}
			if err != nil {
				slog.Error("failed to query crdt deltas", "error", err)
				http.Error(w, "internal server error", http.StatusInternalServerError)
				return
			}
			defer rows.Close()

			for rows.Next() {
				var id, entityID, data string
				var updatedAt time.Time
				if err := rows.Scan(&id, &entityID, &data, &updatedAt); err != nil {
					slog.Error("failed to scan crdt delta", "error", err)
					continue
				}
				deltas = append(deltas, CRDTDelta{
					ID:        id,
					EntityID:  entityID,
					Data:      data,
					UpdatedAt: updatedAt.Format(time.RFC3339),
				})
			}
		}

		writeJSON(w, map[string]interface{}{
			"status": "success",
			"deltas": deltas,
		})
	}
}
