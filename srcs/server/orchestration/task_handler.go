package orchestration

import (
	"encoding/json"
	"net/http"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type TaskHandler struct {
	repo *TaskRepository
	mesh MeshTransport
}

func NewTaskHandler(database db.Provider, mesh MeshTransport) *TaskHandler {
	return &TaskHandler{
		repo: NewTaskRepository(database),
		mesh: mesh,
	}
}

func (h *TaskHandler) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/api/tasks/shared", h.handleTasks)
	mux.HandleFunc("/api/tasks/shared/claim", h.handleClaimTask)
}

func (h *TaskHandler) handleTasks(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		h.listTasks(w, r)
	case http.MethodPost:
		h.createTask(w, r)
	default:
		w.WriteHeader(http.StatusMethodNotAllowed)
	}
}

func (h *TaskHandler) listTasks(w http.ResponseWriter, r *http.Request) {
	tasks, err := h.repo.ListTasks(r.Context())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(tasks)
}

func (h *TaskHandler) createTask(w http.ResponseWriter, r *http.Request) {
	var req struct {
		Title       string `json:"title"`
		Description string `json:"description"`
		Priority    int    `json:"priority"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	task := &TaskEntity{
		ID:          uuid.New().String(),
		Title:       req.Title,
		Description: req.Description,
		Status:      "PENDING",
		Priority:    req.Priority,
		CreatedAt:   time.Now().UTC(),
		UpdatedAt:   time.Now().UTC(),
	}

	if err := h.repo.CreateTask(r.Context(), task); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	if h.mesh != nil {
		h.mesh.BroadcastTask(r.Context(), Task{TaskID: task.ID, Action: "CREATED"})
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(task)
}

func (h *TaskHandler) handleClaimTask(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		TaskID  string `json:"task_id"`
		AgentID string `json:"agent_id"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	claimed, err := h.repo.ClaimTask(r.Context(), req.TaskID, req.AgentID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	if claimed && h.mesh != nil {
		h.mesh.BroadcastTask(r.Context(), Task{TaskID: req.TaskID, AgentID: req.AgentID, Action: "CLAIMED"})
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(map[string]interface{}{"claimed": claimed})
}
