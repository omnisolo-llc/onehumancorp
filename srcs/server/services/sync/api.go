package sync

import (
	"context"
	"encoding/json"
	"net/http"
)

type EscalateRequest struct {
	TaskID string `json:"task_id"`
}

type EscalateResponse struct {
	Status string `json:"status"`
}

func (e *Escalator) HandleEscalate(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req EscalateRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	err := e.EscalateTask(context.Background(), req.TaskID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	resp := EscalateResponse{Status: "escalated"}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}
