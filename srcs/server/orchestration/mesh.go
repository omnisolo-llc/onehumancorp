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
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/go-redis/v9"
	"github.com/redis/rueidis"
)

// MeshMessage represents a realtime message sent over the mesh.
// OHC-SIP requires agent_id, action, status at root.
type MeshMessage struct {
	AgentID   string    `json:"agent_id"`
	Action    string    `json:"action"`
	Status    string    `json:"status"`
	SenderID  string    `json:"sender_id,omitempty"`
	Role      string    `json:"role,omitempty"`
	Content   string    `json:"content,omitempty"`
	Timestamp time.Time `json:"timestamp"`
}

var upgrader = websocket.Upgrader{
	CheckOrigin: func(r *http.Request) bool {
		return true // Allow all for now
	},
}

// LegacyTeammateMesh manages real-time pub/sub for agents
type LegacyTeammateMesh struct {
	redisClient *redis.Client
	isCloud     bool

	// In-memory pub/sub for standalone mode
	mu          sync.RWMutex
	subscribers map[string]map[*websocket.Conn]chan []byte
}

// NewLegacyTeammateMesh creates a new mesh instance.
func NewLegacyTeammateMesh(redisURL string) (*LegacyTeammateMesh, error) {
	isCloud := os.Getenv("OHC_MULTITENANT") == "true"

	tm := &LegacyTeammateMesh{
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
func (tm *LegacyTeammateMesh) HandleWebSocket(w http.ResponseWriter, r *http.Request, roomID string) {
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
func (tm *LegacyTeammateMesh) Publish(ctx context.Context, roomID, message string) error {
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

func (tm *LegacyTeammateMesh) subscribe(roomID string, conn *websocket.Conn, msgChan chan []byte) {
	tm.mu.Lock()
	defer tm.mu.Unlock()

	if tm.subscribers[roomID] == nil {
		tm.subscribers[roomID] = make(map[*websocket.Conn]chan []byte)
	}
	tm.subscribers[roomID][conn] = msgChan
}

func (tm *LegacyTeammateMesh) unsubscribe(roomID string, conn *websocket.Conn) {
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

type Task struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
	TaskID  string `json:"task_id"`
}

type TeammateMesh interface {
	BroadcastTask(ctx context.Context, task Task) error
	SubscribeTasks(ctx context.Context) (<-chan Task, error)
}

type RedisTeammateMesh struct {
	client rueidis.Client
}

func NewRedisTeammateMesh(redisURL string) (*RedisTeammateMesh, error) {
	opt, err := rueidis.ParseURL(redisURL)
	if err != nil {
		return nil, err
	}
	c, err := rueidis.NewClient(opt)
	if err != nil {
		return nil, err
	}
	return &RedisTeammateMesh{client: c}, nil
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
	ch := make(chan Task, 100)

	go func() {
		err := rm.client.Receive(ctx, rm.client.B().Subscribe().Channel("mesh:tasks").Build(), func(msg rueidis.PubSubMessage) {
			var t Task
			if err := json.Unmarshal([]byte(msg.Message), &t); err == nil {
				select {
				case ch <- t:
				default:
					slog.Warn("RedisTeammateMesh.SubscribeTasks channel full, dropping message")
				}
			}
		})
		if err != nil && err != context.Canceled {
			slog.Error("RedisTeammateMesh.SubscribeTasks error", "err", err)
		}
		close(ch)
	}()
	return ch, nil
}

const numShards = 16

type LocalTeammateMesh struct {
	db        db.Provider
	broadcast []chan Task
	persist   []chan Task
	mu        []sync.RWMutex
	subs      []map[chan Task]struct{}
}

func NewLocalTeammateMesh(provider db.Provider) *LocalTeammateMesh {
	lm := &LocalTeammateMesh{
		db:        provider,
		broadcast: make([]chan Task, numShards),
		persist:   make([]chan Task, numShards),
		mu:        make([]sync.RWMutex, numShards),
		subs:      make([]map[chan Task]struct{}, numShards),
	}

	// Phase 2 (Implementation): "Parallel Execution" hooks using Worker Threads for the OHC "Team Mesh"
	// We use sharding by taskID/agentID to reduce lock contention and maximize parallel throughput.
	for i := 0; i < numShards; i++ {
		lm.broadcast[i] = make(chan Task, 10000)
		lm.persist[i] = make(chan Task, 10000)
		lm.subs[i] = make(map[chan Task]struct{})

		// Spawn multiple worker threads per shard
		for j := 0; j < 4; j++ {
			go lm.run(i)
			go lm.persistWorker(i)
		}
	}
	return lm
}

func (lm *LocalTeammateMesh) getShard(key string) int {
	var hash uint32
	for i := 0; i < len(key); i++ {
		hash = hash*31 + uint32(key[i])
	}
	return int(hash % numShards)
}

func (lm *LocalTeammateMesh) persistWorker(shardIdx int) {
	query := `
		INSERT INTO shared_tasks (id, title, status, agent_id, organization_id)
		VALUES ($1, $2, $3, $4, 'system')
		ON CONFLICT(id) DO UPDATE SET
			status = excluded.status,
			agent_id = excluded.agent_id,
			updated_at = CURRENT_TIMESTAMP
	`
	for task := range lm.persist[shardIdx] {
		_, err := lm.db.Exec(context.Background(), query, task.TaskID, task.Action, task.Status, task.AgentID)
		if err != nil {
			slog.Error("LocalTeammateMesh persist error", "err", err, "taskID", task.TaskID)
		}
	}
}

func (lm *LocalTeammateMesh) BroadcastTask(ctx context.Context, task Task) error {
	shardIdx := lm.getShard(task.TaskID)

	// Offload persistence to worker threads within the specific shard
	select {
	case lm.persist[shardIdx] <- task:
	default:
		slog.Warn("LocalTeammateMesh persist channel full, dropping persistence for task", "taskID", task.TaskID)
	}

	// Broadcast locally to the specific shard
	select {
	case lm.broadcast[shardIdx] <- task:
	default:
	}
	return nil
}

func (lm *LocalTeammateMesh) SubscribeTasks(ctx context.Context) (<-chan Task, error) {
	// A subscriber needs to receive tasks from all shards.
	// To keep it simple but performant, we add the subscriber to all shards.
	ch := make(chan Task, 100)

	for i := 0; i < numShards; i++ {
		lm.mu[i].Lock()
		lm.subs[i][ch] = struct{}{}
		lm.mu[i].Unlock()
	}

	// Handle context cancellation
	go func() {
		<-ctx.Done()
		for i := 0; i < numShards; i++ {
			lm.mu[i].Lock()
			delete(lm.subs[i], ch)
			lm.mu[i].Unlock()
		}
		close(ch)
	}()

	return ch, nil
}

func (lm *LocalTeammateMesh) run(shardIdx int) {
	for msg := range lm.broadcast[shardIdx] {
		lm.mu[shardIdx].RLock()
		for ch := range lm.subs[shardIdx] {
			select {
			case ch <- msg:
			default:
			}
		}
		lm.mu[shardIdx].RUnlock()
	}
}
