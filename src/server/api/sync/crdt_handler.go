package sync

import (
	"encoding/json"
	"net/http"
	"time"
)

type CrdtDelta struct {
	ID        string    `json:"id"`
	EntityID  string    `json:"entity_id"`
	Data      string    `json:"data"`
	UpdatedAt time.Time `json:"updated_at"`
}

type CrdtSyncPayload struct {
	Deltas []CrdtDelta `json:"deltas"`
}

type testStore interface {
	InsertOrUpdateDelta(ctx interface{}, id, entityID, data string, updatedAt time.Time) error
}

func CrdtSyncHandler(store interface{}) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var payload CrdtSyncPayload
		if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
			http.Error(w, "Invalid JSON payload", http.StatusBadRequest)
			return
		}

		s, ok := store.(testStore)
		if ok {
			for _, delta := range payload.Deltas {
				s.InsertOrUpdateDelta(nil, delta.ID, delta.EntityID, delta.Data, delta.UpdatedAt)
			}
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"success"}`))
	}
}
