package tasks

import (
	"encoding/json"
	"net/http"
	"github.com/prometheus/client_golang/prometheus"
)

var (
	tasksCompleted = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "tasks_completed_total",
			Help: "Total number of tasks completed",
		},
		[]string{"status"},
	)
)

func init() {
	prometheus.MustRegister(tasksCompleted)
}

// QueueResponse represents a simplistic response
type QueueResponse struct {
	Tasks []string `json:"tasks"`
}

func QueueHandler(w http.ResponseWriter, r *http.Request) {
    tasksCompleted.WithLabelValues("success").Inc()

    resp := QueueResponse{
        Tasks: []string{
			"Architect AutoDream",
			"Setup Vector Database",
			"Integrate Llama",
		},
    }

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
    json.NewEncoder(w).Encode(resp)
}
