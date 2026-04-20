package api

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/mesh"
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow cross-origin for the mesh UI/thin client
	},
}

type MeshHandler struct {
	transport mesh.MeshTransport
}

func NewMeshHandler(transport mesh.MeshTransport) *MeshHandler {
	return &MeshHandler{
		transport: transport,
	}
}

// Broadcast handles POST /api/v1/mesh/broadcast
func (h *MeshHandler) Broadcast(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	bodyBytes, err := io.ReadAll(r.Body)
	if err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	var payload struct {
		AgentID   string          `json:"agent_id"`
		Channel   string          `json:"channel"`
		EventType string          `json:"event_type"`
		Data      json.RawMessage `json:"data"`
	}

	if err := json.Unmarshal(bodyBytes, &payload); err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	if payload.AgentID == "" || payload.Channel == "" || payload.EventType == "" || payload.Data == nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	if err := h.transport.Publish(r.Context(), payload.Channel, payload.Data); err != nil {
		http.Error(w, fmt.Sprintf("Failed to broadcast: %v", err), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
}

// Subscribe handles GET /api/v1/mesh/subscribe
func (h *MeshHandler) Subscribe(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	channel := r.URL.Query().Get("channel")
	if channel == "" {
		channel = "default"
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		// upgrader.Upgrade already replies with an error
		return
	}
	defer conn.Close()

	ctx, cancel := context.WithCancel(r.Context())
	defer cancel()

	sub, err := h.transport.Subscribe(ctx, channel)
	if err != nil {
		conn.WriteMessage(websocket.TextMessage, []byte(fmt.Sprintf("Failed to subscribe: %v", err)))
		return
	}

	// Read from websocket to handle disconnects
	go func() {
		defer cancel()
		for {
			if _, _, err := conn.ReadMessage(); err != nil {
				break
			}
		}
	}()

	for {
		select {
		case msg, ok := <-sub:
			if !ok {
				return
			}
			if err := conn.WriteMessage(websocket.TextMessage, msg); err != nil {
				return
			}
		case <-ctx.Done():
			return
		}
	}
}
