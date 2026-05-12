package mesh

import (
	"encoding/json"
	"io"
	"net/http"

	"github.com/gorilla/websocket"
	"onehumancorp/srcs/server/orchestration"
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
}

type MeshHandler struct {
	Transport orchestration.MeshTransport
}

func NewMeshHandler(transport orchestration.MeshTransport) *MeshHandler {
	return &MeshHandler{Transport: transport}
}

func (h *MeshHandler) Broadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var msg orchestration.MeshMessage
	if err := json.NewDecoder(r.Body).Decode(&msg); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if msg.AgentID == "" || msg.EventType == "" || msg.Channel == "" || msg.Data == nil {
		http.Error(w, "Missing required OHC-SIP fields (agent_id, channel, event_type, data)", http.StatusBadRequest)
		return
	}

	channel := msg.Channel

	data, err := json.Marshal(msg)
	if err != nil {
		http.Error(w, "Failed to marshal message", http.StatusInternalServerError)
		return
	}

	if err := h.Transport.Publish(r.Context(), channel, data); err != nil {
		http.Error(w, "Failed to publish message", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func (h *MeshHandler) Publish(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	channel := r.URL.Query().Get("channel")
	if channel == "" {
		channel = "mesh:default"
	}

	// Read body with limit (e.g. 1MB)
	r.Body = http.MaxBytesReader(w, r.Body, 1024*1024)
	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Failed to read body", http.StatusInternalServerError)
		return
	}

	if err := h.Transport.Publish(r.Context(), channel, body); err != nil {
		http.Error(w, "Failed to publish message", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

func (h *MeshHandler) Subscribe(w http.ResponseWriter, r *http.Request) {
	channel := r.URL.Query().Get("channel")
	if channel == "" {
		http.Error(w, "Missing channel parameter", http.StatusBadRequest)
		return
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		http.Error(w, "Failed to upgrade to websocket", http.StatusInternalServerError)
		return
	}
	defer conn.Close()

	ctx := r.Context()

	// Subscribe to the channel
	err = h.Transport.Subscribe(ctx, channel, func(data []byte) {
		if writeErr := conn.WriteMessage(websocket.TextMessage, data); writeErr != nil {
			// Write failed, client might have disconnected.
		}
	})
	if err != nil {
		return
	}

	// Keep connection open until client disconnects or error occurs
	// Blocking the HTTP handler to keep context active
	for {
		_, _, readErr := conn.ReadMessage()
		if readErr != nil {
			break
		}
	}
}

func (h *MeshHandler) Capabilities(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	skill := r.URL.Query().Get("skill")
	if skill == "" {
		http.Error(w, "Missing skill parameter", http.StatusBadRequest)
		return
	}

	agents, err := h.Transport.DiscoverAgents(r.Context(), skill)
	if err != nil {
		http.Error(w, "Failed to discover agents", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(agents)
}
