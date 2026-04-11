package api

import (
	"encoding/json"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

func HandleRAGSync(ragSyncService hub.RAGSyncService) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		ctx := r.Context()

		if r.Method != http.MethodPost {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}

		r.Body = http.MaxBytesReader(w, r.Body, 5<<20)

		var payload struct {
			Records []struct {
				ID         string    `json:"id"`
				Context    string    `json:"context"`
				LastSyncAt time.Time `json:"last_sync_at"`
			} `json:"records"`
		}

		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}

		var records []hub.RAGSyncRecord
		for _, p := range payload.Records {
			records = append(records, hub.RAGSyncRecord{
				ID:         p.ID,
				Context:    p.Context,
				SyncStatus: hub.SyncStatusSynced,
				LastSyncAt: time.Now(),
			})
		}

		if err := ragSyncService.ProcessIncomingSync(ctx, records); err != nil {
			http.Error(w, "failed to process sync", http.StatusInternalServerError)
			return
		}

		writeJSON(w, map[string]interface{}{
			"status": "success",
			"synced": len(records),
		})
	}
}
