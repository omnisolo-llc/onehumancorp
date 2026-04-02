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
	"github.com/redis/go-redis/v9"
	"github.com/redis/rueidis"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// Task represents the payload for Teammate Mesh broadcasts.
type Task struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
	TaskID  string `json:"task_id"`
}

// TeammateMesh manages real-time pub/sub for agents
type TeammateMesh interface {
	BroadcastTask(ctx context.Context, task Task) error
	SubscribeTasks(ctx context.Context) (<-chan Task, error)
}

// RedisTeammateMesh implements TeammateMesh for Cloud mode.
type RedisTeammateMesh struct {
	client rueidis.Client
}

func NewRedisTeammateMesh(redisURL string) (*RedisTeammateMesh, error) {
	opts, err := rueidis.ParseURL(redisURL)
	if err != nil {
		return nil, fmt.Errorf("failed to parse redis url: %w", err)
	}
	client, err := rueidis.NewClient(opts)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to redis: %w", err)
	}
	return &RedisTeammateMesh{client: client}, nil
}

func (rm *RedisTeammateMesh) BroadcastTask(ctx context.Context, task Task) error {
	data, err := json.Marshal(task)
	if err != nil {
		return err
	}
	cmd := rm.client.B().Publish().Channel("mesh:tasks").Message(string(data)).Build()
	return rm.client.Do(ctx, cmd).Error()
}

func (rm *RedisTeammateMesh) SubscribeTasks(ctx context.Context) (<-chan Task, error) {
	// rueidis pub/sub needs a dedicated connection
	ch := make(chan Task, 100)
	go func() {
		// A full implementation using rueidis subscription
		err := rm.client.Receive(ctx, rm.client.B().Subscribe().Channel("mesh:tasks").Build(), func(msg rueidis.PubSubMessage) {
			var t Task
			if err := json.Unmarshal([]byte(msg.Message), &t); err == nil {
				ch <- t
			}
		})
		if err != nil {
			slog.Error("RedisTeammateMesh subscription error", "error", err)
		}
		close(ch)
	}()
	return ch, nil
}

// LocalTeammateMesh implements TeammateMesh for Standalone Mode.
type LocalTeammateMesh struct {
	db          db.Provider
	mu          sync.RWMutex
	subscribers []chan Task
}

func NewLocalTeammateMesh(provider db.Provider) *LocalTeammateMesh {
	return &LocalTeammateMesh{
		db: provider,
	}
}

func (lm *LocalTeammateMesh) BroadcastTask(ctx context.Context, task Task) error {
	// Persist to SQLite shared_tasks
	query := `
		INSERT INTO shared_tasks (id, title, status, assigned_agent_id, created_at, updated_at)
		VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
		ON CONFLICT(id) DO UPDATE SET
			status = excluded.status,
			assigned_agent_id = excluded.assigned_agent_id,
			updated_at = CURRENT_TIMESTAMP
	`
	_, err := lm.db.Exec(ctx, query, task.TaskID, task.Action, task.Status, task.AgentID)
	if err != nil {
		return fmt.Errorf("failed to persist broadcast to shared_tasks: %w", err)
	}

	lm.mu.RLock()
	defer lm.mu.RUnlock()
	for _, ch := range lm.subscribers {
		select {
		case ch <- task:
		default:
		}
	}
	return nil
}

func (lm *LocalTeammateMesh) SubscribeTasks(ctx context.Context) (<-chan Task, error) {
	ch := make(chan Task, 100)
	lm.mu.Lock()
	lm.subscribers = append(lm.subscribers, ch)
	lm.mu.Unlock()

	go func() {
		<-ctx.Done()
		lm.mu.Lock()
		defer lm.mu.Unlock()
		for i, sub := range lm.subscribers {
			if sub == ch {
				lm.subscribers = append(lm.subscribers[:i], lm.subscribers[i+1:]...)
				close(ch)
				break
			}
		}
	}()
	return ch, nil
}

// ------------------------------------------------------------------------------------------------
// MeshManager (previously TeammateMesh) manages real-time pub/sub for UI connections.
// ------------------------------------------------------------------------------------------------

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

// MeshManager manages real-time pub/sub for agents
type MeshManager struct {
	redisClient *redis.Client
	isCloud     bool

	// In-memory pub/sub for standalone mode
	mu          sync.RWMutex
	subscribers map[string]map[*websocket.Conn]chan []byte
}

// NewMeshManager creates a new mesh instance.
func NewMeshManager(redisURL string) (*MeshManager, error) {
	isCloud := os.Getenv("OHC_MULTITENANT") == "true"

	tm := &MeshManager{
		isCloud:     isCloud,
		subscribers: make(map[string]map[*websocket.Conn]chan []byte),
	}

	if isCloud && redisURL != "" {
		opt, err := redis.ParseURL(redisURL)
		if err != nil {
			return nil, fmt.Errorf("failed to parse redis url: %w", err)
		}
		tm.redisClient = redis.NewClient(opt)
		if err := tm.redisClient.Ping(context.Background()).Err(); err != nil {
			return nil, fmt.Errorf("failed to connect to redis: %w", err)
		}
	}

	return tm, nil
}

// HandleWebSocket handles incoming WS connections for a specific room.
func (tm *MeshManager) HandleWebSocket(w http.ResponseWriter, r *http.Request, roomID string) {
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
	var pubsub *redis.PubSub
	if tm.isCloud && tm.redisClient != nil {
		pubsub = tm.redisClient.Subscribe(ctx, roomID)
		defer pubsub.Close()

		go func() {
			ch := pubsub.Channel()
			for {
				select {
				case <-ctx.Done():
					return
				case msg := <-ch:
					// Send to the write goroutine
					select {
					case msgChan <- []byte(msg.Payload):
					default:
						// Drop message if channel is full to prevent blocking
					}
				}
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
func (tm *MeshManager) Publish(ctx context.Context, roomID, message string) error {
	if tm.isCloud && tm.redisClient != nil {
		return tm.redisClient.Publish(ctx, roomID, message).Err()
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

func (tm *MeshManager) subscribe(roomID string, conn *websocket.Conn, msgChan chan []byte) {
	tm.mu.Lock()
	defer tm.mu.Unlock()

	if tm.subscribers[roomID] == nil {
		tm.subscribers[roomID] = make(map[*websocket.Conn]chan []byte)
	}
	tm.subscribers[roomID][conn] = msgChan
}

func (tm *MeshManager) unsubscribe(roomID string, conn *websocket.Conn) {
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
