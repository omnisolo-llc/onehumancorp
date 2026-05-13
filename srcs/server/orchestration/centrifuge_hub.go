package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"os"
	"sync"

	"github.com/redis/rueidis"
)

type CentrifugeHub struct {
	redisClient rueidis.Client
	localMesh   MeshTransport
	isCloud     bool
	mu          sync.Mutex
}

func NewCentrifugeHub(localMesh MeshTransport) *CentrifugeHub {
	isCloud := os.Getenv("OHC_MULTITENANT") == "true"
	var client rueidis.Client
	var err error
	if isCloud {
		redisAddr := os.Getenv("REDIS_ADDR")
		if redisAddr == "" {
			redisAddr = "127.0.0.1:6379"
		}
		client, err = rueidis.NewClient(rueidis.ClientOption{InitAddress: []string{redisAddr}})
		if err != nil {
			// graceful fallback if redis init fails
			isCloud = false
		}
	}
	return &CentrifugeHub{
		redisClient: client,
		localMesh:   localMesh,
		isCloud:     isCloud,
	}
}

func (h *CentrifugeHub) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var msg MeshMessage
	if err := json.NewDecoder(r.Body).Decode(&msg); err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	data, _ := json.Marshal(msg)
	channel := msg.Channel
	if channel == "" {
		channel = "mesh:broadcast"
	}

	if h.isCloud && h.redisClient != nil {
		ctx := context.Background()
		cmd := h.redisClient.B().Publish().Channel(channel).Message(string(data)).Build()
		err := h.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		}
	} else {
		err := h.localMesh.Publish(context.Background(), channel, data)
		if err != nil {
			http.Error(w, "Internal server error", http.StatusInternalServerError)
			return
		}
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"ok"}`))
}
