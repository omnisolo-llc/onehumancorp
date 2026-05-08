package api

import (
	"context"
	"encoding/json"
	"io"
	"net/http"
	"sync"

	"onehumancorp/srcs/server/orchestration"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true
	},
}

type MeshHandler struct {
	transport orchestration.MeshTransport
}

type safeWebSocket struct {
	conn *websocket.Conn
	mu   sync.Mutex
}

func (s *safeWebSocket) WriteMessage(messageType int, data []byte) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	return s.conn.WriteMessage(messageType, data)
}

func NewMeshHandler(transport orchestration.MeshTransport) *MeshHandler {
	return &MeshHandler{transport: transport}
}

// MeshBroadcastRequest matches OHC-SIP compliance standards
type MeshBroadcastRequest struct {
	AgentID   string          `json:"agent_id"`
	Channel   string          `json:"channel"`
	EventType string          `json:"event_type"`
	Data      json.RawMessage `json:"data"`
}

func (h *MeshHandler) HandleBroadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	body, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Failed to read request body", http.StatusInternalServerError)
		return
	}
	defer r.Body.Close()

	var req MeshBroadcastRequest
	if err := json.Unmarshal(body, &req); err != nil {
		http.Error(w, "Invalid JSON", http.StatusBadRequest)
		return
	}

	if req.Channel == "" || req.AgentID == "" || req.EventType == "" {
		http.Error(w, "Missing required OHC-SIP fields", http.StatusBadRequest)
		return
	}

	payload, err := json.Marshal(req)
	if err != nil {
		http.Error(w, "Failed to encode payload", http.StatusInternalServerError)
		return
	}

	if err := h.transport.Publish(r.Context(), req.Channel, payload); err != nil {
		http.Error(w, "Failed to broadcast message", http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status":"ok"}`))
}

func (h *MeshHandler) HandleSubscribe(w http.ResponseWriter, r *http.Request) {
	channel := r.URL.Query().Get("channel")
	if channel == "" {
		http.Error(w, "Channel is required", http.StatusBadRequest)
		return
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		return
	}
	defer conn.Close()

	safeConn := &safeWebSocket{conn: conn}

	ctx, cancel := context.WithCancel(r.Context())
	defer cancel()

	err = h.transport.Subscribe(ctx, channel, func(data []byte) {
		if err := safeConn.WriteMessage(websocket.TextMessage, data); err != nil {
			cancel()
		}
	})

	if err != nil {
		return
	}

	for {
		if _, _, err := conn.ReadMessage(); err != nil {
			break
		}
	}
}
