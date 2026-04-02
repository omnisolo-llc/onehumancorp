package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strings"
	"sync"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow cross-origin for the mesh
	},
}

// MeshMessage represents a realtime message sent over the Teammate Mesh
type MeshMessage struct {
	Type        string `json:"type"`
	TaskID      string `json:"task_id,omitempty"`
	Priority    string `json:"priority,omitempty"`
	Description string `json:"description,omitempty"`
	SenderID    string `json:"sender_id,omitempty"`
	Role        string `json:"role,omitempty"`
	Content     string `json:"content,omitempty"`
	Timestamp   string `json:"timestamp,omitempty"`
}

// MeshTransport handles websocket connections
type MeshTransport struct {
	hub      *Hub
	clients  map[*websocket.Conn]string // Conn -> AgentID
	clientsMu sync.RWMutex
}

// NewMeshTransport creates a new transport for the Teammate Mesh
func NewMeshTransport(hub *Hub) *MeshTransport {
	return &MeshTransport{
		hub:     hub,
		clients: make(map[*websocket.Conn]string),
	}
}

// HandleWS upgrades the HTTP connection and handles websocket mesh messages
func (mt *MeshTransport) HandleWS(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		log.Printf("mesh: websocket upgrade failed: %v", err)
		return
	}

	agentID := r.URL.Query().Get("agent_id")
	if agentID == "" {
		agentID = "anonymous"
	}

	mt.clientsMu.Lock()
	mt.clients[conn] = agentID
	mt.clientsMu.Unlock()

	defer func() {
		mt.clientsMu.Lock()
		delete(mt.clients, conn)
		mt.clientsMu.Unlock()
		conn.Close()
	}()

	for {
		_, msgBytes, err := conn.ReadMessage()
		if err != nil {
			break
		}

		var msg MeshMessage
		if err := json.Unmarshal(msgBytes, &msg); err != nil {
			continue
		}

		// Update telemetry counter for tasks completed if applicable
		if msg.Type == "TASK_COMPLETED" {
			ctx := r.Context()
			telemetry.Add(ctx, "ohc_swarm_tasks_completed", 1)
		}

		// Broadcast message to other connected clients
		mt.Broadcast(msgBytes, conn)
	}
}

// Broadcast sends a message to all connected clients except the sender
func (mt *MeshTransport) Broadcast(message []byte, sender *websocket.Conn) {
	// First gather all target clients to avoid holding the lock during blocking I/O
	mt.clientsMu.RLock()
	var targets []*websocket.Conn
	for client := range mt.clients {
		if client != sender {
			targets = append(targets, client)
		}
	}
	mt.clientsMu.RUnlock()

	for _, client := range targets {
		// In a production system we'd protect concurrent writes to a single connection via channels/mutexes per connection,
		// but since Broadcast is iterating over all conns, this at least prevents deadlocking the whole map.
		client.WriteMessage(websocket.TextMessage, message)
	}
}
