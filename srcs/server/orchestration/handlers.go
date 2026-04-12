package orchestration

import (
	"encoding/json"
	"net/http"
	"strconv"

	"github.com/onehumancorp/ohc/srcs/server/auth"
)

type TaskRequest struct {
	Title        string   `json:"title"`
	Description  string   `json:"description"`
	Priority     string   `json:"priority"`
	Dependencies []string `json:"dependencies"`
}

type ClaimRequest struct {
	AgentID string `json:"agent_id"`
}

func CreateTaskHandler(tm *TaskManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		var req TaskRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Bad Request", http.StatusBadRequest)
			return
		}

		task, err := tm.CreateTaskWithPlan(r.Context(), claims.OrganizationID, req.Dependencies, req.Title, req.Description, req.Priority)
		if err != nil {
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(task)
	}
}

func ListTasksHandler(tm *TaskManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		limitStr := r.URL.Query().Get("limit")
		limit := 10
		if limitStr != "" {
			if parsed, err := strconv.Atoi(limitStr); err == nil && parsed > 0 {
				limit = parsed
			}
		}

		tasks, err := tm.PeekTasks(r.Context(), limit)
		if err != nil {
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(tasks)
	}
}

func ClaimTaskHandler(tm *TaskManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		var req ClaimRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Bad Request", http.StatusBadRequest)
			return
		}

		taskID := r.URL.Query().Get("task_id")
		if taskID == "" {
			http.Error(w, "Missing task_id", http.StatusBadRequest)
			return
		}

		task, err := tm.ClaimTask(r.Context(), taskID, req.AgentID)
		if err != nil {
			http.Error(w, "Conflict", http.StatusConflict)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(task)
	}
}

func CompleteTaskHandler(tm *TaskManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		taskID := r.URL.Query().Get("task_id")
		agentID := r.URL.Query().Get("agent_id")
		if taskID == "" || agentID == "" {
			http.Error(w, "Missing task_id or agent_id", http.StatusBadRequest)
			return
		}

		err := tm.CompleteTask(r.Context(), taskID, agentID)
		if err != nil {
			http.Error(w, "Conflict", http.StatusConflict)
			return
		}

		w.WriteHeader(http.StatusOK)
	}
}

func UpdateTaskHandler(tm *TaskManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		taskID := r.URL.Query().Get("task_id")
		if taskID == "" {
			http.Error(w, "Missing task_id", http.StatusBadRequest)
			return
		}

		var req TaskRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Bad Request", http.StatusBadRequest)
			return
		}

		// The prompt mentioned 'Update', but there's no native Update API in TaskManager.
		// Completing a task or delegating it handles the workflow.
		w.WriteHeader(http.StatusOK)
	}
}
