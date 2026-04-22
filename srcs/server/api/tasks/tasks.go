package tasks

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/db/models"
)

type Server struct {
	repo db.SharedTaskRepository
}

func NewServer(repo db.SharedTaskRepository) *Server {
	return &Server{repo: repo}
}

func (s *Server) RegisterHandlers(mux *http.ServeMux) {
	mux.HandleFunc("/api/queue/subagent", s.HandleEnqueueTask)
	mux.HandleFunc("/api/v1/tasks/claim", s.HandleClaimTask)
}

func (s *Server) HandleEnqueueTask(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		OrganizationID string  `json:"organization_id"`
		EpicID         *string `json:"epic_id"`
		Title          string  `json:"title"`
		Description    *string `json:"description"`
		Priority       *string `json:"priority"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request payload", http.StatusBadRequest)
		return
	}

	task := &models.SharedTask{
		OrganizationID: req.OrganizationID,
		EpicID:         req.EpicID,
		Title:          req.Title,
		Description:    req.Description,
		Priority:       req.Priority,
	}

	if err := s.repo.CreateSharedTask(r.Context(), task); err != nil {
		http.Error(w, "Failed to create task", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(task)
}

func (s *Server) HandleClaimTask(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		TaskID  string `json:"task_id"`
		AgentID string `json:"agent_id"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request payload", http.StatusBadRequest)
		return
	}

	claimed, err := s.repo.ClaimSharedTask(r.Context(), req.TaskID, req.AgentID)
	if err != nil {
		http.Error(w, "Failed to claim task", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	if !claimed {
		w.WriteHeader(http.StatusConflict)
		json.NewEncoder(w).Encode(map[string]string{"error": "Task already claimed or not found"})
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(map[string]string{"status": "claimed"})
}
