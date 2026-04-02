package orchestration

import (
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"os"
	"sync"
	"time"

	"github.com/gorilla/mux"
	"github.com/gorilla/websocket"
	"github.com/redis/go-redis/v9"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool { return true },
}

// MeshMessage represents a teammate mesh broadcast
type MeshMessage struct {
	SenderID  string    `json:"sender_id"`
	Role      string    `json:"role"`
	Content   string    `json:"content"`
	Timestamp time.Time `json:"timestamp"`
}

// MeshRoom represents an isolated pub/sub room for agents
type MeshRoom struct {
	ID          string
	clients     map[*websocket.Conn]bool
	clientsLock sync.RWMutex
}

// MeshManager manages the teammate mesh API
type MeshManager struct {
	redisClient  *redis.Client
	isCloudMode  bool
	rooms        map[string]*MeshRoom
	roomsLock    sync.RWMutex
	localPubSub  map[string][]chan MeshMessage
	localPubLock sync.RWMutex
}

// NewMeshManager creates a new MeshManager
func NewMeshManager(redisClient *redis.Client) *MeshManager {
	return &MeshManager{
		redisClient: redisClient,
		isCloudMode: os.Getenv("OHC_MULTITENANT") == "true",
		rooms:       make(map[string]*MeshRoom),
		localPubSub: make(map[string][]chan MeshMessage),
	}
}

// getRoom retrieves or creates a MeshRoom
func (m *MeshManager) getRoom(roomID string) *MeshRoom {
	m.roomsLock.Lock()
	defer m.roomsLock.Unlock()

	room, exists := m.rooms[roomID]
	if !exists {
		room = &MeshRoom{
			ID:      roomID,
			clients: make(map[*websocket.Conn]bool),
		}
		m.rooms[roomID] = room
	}
	return room
}

// HandleSubscribe sets up a websocket connection for a room
func (m *MeshManager) HandleSubscribe(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	roomID := vars["room_id"]
	if roomID == "" {
		http.Error(w, "room_id is required", http.StatusBadRequest)
		return
	}

	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		slog.Error("Failed to upgrade websocket", "error", err)
		return
	}

	room := m.getRoom(roomID)
	room.clientsLock.Lock()
	room.clients[conn] = true
	room.clientsLock.Unlock()

	ctx, cancel := context.WithCancel(r.Context())
	defer func() {
		cancel()
		room.clientsLock.Lock()
		delete(room.clients, conn)
		room.clientsLock.Unlock()
		conn.Close()
	}()

	// Setup Pub/Sub listener
	if m.isCloudMode && m.redisClient != nil {
		go m.listenRedis(ctx, roomID, conn)
	} else {
		go m.listenLocal(ctx, roomID, conn)
	}

	// Keep alive loop
	for {
		if _, _, err := conn.ReadMessage(); err != nil {
			break
		}
	}
}

// listenRedis listens to a Redis Pub/Sub channel
func (m *MeshManager) listenRedis(ctx context.Context, roomID string, conn *websocket.Conn) {
	pubsub := m.redisClient.Subscribe(ctx, "mesh:room:"+roomID)
	defer pubsub.Close()

	ch := pubsub.Channel()
	for {
		select {
		case <-ctx.Done():
			return
		case msg := <-ch:
			var meshMsg MeshMessage
			if err := json.Unmarshal([]byte(msg.Payload), &meshMsg); err == nil {
				conn.WriteJSON(meshMsg)
			}
		}
	}
}

// listenLocal listens to local pub/sub
func (m *MeshManager) listenLocal(ctx context.Context, roomID string, conn *websocket.Conn) {
	ch := make(chan MeshMessage, 10)

	m.localPubLock.Lock()
	m.localPubSub[roomID] = append(m.localPubSub[roomID], ch)
	m.localPubLock.Unlock()

	defer func() {
		m.localPubLock.Lock()
		subscribers := m.localPubSub[roomID]
		for i, sub := range subscribers {
			if sub == ch {
				m.localPubSub[roomID] = append(subscribers[:i], subscribers[i+1:]...)
				break
			}
		}
		m.localPubLock.Unlock()
	}()

	for {
		select {
		case <-ctx.Done():
			return
		case msg := <-ch:
			conn.WriteJSON(msg)
		}
	}
}

// HandlePublish handles HTTP POST to publish messages to a room
func (m *MeshManager) HandlePublish(w http.ResponseWriter, r *http.Request) {
	vars := mux.Vars(r)
	roomID := vars["room_id"]
	if roomID == "" {
		http.Error(w, "room_id is required", http.StatusBadRequest)
		return
	}

	var msg MeshMessage
	if err := json.NewDecoder(r.Body).Decode(&msg); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	if msg.Timestamp.IsZero() {
		msg.Timestamp = time.Now()
	}

	payloadBytes, _ := json.Marshal(msg)

	if m.isCloudMode && m.redisClient != nil {
		err := m.redisClient.Publish(r.Context(), "mesh:room:"+roomID, string(payloadBytes)).Err()
		if err != nil {
			http.Error(w, "failed to publish", http.StatusInternalServerError)
			return
		}
	} else {
		m.localPubLock.RLock()
		subscribers := m.localPubSub[roomID]
		for _, ch := range subscribers {
			select {
			case ch <- msg:
			default: // Avoid blocking
			}
		}
		m.localPubLock.RUnlock()
	}

	w.WriteHeader(http.StatusOK)
}
