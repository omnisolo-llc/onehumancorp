package orchestration

import (
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"sync"
	"time"

	"github.com/gorilla/websocket"
)

// MeshMessage represents a message sent across the Teammate Mesh.
type MeshMessage struct {
	SenderID  string    `json:"sender_id"`
	Role      string    `json:"role"`
	Content   string    `json:"content"`
	Timestamp time.Time `json:"timestamp"`
}

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow all origins for the mesh
	},
}

// TeammateMesh handles WebSockets and pub/sub for agent communication.
type TeammateMesh struct {
	// rooms maps room ID to a map of active websocket connections
	rooms map[string]map[*websocket.Conn]bool
	mu    sync.RWMutex

	// If true, use Redis for multi-tenant pub/sub (placeholder for actual implementation).
	useRedis bool
}

// NewTeammateMesh creates a new TeammateMesh.
func NewTeammateMesh() *TeammateMesh {
	return &TeammateMesh{
		rooms:    make(map[string]map[*websocket.Conn]bool),
		useRedis: os.Getenv("OHC_MULTITENANT") == "true",
	}
}

// ServeHTTP handles the WebSocket connection for a specific room.
// Expected endpoint: /api/v1/mesh/rooms/{room_id}
func (m *TeammateMesh) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	// Extract room ID from URL (e.g. /api/v1/mesh/rooms/room1)
	roomID := r.URL.Path[len("/api/v1/mesh/rooms/"):]
	if roomID == "" {
		http.Error(w, "room_id is required", http.StatusBadRequest)
		return
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		fmt.Printf("TeammateMesh Upgrade error: %v\n", err)
		return
	}
	defer conn.Close()

	m.mu.Lock()
	if m.rooms[roomID] == nil {
		m.rooms[roomID] = make(map[*websocket.Conn]bool)
	}
	m.rooms[roomID][conn] = true
	m.mu.Unlock()

	defer func() {
		m.mu.Lock()
		delete(m.rooms[roomID], conn)
		if len(m.rooms[roomID]) == 0 {
			delete(m.rooms, roomID)
		}
		m.mu.Unlock()
	}()

	for {
		_, message, err := conn.ReadMessage()
		if err != nil {
			break
		}

		// A PUBLISH comes in via WS payload or HTTP, we assume WS messages here
		// broadcast to the room.
		var msg MeshMessage
		if err := json.Unmarshal(message, &msg); err == nil {
			m.Broadcast(roomID, msg)
		} else {
			// If not a JSON object we can understand, wrap it
			msg = MeshMessage{
				SenderID:  "unknown",
				Role:      "unknown",
				Content:   string(message),
				Timestamp: time.Now(),
			}
			m.Broadcast(roomID, msg)
		}
	}
}

// Broadcast sends a message to all connected clients in a room.
func (m *TeammateMesh) Broadcast(roomID string, msg MeshMessage) {
	// In OHC_MULTITENANT mode, this should ideally publish to Redis.
	// For now, we broadcast to local websockets.
	payload, err := json.Marshal(msg)
	if err != nil {
		return
	}

	m.mu.RLock()
	conns := m.rooms[roomID]
	// Make a copy to avoid holding lock during write
	connList := make([]*websocket.Conn, 0, len(conns))
	for c := range conns {
		connList = append(connList, c)
	}
	m.mu.RUnlock()

	for _, c := range connList {
		c.WriteMessage(websocket.TextMessage, payload)
	}
}
