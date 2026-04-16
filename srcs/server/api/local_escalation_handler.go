package api

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/services/sync"
)

type LocalEscalateRequest struct {
	TaskID string `json:"task_id"`
}

type LocalEscalateResponse struct {
	Status string `json:"status"`
}

func HandleLocalEscalate(escalator *sync.Escalator) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		var req LocalEscalateRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "invalid request body", http.StatusBadRequest)
			return
		}

		if err := escalator.EscalateTask(r.Context(), req.TaskID); err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(LocalEscalateResponse{Status: "success"})
	}
}
