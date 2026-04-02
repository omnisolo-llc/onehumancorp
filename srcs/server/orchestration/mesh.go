package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"sync"
	"time"

	"github.com/gorilla/websocket"
	"github.com/redis/rueidis"
)

// MeshMessage represents a realtime message sent over the mesh.
type MeshMessage struct {
	SenderID  string    `json:"sender_id"`
	Role      string    `json:"role"`
	Content   string    `json:"content"`
	Timestamp time.Time `json:"timestamp"`
}

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow all for now
	},
}

// TeammateMesh manages real-time pub/sub for agents
type TeammateMesh struct {
	redisClient rueidis.Client
	isCloud     bool

	// In-memory pub/sub for standalone mode
	mu          sync.RWMutex
	subscribers map[string]map[*websocket.Conn]chan []byte
}

// NewTeammateMesh creates a new mesh instance.
func NewTeammateMesh(redisURL string) (*TeammateMesh, error) {
	isCloud := os.Getenv("OHC_MULTITENANT") == "true"

	tm := &TeammateMesh{
		isCloud:     isCloud,
		subscribers: make(map[string]map[*websocket.Conn]chan []byte),
	}

	if isCloud && redisURL != "" {
		opt, err := rueidis.ParseURL(redisURL)
		if err != nil {
			return nil, fmt.Errorf("failed to parse redis url: %w", err)
		}
		c, err := rueidis.NewClient(opt)
		if err != nil {
			return nil, fmt.Errorf("failed to connect to redis: %w", err)
		}
		tm.redisClient = c
	}

	return tm, nil
}

// HandleWebSocket handles incoming WS connections for a specific room.
func (tm *TeammateMesh) HandleWebSocket(w http.ResponseWriter, r *http.Request, roomID string) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		slog.Error("mesh: upgrade error", "err", err)
		return
	}

	msgChan := make(chan []byte, 256)
	tm.subscribe(roomID, conn, msgChan)
	defer tm.unsubscribe(roomID, conn)

	ctx, cancel := context.WithCancel(r.Context())
	defer cancel()

	// Start a single write goroutine to prevent concurrent writes
	go func() {
		defer conn.Close()
		for {
			select {
			case <-ctx.Done():
				return
			case payload, ok := <-msgChan:
				if !ok {
					return
				}
				_ = conn.WriteMessage(websocket.TextMessage, payload)
			}
		}
	}()

	// If cloud, subscribe to redis channel
	if tm.isCloud && tm.redisClient != nil {
		go func() {
			err := tm.redisClient.Receive(ctx, tm.redisClient.B().Subscribe().Channel(roomID).Build(), func(msg rueidis.PubSubMessage) {
				select {
				case msgChan <- []byte(msg.Message):
				default:
					// Drop message if channel is full to prevent blocking
				}
			})
			if err != nil {
				slog.Error("mesh: redis subscribe error", "err", err)
			}
		}()
	}

	for {
		_, payload, err := conn.ReadMessage()
		if err != nil {
			break
		}

		// Ensure it's valid JSON
		var msg MeshMessage
		if err := json.Unmarshal(payload, &msg); err != nil {
			continue
		}
		msg.Timestamp = time.Now()
		broadcastPayload, _ := json.Marshal(msg)

		tm.Publish(ctx, roomID, string(broadcastPayload))
	}
}

// Publish broadcasts a message to a room.
func (tm *TeammateMesh) Publish(ctx context.Context, roomID, message string) error {
	if tm.isCloud && tm.redisClient != nil {
		cmd := tm.redisClient.B().Publish().Channel(roomID).Message(message).Build()
		return tm.redisClient.Do(ctx, cmd).Error()
	}

	tm.mu.RLock()
	defer tm.mu.RUnlock()

	subs := tm.subscribers[roomID]
	payload := []byte(message)
	for _, ch := range subs {
		// Non-blocking write to prevent slow clients from blocking the publisher
		select {
		case ch <- payload:
		default:
		}
	}
	return nil
}

func (tm *TeammateMesh) subscribe(roomID string, conn *websocket.Conn, msgChan chan []byte) {
	tm.mu.Lock()
	defer tm.mu.Unlock()

	if tm.subscribers[roomID] == nil {
		tm.subscribers[roomID] = make(map[*websocket.Conn]chan []byte)
	}
	tm.subscribers[roomID][conn] = msgChan
}

func (tm *TeammateMesh) unsubscribe(roomID string, conn *websocket.Conn) {
	tm.mu.Lock()
	defer tm.mu.Unlock()

	if tm.subscribers[roomID] != nil {
		if ch, exists := tm.subscribers[roomID][conn]; exists {
			close(ch)
			delete(tm.subscribers[roomID], conn)
		}
		if len(tm.subscribers[roomID]) == 0 {
			delete(tm.subscribers, roomID)
		}
	}
	_ = conn.Close()
}
