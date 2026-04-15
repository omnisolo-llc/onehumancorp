package mesh

import (
	"encoding/json"
	"log/slog"
	"net/http"
	"context"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/interop"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type MeshEvent struct {
	AgentID string                 `json:"agent_id"`
	Action  string                 `json:"action"`
	Status  string                 `json:"status"`
	Data    map[string]interface{} `json:"data"`
}

func HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	mode := "cloud"
	if os.Getenv("OHC_STANDALONE") == "true" {
		mode = "standalone"
	}
	telemetry.RecordMeshBroadcast(r.Context(), mode)

	var event MeshEvent
	if err := json.NewDecoder(r.Body).Decode(&event); err != nil {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	if event.AgentID == "" || event.Action == "" || event.Status == "" {
		http.Error(w, "invalid request", http.StatusBadRequest)
		return
	}

	meshClient, err := interop.NewTeammateMesh()
	if err != nil {
		slog.Error("HandleBroadcast: failed to initialize mesh", "error", err)
		http.Error(w, "internal server error", http.StatusInternalServerError)
		return
	}

	payloadBytes, err := json.Marshal(event)
	if err != nil {
		slog.Error("HandleBroadcast: failed to marshal payload", "error", err)
		http.Error(w, "internal server error", http.StatusInternalServerError)
		return
	}

	ctx, cancel := context.WithTimeout(r.Context(), 5*time.Second)
	defer cancel()

	if err := meshClient.Publish(ctx, "mesh:tasks", payloadBytes); err != nil {
		slog.Error("HandleBroadcast: failed to publish to mesh", "error", err)
		http.Error(w, "failed to publish", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}
