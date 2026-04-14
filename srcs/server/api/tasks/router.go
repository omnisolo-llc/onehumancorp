package tasks

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type Router struct {
	queue *TaskQueue
}

func NewRouter(dbProvider db.Provider) *Router {
	return &Router{queue: NewTaskQueue(dbProvider)}
}

func (r *Router) Register(mux *http.ServeMux) {
	mux.HandleFunc("/api/tasks/list", r.handleList)
	mux.HandleFunc("/api/tasks/claim", r.handleClaim)
	mux.HandleFunc("/api/tasks/complete", r.handleComplete)
}

func (r *Router) handleList(w http.ResponseWriter, req *http.Request) {
	tasks, err := r.queue.ListTasks(req.Context())
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}
	if tasks == nil {
		tasks = []Task{}
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(tasks)
}

func (r *Router) handleClaim(w http.ResponseWriter, req *http.Request) {
	agentID := req.URL.Query().Get("agent_id")
	if agentID == "" {
		http.Error(w, "missing agent_id", http.StatusBadRequest)
		return
	}

	task, err := r.queue.ClaimTask(req.Context(), agentID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	if task == nil {
		w.WriteHeader(http.StatusNoContent)
		return
	}

	json.NewEncoder(w).Encode(task)
}

func (r *Router) handleComplete(w http.ResponseWriter, req *http.Request) {
	taskID := req.URL.Query().Get("task_id")
	if taskID == "" {
		http.Error(w, "missing task_id", http.StatusBadRequest)
		return
	}

	err := r.queue.CompleteTask(req.Context(), taskID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
