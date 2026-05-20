package orchestration

import (
	"encoding/json"
	"net/http"
	"github.com/gorilla/mux"
)

// TaskHandler provides HTTP endpoints for tasks.
type TaskHandler struct {
	repo     *TaskRepository
	mesh     TeammateMesh
}

// NewTaskHandler creates a new TaskHandler.
func NewTaskHandler(repo *TaskRepository, mesh TeammateMesh) *TaskHandler {
	return &TaskHandler{repo: repo, mesh: mesh}
}

// CreateTask handles creating a new task.
func (h *TaskHandler) CreateTask(w http.ResponseWriter, r *http.Request) {
	var task Task
	if err := json.NewDecoder(r.Body).Decode(&task); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if err := h.repo.CreateTask(r.Context(), &task); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	// Publish event to Teammate Mesh
	if h.mesh != nil {
		eventData, _ := json.Marshal(task)
		h.mesh.Publish("ohc:tasks:events", eventData)
	}

	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(task)
}

// ListTasks handles listing tasks.
func (h *TaskHandler) ListTasks(w http.ResponseWriter, r *http.Request) {
	tasks, err := h.repo.ListTasks(r.Context())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(tasks)
}

// ClaimTask handles claiming a task.
func (h *TaskHandler) ClaimTask(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	taskID := vars["id"]
    if taskID == "" {
        http.Error(w, "id parameter is required", http.StatusBadRequest)
        return
    }

	var payload struct {
		AgentID string `json:"agent_id"`
	}
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	claimed, err := h.repo.ClaimTask(r.Context(), taskID, payload.AgentID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	if !claimed {
		http.Error(w, "Task already claimed or not pending", http.StatusConflict)
		return
	}

	// Publish event to Teammate Mesh
	if h.mesh != nil {
		eventData, _ := json.Marshal(map[string]string{
			"task_id":  taskID,
			"agent_id": payload.AgentID,
			"status":   "IN_PROGRESS",
		})
		h.mesh.Publish("ohc:tasks:events", eventData)
	}

	w.WriteHeader(http.StatusOK)
}

// TeammateMesh is the interface for event broadcasting.
type TeammateMesh interface {
	Publish(channel string, payload []byte) error
}

// RedisMesh is the Redis-backed version for Cloud mode.
type RedisMesh struct {
	// Add Redis client connection here
}

func (m *RedisMesh) Publish(channel string, payload []byte) error {
	// Implement Redis publish logic here
	return nil
}

// LocalMesh is the no-op/local version for Standalone mode.
type LocalMesh struct{}

func (m *LocalMesh) Publish(channel string, payload []byte) error {
	// Implement local event bus logic or no-op here
	return nil
}

func (h *TaskHandler) RegisterRoutes(router *mux.Router) {
	router.HandleFunc("/tasks", h.CreateTask).Methods("POST")
	router.HandleFunc("/tasks", h.ListTasks).Methods("GET")
	router.HandleFunc("/tasks/{id}/claim", h.ClaimTask).Methods("POST")
}
