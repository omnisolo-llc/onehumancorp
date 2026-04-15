package mesh

import (
	"context"
	"encoding/json"
	"net/http"
	"os"

	"github.com/redis/rueidis"
)

func HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Payload is already validated by ValidationMiddleware, but we decode it to extract the channel/action
	var payload map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&payload); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		http.Error(w, "failed to marshal payload", http.StatusInternalServerError)
		return
	}

	// Assuming action is the channel for broadcast
	channel := "system"
	if action, ok := payload["action"].(string); ok && action != "" {
		channel = action
	}

	isCloud := os.Getenv("OHC_STANDALONE") != "true"

	if isCloud {
		// Redis Pub/Sub broadcast
		client, err := rueidis.NewClient(rueidis.ClientOption{
			InitAddress: []string{os.Getenv("REDIS_URL")},
		})
		if err == nil {
			defer client.Close()
			cmd := client.B().Publish().Channel(channel).Message(string(payloadBytes)).Build()
			client.Do(context.Background(), cmd)
		}
	} else {
		// Local broker fallback (dummy implementation for standalone without redis)
		// In a real scenario, this would use the LocalMeshBroker's channel system
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"ok"}`))
}
