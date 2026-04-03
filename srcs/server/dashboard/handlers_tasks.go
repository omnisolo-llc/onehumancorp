package dashboard

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func (s *Server) handleTasks(w http.ResponseWriter, r *http.Request) {
	if s.hub == nil || s.hub.TaskManager() == nil {
		http.Error(w, "Task Manager not configured", http.StatusInternalServerError)
		return
	}

	ctx := r.Context()
	tm := s.hub.TaskManager()

	switch r.Method {
	case http.MethodPost:
		var req struct {
			MissionID   string `json:"mission_id"`
			Title       string `json:"title"`
			Description string `json:"description"`
			Priority    string `json:"priority"`
		}
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "invalid request body", http.StatusBadRequest)
			return
		}

		if req.Priority == "" {
			req.Priority = "P2"
		}

		task, err := tm.CreateTask(ctx, req.MissionID, req.Title, req.Description, req.Priority)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(task)

	case http.MethodGet:
		agentID := r.URL.Query().Get("agent_id")
		if agentID == "" {
			http.Error(w, "missing agent_id", http.StatusBadRequest)
			return
		}

		tasks, err := tm.PollTasks(ctx, agentID, 10)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		if tasks == nil {
			tasks = []*orchestration.SharedTask{} // return empty list instead of null
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(tasks)

	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

func (s *Server) handleTaskStatus(w http.ResponseWriter, r *http.Request) {
	if s.hub == nil || s.hub.TaskManager() == nil {
		http.Error(w, "Task Manager not configured", http.StatusInternalServerError)
		return
	}

	if r.Method != http.MethodPut {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	parts := strings.Split(strings.Trim(r.URL.Path, "/"), "/")
	if len(parts) < 4 {
		http.Error(w, "invalid task id", http.StatusBadRequest)
		return
	}
	taskID := parts[3] // /api/orchestration/tasks/{id}/status

	var req struct {
		Status  string `json:"status"`
		AgentID string `json:"agent_id"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	ctx := r.Context()
	tm := s.hub.TaskManager()

	var err error
	switch req.Status {
	case "REVIEW":
		err = tm.ReviewTask(ctx, taskID, req.AgentID)
	case "COMPLETED":
		err = tm.CompleteTask(ctx, taskID, req.AgentID)
	default:
		http.Error(w, "invalid status", http.StatusBadRequest)
		return
	}

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
