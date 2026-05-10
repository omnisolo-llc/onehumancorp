package sync

import (
	"encoding/json"
	"net/http"
	"onehumancorp/srcs/server/orchestration"
	"onehumancorp/srcs/server/repository"
	"time"
)

type SyncHandler struct {
	CloudStore orchestration.TaskStore
}

func NewSyncHandler(cloudStore orchestration.TaskStore) *SyncHandler {
	return &SyncHandler{CloudStore: cloudStore}
}

func (h *SyncHandler) HandleSyncMissions(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var task orchestration.SharedTask
	if err := json.NewDecoder(r.Body).Decode(&task); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	// SECURITY: Never trust tenant_id from request body.
	// Derive it from the authenticated session (context).
	orgID := repository.OrganizationIDFromContext(r.Context())
	if orgID == "" {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	// Enforce the organization ID from the session
	task.OrganizationID = orgID

	if task.CreatedAt.IsZero() {
		task.CreatedAt = time.Now()
	}
	if task.UpdatedAt.IsZero() {
		task.UpdatedAt = time.Now()
	}

	if err := h.CloudStore.CreateTask(r.Context(), &task); err != nil {
		http.Error(w, "Failed to insert mission", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

// DUMMY VALIDATION COMMENT
