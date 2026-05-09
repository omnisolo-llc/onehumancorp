package orchestration

import (
	"encoding/json"
	"net/http"
	"strings"
)

type TaskHandler struct {
	repo TaskRepository
	mesh MeshHub
}

func NewTaskHandler(repo TaskRepository, mesh MeshHub) *TaskHandler {
	return &TaskHandler{
		repo: repo,
		mesh: mesh,
	}
}

func (h *TaskHandler) HandleCreateTask(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Multi-Tenant Safety Check: Read tenant_id from session/context (using the existing onboarding context key logic)
	// Do not use the header directly here; we rely on the middleware.
	tenantID, ok := r.Context().Value("tenant_id").(string) // Reusing onboarding's context key string
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing tenant session", http.StatusUnauthorized)
		return
	}

	var task Task
	if err := json.NewDecoder(r.Body).Decode(&task); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	task.TenantID = tenantID // Enforce tenant isolation

	if err := h.repo.CreateTask(r.Context(), &task); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	// Publish event to Teammate Mesh
	if h.mesh != nil {
		eventData, _ := json.Marshal(map[string]interface{}{
			"type": "task_created",
			"task": task,
		})
		_ = h.mesh.Publish(r.Context(), "ohc:tasks:events", eventData)
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(task)
}

func (h *TaskHandler) HandleListTasks(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	tenantID, ok := r.Context().Value("tenant_id").(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing tenant session", http.StatusUnauthorized)
		return
	}

	tasks, err := h.repo.ListTasks(r.Context(), tenantID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(tasks)
}

func (h *TaskHandler) HandleUpdateTask(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPut {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	tenantID, ok := r.Context().Value("tenant_id").(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing tenant session", http.StatusUnauthorized)
		return
	}

	pathParts := strings.Split(r.URL.Path, "/")
	if len(pathParts) < 5 { // e.g. /api/tasks/update/{id}
		http.Error(w, "Missing task ID in URL", http.StatusBadRequest)
		return
	}
	taskID := pathParts[4]

	// Verify tenant ownership via scoped query
	existingTask, err := h.repo.GetTask(r.Context(), tenantID, taskID)
	if err != nil {
		http.Error(w, "Task not found", http.StatusNotFound)
		return
	}

	var taskUpdate Task
	if err := json.NewDecoder(r.Body).Decode(&taskUpdate); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	// Preserve critical IDs
	taskUpdate.ID = existingTask.ID
	taskUpdate.TenantID = existingTask.TenantID

	if err := h.repo.UpdateTask(r.Context(), &taskUpdate); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	// Publish event to Teammate Mesh
	if h.mesh != nil {
		eventData, _ := json.Marshal(map[string]interface{}{
			"type": "task_updated",
			"task": taskUpdate,
		})
		_ = h.mesh.Publish(r.Context(), "ohc:tasks:events", eventData)
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(taskUpdate)
}