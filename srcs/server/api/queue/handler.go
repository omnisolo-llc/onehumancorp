package queue

import (
	"encoding/json"
	"net/http"

	orchqueue "github.com/onehumancorp/mono/srcs/server/orchestration/queue"
)

type SpawnRequest struct {
	JobID     string                       `json:"job_id"`
	QueueName string                       `json:"queue_name"`
	Data      orchqueue.SubAgentTaskData   `json:"data"`
}

func HandleSpawn(getQueue func() orchqueue.SubAgentTaskQueue) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method Not Allowed", http.StatusMethodNotAllowed)
			return
		}

		var req SpawnRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Bad Request", http.StatusBadRequest)
			return
		}

		queue := getQueue()
		if queue == nil {
			http.Error(w, "Queue not initialized", http.StatusInternalServerError)
			return
		}

		err := queue.Enqueue(r.Context(), &orchqueue.SubAgentTaskQueuePayload{
			JobID:     req.JobID,
			QueueName: req.QueueName,
			Data:      req.Data,
		})

		if err != nil {
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusAccepted)
		json.NewEncoder(w).Encode(map[string]string{"status": "queued"})
	}
}
