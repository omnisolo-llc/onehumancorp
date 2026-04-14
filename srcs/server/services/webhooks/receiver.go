package webhooks

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/api/mesh"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type WebhookReceiver struct {
	Mesh mesh.TeammateMeshService
}

func NewWebhookReceiver(m mesh.TeammateMeshService) *WebhookReceiver {
	return &WebhookReceiver{
		Mesh: m,
	}
}

func (w *WebhookReceiver) HandleIncoming(rw http.ResponseWriter, req *http.Request) {
	if req.Method != http.MethodPost {
		http.Error(rw, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	body, err := io.ReadAll(req.Body)
	if err != nil {
		http.Error(rw, "Failed to read body", http.StatusBadRequest)
		return
	}

	msg := orchestration.MeshMessage{
		AgentID:   "webhook-receiver",
		Action:    "WebhookReceived",
		Status:    "success",
		Content:   string(body),
		Timestamp: time.Now(),
	}

	payload, err := json.Marshal(msg)
	if err != nil {
		http.Error(rw, "Failed to encode message", http.StatusInternalServerError)
		return
	}

	err = w.Mesh.BroadcastIntent(context.Background(), string(payload))
	if err != nil {
		http.Error(rw, "Failed to broadcast to mesh", http.StatusInternalServerError)
		return
	}

	rw.WriteHeader(http.StatusOK)
	rw.Write([]byte(`{"status":"ok"}`))
}
