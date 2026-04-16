package api

import (
	"context"
	"encoding/json"
	"net/http"

	"github.com/redis/go-redis/v9"
)

type MeshEvent struct {
	AgentID   string `json:"agent_id"`
	Channel   string `json:"channel"`
	EventType string `json:"event_type"`
	Data      struct {
		TaskID        string `json:"task_id"`
		PreviousState string `json:"previous_state"`
		NewState      string `json:"new_state"`
	} `json:"data"`
}

type MeshHandler struct {
	redisClient *redis.Client
}

func NewMeshHandler(redisClient *redis.Client) *MeshHandler {
	return &MeshHandler{redisClient: redisClient}
}

func (h *MeshHandler) Broadcast(w http.ResponseWriter, r *http.Request) {
	var event MeshEvent
	if err := json.NewDecoder(r.Body).Decode(&event); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	payload, err := json.Marshal(event)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	if err := h.redisClient.Publish(context.Background(), event.Channel, payload).Err(); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

// TODO: Implement WebSockets and gRPC broadcast channels
// As per the architecture requirements, the Teammate Mesh must also support WebSocket connections for UI clients
// and gRPC streams for high-throughput inter-agent communication.
