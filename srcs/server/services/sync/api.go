package sync

import (
	"encoding/json"
	"net/http"
)

type APIHandler struct {
	escalator *Escalator
}

func NewAPIHandler(e *Escalator) *APIHandler {
	return &APIHandler{escalator: e}
}

type EscalateRequest struct {
	TaskID string `json:"task_id"`
}

func (h *APIHandler) HandleEscalate(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req EscalateRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request payload", http.StatusBadRequest)
		return
	}

	if req.TaskID == "" {
		http.Error(w, "task_id is required", http.StatusBadRequest)
		return
	}

	// For the sake of the API, we simply queue it by updating local DB status.
	// The daemon will pick it up on its next tick.
	_, err := h.escalator.db.ExecContext(r.Context(), "UPDATE local_mcp_rag_tasks SET escalation_status = 'pending_escalation', updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND escalation_status = 'local'", req.TaskID)
	if err != nil {
		http.Error(w, "Failed to update task status", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusAccepted)
	json.NewEncoder(w).Encode(map[string]string{
		"status": "escalation_queued",
		"task_id": req.TaskID,
	})
}
