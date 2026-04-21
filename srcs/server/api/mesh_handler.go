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

	var sipReq struct {
		AgentID string          `json:"agent_id"`
		Action  string          `json:"action"`
		Status  string          `json:"status"`
		Channel string          `json:"channel"`
		Data    json.RawMessage `json:"data"`
	}

	if err := json.Unmarshal(bodyBytes, &sipReq); err != nil {
		http.Error(w, "Invalid JSON payload", http.StatusBadRequest)
		return
	}

	if sipReq.AgentID == "" || sipReq.Action == "" || sipReq.Status == "" {
		http.Error(w, "Missing required OHC-SIP fields (agent_id, action, status)", http.StatusBadRequest)
		return
	}

	channel := sipReq.Channel
	if channel == "" {
		channel = "default"
	}

	if err := h.transport.Publish(r.Context(), channel, bodyBytes); err != nil {
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

func (h *MeshHandler) Capabilities(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	skill := r.URL.Query().Get("skill")
	agents, err := h.transport.DiscoverAgents(r.Context(), skill)
	if err != nil {
		http.Error(w, fmt.Sprintf("Failed to discover agents: %v", err), http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(map[string]interface{}{"capabilities": agents}); err != nil {
		http.Error(w, "Failed to encode response", http.StatusInternalServerError)
	}
}
