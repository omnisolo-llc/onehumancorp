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
	"github.com/redis/rueidis"
)

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow all for now
	},
}

type MeshMessage struct {
	SenderID  string `json:"sender_id"`
	Role      string `json:"role"`
	Content   string `json:"content"`
	Timestamp string `json:"timestamp"`
}

type Room struct {
	ID      string
	Clients map[*websocket.Conn]bool
	mu      sync.Mutex
}

type MeshServer struct {
	rooms       map[string]*Room
	mu          sync.Mutex
	redisClient rueidis.Client
	isRedis     bool
}

func NewMeshServer() *MeshServer {
	redisURL := os.Getenv("REDIS_URL")
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"

	var client rueidis.Client
	var err error
	var isRedis bool

	if redisURL != "" && !isStandalone {
		// Use Rueidis
		client, err = rueidis.NewClient(rueidis.ClientOption{
			InitAddress: []string{redisURL}, // In a real app we might need to parse URL
		})
		if err != nil {
			slog.Error("Failed to connect to Redis for mesh, falling back to local memory", "error", err)
		} else {
			isRedis = true
			slog.Info("Configured mesh with Redis pub/sub")
		}
	} else {
		slog.Info("Configured mesh with local memory pub/sub")
	}

	return &MeshServer{
		rooms:       make(map[string]*Room),
		redisClient: client,
		isRedis:     isRedis,
	}
}

func (ms *MeshServer) getRoom(roomID string) *Room {
	ms.mu.Lock()
	defer ms.mu.Unlock()

	room, ok := ms.rooms[roomID]
	if !ok {
		room = &Room{
			ID:      roomID,
			Clients: make(map[*websocket.Conn]bool),
		}
		ms.rooms[roomID] = room

		if ms.isRedis {
			go ms.subscribeRedis(roomID)
		}
	}
	return room
}

func (ms *MeshServer) subscribeRedis(roomID string) {
	ctx := context.Background()
	channel := "mesh:" + roomID

	err := ms.redisClient.Receive(ctx, ms.redisClient.B().Subscribe().Channel(channel).Build(), func(msg rueidis.PubSubMessage) {
		var meshMsg MeshMessage
		if err := json.Unmarshal([]byte(msg.Message), &meshMsg); err != nil {
			slog.Error("Failed to unmarshal redis pub/sub message", "error", err)
			return
		}

		ms.broadcastLocal(roomID, meshMsg)
	})

	if err != nil {
		slog.Error("Redis subscription ended", "room", roomID, "error", err)
	}
}

func (ms *MeshServer) HandleSubscribe(w http.ResponseWriter, r *http.Request, roomID string) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		slog.Error("Failed to upgrade to websocket", "error", err)
		return
	}

	room := ms.getRoom(roomID)
	room.mu.Lock()
	room.Clients[conn] = true
	room.mu.Unlock()

	slog.Info("Client connected to mesh room", "room", roomID)

	defer func() {
		room.mu.Lock()
		delete(room.Clients, conn)
		room.mu.Unlock()
		conn.Close()
	}()

	for {
		var msg MeshMessage
		err := conn.ReadJSON(&msg)
		if err != nil {
			slog.Debug("Client disconnected from mesh room", "room", roomID)
			break
		}

		if msg.Timestamp == "" {
			msg.Timestamp = time.Now().UTC().Format(time.RFC3339)
		}

		ms.Broadcast(roomID, msg)
	}
}

func (ms *MeshServer) Broadcast(roomID string, msg MeshMessage) {
	if ms.isRedis {
		ctx := context.Background()
		channel := "mesh:" + roomID

		data, err := json.Marshal(msg)
		if err != nil {
			slog.Error("Failed to marshal mesh message", "error", err)
			return
		}

		err = ms.redisClient.Do(ctx, ms.redisClient.B().Publish().Channel(channel).Message(string(data)).Build()).Error()
		if err != nil {
			slog.Error("Failed to publish to redis", "error", err)
		}
	} else {
		// Local only broadcast
		ms.broadcastLocal(roomID, msg)
	}
}

func (ms *MeshServer) broadcastLocal(roomID string, msg MeshMessage) {
	room := ms.getRoom(roomID)
	room.mu.Lock()
	defer room.mu.Unlock()

	for conn := range room.Clients {
		err := conn.WriteJSON(msg)
		if err != nil {
			slog.Error("Failed to write to websocket", "error", err)
			conn.Close()
			delete(room.Clients, conn)
		}
	}
}
