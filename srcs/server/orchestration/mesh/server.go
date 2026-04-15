package mesh

import (
	"encoding/json"
	"net/http"
)

type MeshEvent struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
}

func BroadcastHandler(mesh TeammateMesh) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		var event MeshEvent
		if err := json.NewDecoder(r.Body).Decode(&event); err != nil {
			http.Error(w, "invalid payload", http.StatusBadRequest)
			return
		}

		if event.AgentID == "" || event.Action == "" || event.Status == "" {
			http.Error(w, "missing required fields", http.StatusBadRequest)
			return
		}

		payload, _ := json.Marshal(event)
		if err := mesh.Publish(r.Context(), "mesh_events", payload); err != nil {
			http.Error(w, "failed to broadcast", http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
	}
}
