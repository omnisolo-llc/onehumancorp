package orchestration

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"log/slog"
)

// RegisterTaskHTTPHandlers registers the REST endpoints for Shared Tasks.
func RegisterTaskHTTPHandlers(mux *http.ServeMux, tm *TaskManager) {
	mux.HandleFunc("/api/sync/missions", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost {
			handleSyncMissions(w, r, tm)
			return
		}
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
	})

	mux.HandleFunc("/api/orchestration/tasks", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost {
			handleCreateTask(w, r, tm)
			return
		}
		if r.Method == http.MethodGet {
			handlePollTasks(w, r, tm)
			return
		}
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
	})

	mux.HandleFunc("/api/orchestration/tasks/", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPut && strings.HasSuffix(r.URL.Path, "/status") {
			handleUpdateTaskStatus(w, r, tm)
			return
		}
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
	})
}

func handleSyncMissions(w http.ResponseWriter, r *http.Request, tm *TaskManager) {
	var payloads []SyncDaemonPayload
	if err := json.NewDecoder(r.Body).Decode(&payloads); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	for _, payload := range payloads {
		// Assuming we do an INSERT on conflict UPDATE or just an INSERT
		// Let's implement an upsert logic using tm.db
		// Or using K8s cloud orchestration agent_missions table
		query := `INSERT INTO agent_missions (id, status, payload, synced_to_cloud)
				  VALUES ($1, $2, $3, true)
				  ON CONFLICT (id) DO UPDATE SET status = EXCLUDED.status, payload = EXCLUDED.payload, synced_to_cloud = true`
		if tm.db.IsSQLite() {
			query = `INSERT INTO agent_missions (id, status, payload, synced_to_cloud)
					 VALUES ($1, $2, $3, 1)
					 ON CONFLICT (id) DO UPDATE SET status = EXCLUDED.status, payload = EXCLUDED.payload, synced_to_cloud = 1`
		}

		_, err := tm.db.Exec(r.Context(), query, payload.ID, payload.Status, payload.Payload)
		if err != nil {
			slog.Error("failed to inject synced mission", "error", err, "id", payload.ID)
			continue
		}

		// KAIROS Orchestration broadcasts task updates
		tm.hub.PublishTaskBroadcast(payload.ID, map[string]interface{}{
			"action": "sync",
			"status": payload.Status,
			"agent_id": "system", // Or something appropriate
			"payload": payload.Payload,
		})
	}

	w.WriteHeader(http.StatusOK)
}

func handleCreateTask(w http.ResponseWriter, r *http.Request, tm *TaskManager) {
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

	claims := auth.ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	task, err := tm.CreateTask(r.Context(), claims.OrganizationID, req.Title, req.Description, req.Priority)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(task)
}

func handlePollTasks(w http.ResponseWriter, r *http.Request, tm *TaskManager) {
	agentID := r.URL.Query().Get("agent_id")
	if agentID == "" {
		http.Error(w, "agent_id query parameter is required", http.StatusBadRequest)
		return
	}

	limit := 10 // Default limit

	tasks, err := tm.PollTasks(r.Context(), agentID, limit)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	if tasks == nil {
		tasks = []*SharedTask{}
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(tasks)
}

func handleUpdateTaskStatus(w http.ResponseWriter, r *http.Request, tm *TaskManager) {
	parts := strings.Split(strings.Trim(r.URL.Path, "/"), "/")
	if len(parts) != 5 || parts[4] != "status" {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}
	taskID := parts[3]

	var req struct {
		Status  string `json:"status"`
		AgentID string `json:"agent_id"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	var err error
	switch req.Status {
	case "REVIEW":
		err = tm.ReviewTask(r.Context(), taskID, req.AgentID)
	case "COMPLETED":
		err = tm.CompleteTask(r.Context(), taskID, req.AgentID)
	default:
		http.Error(w, "invalid status transition", http.StatusBadRequest)
		return
	}

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
