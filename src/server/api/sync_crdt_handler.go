package api

import (
	"encoding/json"
	"log/slog"
	"net/http"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/orchestration"
	"go.opentelemetry.io/otel"
)

type SyncDeltasPayload struct {
	Deltas []struct {
		ID        string `json:"id"`
		EntityID  string `json:"entity_id"`
		Data      string `json:"data"`
		UpdatedAt string `json:"updated_at"`
	} `json:"deltas"`
}

// HandleSyncMCPDeltas handles receiving CRDT deltas from local standalone clients.
func HandleSyncMCPDeltas(hub *orchestration.Hub) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx, span := otel.Tracer("github.com/onehumancorp/mono/src/server/api").Start(r.Context(), "HandleSyncMCPDeltas")
		defer span.End()

		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		tenantID := r.Header.Get("X-Tenant-ID")
		if tenantID == "" {
			claims := auth.ClaimsFromContext(ctx)
			if claims != nil && claims.OrganizationID != "" {
				tenantID = claims.OrganizationID
			} else {
				http.Error(w, "missing tenant_id", http.StatusBadRequest)
				return
			}
		}

		r.Body = http.MaxBytesReader(w, r.Body, 5<<20)

		var payload SyncDeltasPayload
		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}

		if len(payload.Deltas) == 0 {
			writeJSON(w, map[string]string{"status": "success", "message": "no deltas to sync"})
			return
		}

		syncedCount := 0
		sipDB := hub.SIPDB()
		if sipDB != nil && sipDB.Provider() != nil {
			provider := sipDB.Provider()
			for _, delta := range payload.Deltas {
				if delta.ID == "" || delta.EntityID == "" || delta.Data == "" || delta.UpdatedAt == "" {
					continue
				}

				query := `INSERT INTO crdt_deltas (tenant_id, id, entity_id, data, updated_at, synced_to_cloud)
				          VALUES ($1, $2, $3, $4, $5, true)
				          ON CONFLICT(tenant_id, id) DO UPDATE SET
				          data = excluded.data, updated_at = excluded.updated_at, synced_to_cloud = true`

				_, err := provider.Exec(ctx, query, tenantID, delta.ID, delta.EntityID, delta.Data, delta.UpdatedAt)
				if err != nil {
					slog.Error("failed to upsert CRDT delta", "id", delta.ID, "error", err)
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
