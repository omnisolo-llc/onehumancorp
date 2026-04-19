package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type MeshAPI struct {
	meshTransport MeshTransport
}

func NewMeshAPI(mt MeshTransport) *MeshAPI {
	return &MeshAPI{meshTransport: mt}
}

func (api *MeshAPI) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/api/mesh/broadcast", api.HandleBroadcast)
	mux.HandleFunc("/api/v1/mesh/broadcast", api.HandleMeshV1Broadcast)
	mux.HandleFunc("/api/mesh/stream", api.HandleStream)
	mux.HandleFunc("/api/mesh/publish", api.HandlePublish)
	mux.HandleFunc("/api/mesh/connect", api.HandleConnect)
}

func (api *MeshAPI) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(r.Context(), "HandleBroadcast", time.Since(start)) }()

	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req map[string]interface{}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	payload, err := json.Marshal(req)
	if err != nil {
		http.Error(w, "Failed to marshal payload", http.StatusInternalServerError)
		return
	}

	if err := api.meshTransport.BroadcastMeshEvent(context.Background(), "tasks", payload); err != nil {
		http.Error(w, "Failed to broadcast", http.StatusInternalServerError)
		return
	}

	if telemetry.BufferMetricFunc == nil {
		telemetry.RecordMeshBroadcast(r.Context(), "tasks")
	} else {
		payloadBytes, _ := json.Marshal(map[string]interface{}{"mode": "tasks"})
		_ = telemetry.BufferMetricFunc(r.Context(), "mesh_broadcast", string(payloadBytes))
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}

func (api *MeshAPI) HandleStream(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(r.Context(), "HandleStream", time.Since(start)) }()

	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// We pass the request context here, so that when the client disconnects,
	// the underlying SubscribeMeshEvents handles the cleanup (which we verified it does via <-ctx.Done())
	ch, err := api.meshTransport.SubscribeMeshEvents(r.Context(), "tasks")
	if err != nil {
		http.Error(w, "Failed to subscribe", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "text/event-stream")
	w.Header().Set("Cache-Control", "no-cache")
	w.Header().Set("Connection", "keep-alive")

	flusher, ok := w.(http.Flusher)
	if !ok {
		http.Error(w, "Streaming unsupported", http.StatusInternalServerError)
		return
	}

	for {
		select {
		case msg, ok := <-ch:
			if !ok {
				// channel closed, meaning underlying context is done or error occurred
				return
			}
			w.Write([]byte("data: "))
			w.Write(msg)
			w.Write([]byte("\n\n"))
			flusher.Flush()
			if telemetry.BufferMetricFunc == nil {
				telemetry.RecordMeshBroadcast(r.Context(), "events")
			} else {
				payloadBytes, _ := json.Marshal(map[string]interface{}{"mode": "events"})
				_ = telemetry.BufferMetricFunc(r.Context(), "mesh_broadcast", string(payloadBytes))
			}
		case <-r.Context().Done():
			// Explicitly return on context cancellation just in case.
			return
		}
	}
}

func (api *MeshAPI) HandlePublish(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(r.Context(), "HandlePublish", time.Since(start)) }()

	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var event MeshEvent
	if err := json.NewDecoder(r.Body).Decode(&event); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	payload, err := json.Marshal(event)
	if err != nil {
		http.Error(w, "Failed to marshal payload", http.StatusInternalServerError)
		return
	}

	if err := api.meshTransport.BroadcastMeshEvent(context.Background(), "tasks", payload); err != nil {
		http.Error(w, "Failed to publish", http.StatusInternalServerError)
		return
	}

	if telemetry.BufferMetricFunc == nil {
		telemetry.RecordMeshBroadcast(r.Context(), "events")
	} else {
		payloadBytes, _ := json.Marshal(map[string]interface{}{"mode": "events"})
		_ = telemetry.BufferMetricFunc(r.Context(), "mesh_broadcast", string(payloadBytes))
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}

func (api *MeshAPI) HandleConnect(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(r.Context(), "HandleConnect", time.Since(start)) }()
	api.HandleStream(w, r)
}


func (api *MeshAPI) HandleMeshV1Broadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req struct {
		AgentID string                 `json:"agent_id"`
		Channel string                 `json:"channel"`
		Action  string                 `json:"action"`
		Status  string                 `json:"status"`
		Payload map[string]interface{} `json:"payload,omitempty"`
	}

	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if req.AgentID == "" || req.Channel == "" || req.Action == "" || req.Status == "" {
		http.Error(w, "Missing required fields", http.StatusBadRequest)
		return
	}

	payloadMap := map[string]interface{}{
		"agent_id": req.AgentID,
		"action":   req.Action,
		"status":   req.Status,
	}
	if req.Payload != nil {
		payloadMap["payload"] = req.Payload
	}

	payloadBytes, err := json.Marshal(payloadMap)
	if err != nil {
		http.Error(w, "Failed to marshal payload", http.StatusInternalServerError)
		return
	}

	if err := api.meshTransport.BroadcastMeshEvent(context.Background(), req.Channel, payloadBytes); err != nil {
		http.Error(w, "Failed to broadcast", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}
