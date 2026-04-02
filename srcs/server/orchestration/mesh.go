package orchestration

import (
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"os"
	"sync"
	"time"

	"github.com/gorilla/websocket"
	"github.com/redis/go-redis/v9"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		// Secure CheckOrigin implementation. Allow connections from the same origin,
		// or if it's explicitly enabled for local development.
		origin := r.Header.Get("Origin")
		if origin == "" {
			return true // Non-browser clients (e.g. CLI, tests)
		}
		if os.Getenv("OHC_DEV_ALLOW_ORIGIN") == "true" {
			return true
		}
		// In production, verify origin against expected host
		return origin == "http://"+r.Host || origin == "https://"+r.Host
	},
}

// MeshMessage represents a message sent or received over the Teammate Mesh.
type MeshMessage struct {
	SenderID  string    `json:"sender_id"`
	Role      string    `json:"role"`
	Content   string    `json:"content"`
	Timestamp time.Time `json:"timestamp"`
}

// MeshRoom represents a communication room in the Teammate Mesh.
type MeshRoom struct {
	ID      string
	Clients map[*websocket.Conn]bool
	mu      sync.Mutex
}

// MeshManager manages active Teammate Mesh rooms and client connections.
type MeshManager struct {
	rooms    map[string]*MeshRoom
	mu       sync.Mutex
	redisCli *redis.Client
}

// NewMeshManager creates a new MeshManager instance.
func NewMeshManager() *MeshManager {
	manager := &MeshManager{
		rooms: make(map[string]*MeshRoom),
	}

	redisURL := os.Getenv("REDIS_URL")
	if os.Getenv("OHC_MULTITENANT") == "true" && redisURL != "" {
		opts, err := redis.ParseURL(redisURL)
		if err == nil {
			manager.redisCli = redis.NewClient(opts)
			slog.Info("mesh: initialized with Redis Pub/Sub", "redis_url", redisURL)
			go manager.listenRedis(context.Background())
		} else {
			slog.Error("mesh: failed to parse REDIS_URL for pub/sub", "err", err)
		}
	} else {
		slog.Info("mesh: initialized with in-memory Pub/Sub (standalone mode)")
	}

	return manager
}

// listenRedis listens to Redis Pub/Sub channels for messages from other pods.
func (m *MeshManager) listenRedis(ctx context.Context) {
	pubsub := m.redisCli.PSubscribe(ctx, "mesh_room:*")
	defer pubsub.Close()

	ch := pubsub.Channel()
	for msg := range ch {
		// Extract room ID from channel name
		roomID := msg.Channel[len("mesh_room:"):]

		var meshMsg MeshMessage
		if err := json.Unmarshal([]byte(msg.Payload), &meshMsg); err != nil {
			slog.Error("mesh: failed to unmarshal redis message", "err", err)
			continue
		}

		// Broadcast to local connected websocket clients
		m.localBroadcast(roomID, meshMsg)
	}
}

// getOrCreateRoom returns the room for the given ID, creating it if it doesn't exist.
func (m *MeshManager) getOrCreateRoom(roomID string) *MeshRoom {
	m.mu.Lock()
	defer m.mu.Unlock()
	room, exists := m.rooms[roomID]
	if !exists {
		room = &MeshRoom{
			ID:      roomID,
			Clients: make(map[*websocket.Conn]bool),
		}
		m.rooms[roomID] = room
	}
	return room
}

// SubscribeHandler handles WebSocket connections for subscribing to a room.
func (m *MeshManager) SubscribeHandler(w http.ResponseWriter, r *http.Request) {
	roomID := r.PathValue("room_id")
	if roomID == "" {
		http.Error(w, "room_id is required", http.StatusBadRequest)
		return
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		slog.Error("mesh: failed to upgrade connection", "err", err)
		return
	}

	room := m.getOrCreateRoom(roomID)
	room.mu.Lock()
	room.Clients[conn] = true
	room.mu.Unlock()

	slog.Info("mesh: client subscribed", "room_id", roomID)

	go func() {
		defer func() {
			room.mu.Lock()
			delete(room.Clients, conn)
			room.mu.Unlock()
			conn.Close()
		}()
		for {
			_, _, err := conn.ReadMessage()
			if err != nil {
				if websocket.IsUnexpectedCloseError(err, websocket.CloseGoingAway, websocket.CloseAbnormalClosure) {
					slog.Error("mesh: unexpected close error", "err", err)
				}
				break
			}
		}
	}()
}

// PublishHandler handles HTTP requests to publish a message to a room.
func (m *MeshManager) PublishHandler(w http.ResponseWriter, r *http.Request) {
	roomID := r.PathValue("room_id")
	if roomID == "" {
		http.Error(w, "room_id is required", http.StatusBadRequest)
		return
	}

	var msg MeshMessage
	if err := json.NewDecoder(r.Body).Decode(&msg); err != nil {
		http.Error(w, "invalid request body", http.StatusBadRequest)
		return
	}

	if msg.Timestamp.IsZero() {
		msg.Timestamp = time.Now().UTC()
	}

	m.Broadcast(roomID, msg)
	w.WriteHeader(http.StatusOK)
}

// Broadcast sends a message to the room. In cloud mode, it publishes to Redis.
func (m *MeshManager) Broadcast(roomID string, msg MeshMessage) {
	if m.redisCli != nil {
		payload, err := json.Marshal(msg)
		if err != nil {
			slog.Error("mesh: failed to marshal message for redis", "err", err)
			return
		}
		err = m.redisCli.Publish(context.Background(), "mesh_room:"+roomID, payload).Err()
		if err != nil {
			slog.Error("mesh: failed to publish to redis", "err", err)
		}
		return
	}

	// Standalone mode: just local broadcast
	m.localBroadcast(roomID, msg)
}

// localBroadcast sends a message to all local websocket clients subscribed to the room.
func (m *MeshManager) localBroadcast(roomID string, msg MeshMessage) {
	m.mu.Lock()
	room, exists := m.rooms[roomID]
	m.mu.Unlock()

	if !exists {
		return
	}

	room.mu.Lock()
	// Copy clients to avoid holding lock during network I/O
	clients := make([]*websocket.Conn, 0, len(room.Clients))
	for client := range room.Clients {
		clients = append(clients, client)
	}
	room.mu.Unlock()

	payload, err := json.Marshal(msg)
	if err != nil {
		slog.Error("mesh: failed to marshal message", "err", err)
		return
	}

	for _, client := range clients {
		if err := client.WriteMessage(websocket.TextMessage, payload); err != nil {
			slog.Error("mesh: failed to write message to client", "err", err)
			room.mu.Lock()
			client.Close()
			delete(room.Clients, client)
			room.mu.Unlock()
		}
	}
}
