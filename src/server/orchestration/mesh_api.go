package orchestration


import (
	"context"
	"encoding/json"
	"net/http"
	"time"
	"io"

	"google.golang.org/protobuf/encoding/protojson"

	"github.com/onehumancorp/mono/src/server/telemetry"
	pb "github.com/onehumancorp/mono/src/proto"
)


type MeshAPI struct {
	meshTransport MeshTransport
}

func NewMeshAPI(mt MeshTransport) *MeshAPI {
	return &MeshAPI{meshTransport: mt}
}

func (api *MeshAPI) RegisterRoutes(mux *http.ServeMux) {
	mux.HandleFunc("/api/mesh/v2/broadcast", api.HandleMeshV2Broadcast)
	mux.HandleFunc("/api/mesh/broadcast", api.HandleBroadcast)
	mux.HandleFunc("/api/v1/mesh/broadcast", api.HandleMeshV1Broadcast)
	mux.HandleFunc("/api/mesh/stream", api.HandleStream)
	mux.HandleFunc("/api/mesh/sync", api.HandleSync)
	mux.HandleFunc("/api/mesh/publish", api.HandlePublish)
	mux.HandleFunc("/api/mesh/connect", api.HandleConnect)
}


func (api *MeshAPI) HandleMeshV2Broadcast(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(r.Context(), "HandleMeshV2Broadcast", time.Since(start)) }()
	if telemetry.BufferMetricFunc == nil {
		telemetry.RecordMeshBroadcast(r.Context(), "events")
	} else {
		payloadBytes, _ := json.Marshal(map[string]interface{}{"mode": "events"})
		_ = telemetry.BufferMetricFunc(r.Context(), "mesh_broadcast", string(payloadBytes))
	}

	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req pb.PublishTeammateMeshEventRequest

	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Failed to read body", http.StatusBadRequest)
		return
	}

	if err := protojson.Unmarshal(body, &req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if req.GetChannel() == "" {
		http.Error(w, "Missing channel parameter", http.StatusBadRequest)
		return
	}

	if req.GetEvent() == nil {
		http.Error(w, "Missing event parameter", http.StatusBadRequest)
		return
	}

	payload, err := protojson.Marshal(req.GetEvent())
	if err != nil {
		http.Error(w, "Failed to marshal payload", http.StatusInternalServerError)
		return
	}

	if err := api.meshTransport.BroadcastMeshEvent(r.Context(), req.GetChannel(), payload); err != nil {
		http.Error(w, "Failed to broadcast", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}

func (api *MeshAPI) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(r.Context(), "HandleBroadcast", time.Since(start)) }()
	if telemetry.BufferMetricFunc == nil {
		telemetry.RecordMeshBroadcast(r.Context(), "events")
	} else {
		payloadBytes, _ := json.Marshal(map[string]interface{}{"mode": "events"})
		_ = telemetry.BufferMetricFunc(r.Context(), "mesh_broadcast", string(payloadBytes))
	}

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

	channelName := "tasks"
	if ch, ok := req["channel"].(string); ok && ch != "" {
		channelName = ch
	}

	if err := api.meshTransport.BroadcastMeshEvent(context.Background(), channelName, payload); err != nil {
		http.Error(w, "Failed to broadcast", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}

func (api *MeshAPI) HandleStream(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(r.Context(), "HandleStream", time.Since(start)) }()
	if telemetry.BufferMetricFunc == nil {
		telemetry.RecordMeshBroadcast(r.Context(), "events")
	} else {
		payloadBytes, _ := json.Marshal(map[string]interface{}{"mode": "events"})
		_ = telemetry.BufferMetricFunc(r.Context(), "mesh_broadcast", string(payloadBytes))
	}

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

	w.Write([]byte("retry: 3000\n\n"))
	flusher.Flush()

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
		case <-r.Context().Done():
			// Explicitly return on context cancellation just in case.
			return
		}
	}
}

func (api *MeshAPI) HandlePublish(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(r.Context(), "HandlePublish", time.Since(start)) }()
	if telemetry.BufferMetricFunc == nil {
		telemetry.RecordMeshBroadcast(r.Context(), "events")
	} else {
		payloadBytes, _ := json.Marshal(map[string]interface{}{"mode": "events"})
		_ = telemetry.BufferMetricFunc(r.Context(), "mesh_broadcast", string(payloadBytes))
	}

	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var event pb.MeshEvent
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

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"success"}`))
}

func (api *MeshAPI) HandleConnect(w http.ResponseWriter, r *http.Request) {
	api.HandleStream(w, r)
}


func (api *MeshAPI) HandleMeshV1Broadcast(w http.ResponseWriter, r *http.Request) {
	start := time.Now()
	defer func() { telemetry.RecordMeshLatency(r.Context(), "HandleMeshV1Broadcast", time.Since(start)) }()
	if telemetry.BufferMetricFunc == nil {
		telemetry.RecordMeshBroadcast(r.Context(), "events")
	} else {
		payloadBytes, _ := json.Marshal(map[string]interface{}{"mode": "events"})
		_ = telemetry.BufferMetricFunc(r.Context(), "mesh_broadcast", string(payloadBytes))
	}

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

func (api *MeshAPI) HandleSync(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	channel := r.URL.Query().Get("channel")
	if channel == "" {
		http.Error(w, "Missing channel parameter", http.StatusBadRequest)
		return
	}

	ch, err := api.meshTransport.SubscribeMeshEvents(r.Context(), channel)
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

	w.Write([]byte("retry: 3000\n\n"))
	flusher.Flush()

	for {
		select {
		case msg, ok := <-ch:
			if !ok {
				return
			}
			w.Write([]byte("data: "))
			w.Write(msg)
			w.Write([]byte("\n\n"))
			flusher.Flush()
		case <-r.Context().Done():
			return
		}
	}
}
