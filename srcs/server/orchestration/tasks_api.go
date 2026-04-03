package orchestration

import (
	"encoding/json"
	"net/http"
	"strings"
)

// Define structures for API
type CreateTaskRequest struct {
	MissionID   string `json:"missionId"`
	Title       string `json:"title"`
	Description string `json:"description"`
	Priority    string `json:"priority"`
}

type UpdateTaskStatusRequest struct {
	Status  string `json:"status"`
	AgentID string `json:"agentId"`
}

// TasksHandler handles HTTP requests to /api/orchestration/tasks
func (tm *TaskManager) TasksHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodPost {
		var req CreateTaskRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}

		task, err := tm.CreateTask(r.Context(), req.MissionID, req.Title, req.Description, req.Priority)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(task)
		return
	}

	if r.Method == http.MethodGet {
		agentID := r.URL.Query().Get("agentId")
		if agentID == "" {
			http.Error(w, "agentId query parameter is required", http.StatusBadRequest)
			return
		}

		tasks, err := tm.PollTasks(r.Context(), agentID, 10)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(tasks)
		return
	}

	http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
}

// TaskStatusHandler handles HTTP requests to /api/orchestration/tasks/{id}/status
func (tm *TaskManager) TaskStatusHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPut {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Extract {id} from path assuming standard mapping or passing it properly.
	// For simplicity, we get it from URL if we handle basic matching
	parts := strings.Split(r.URL.Path, "/")
	if len(parts) < 2 {
		http.Error(w, "Invalid path", http.StatusBadRequest)
		return
	}
	// /api/orchestration/tasks/{id}/status
	// Extract {id} more robustly
	var id string
	for i, part := range parts {
		if part == "tasks" && i+2 < len(parts) && parts[i+2] == "status" {
			id = parts[i+1]
			break
		}
	}

	if id == "" {
		http.Error(w, "Invalid path: missing task ID", http.StatusBadRequest)
		return
	}

	var req UpdateTaskStatusRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	if req.AgentID == "" {
		http.Error(w, "agentId is required", http.StatusBadRequest)
		return
	}

	var err error
	if req.Status == "REVIEW" {
		err = tm.ReviewTask(r.Context(), id, req.AgentID)
	} else if req.Status == "COMPLETED" {
		err = tm.CompleteTask(r.Context(), id, req.AgentID)
	} else {
		http.Error(w, "Invalid status transition", http.StatusBadRequest)
		return
	}

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
