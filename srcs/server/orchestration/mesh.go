package orchestration

import (
	"context"
	pb "github.com/onehumancorp/mono/srcs/proto"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"

	"sync"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/lib/resilience"
	"github.com/redis/go-redis/v9"
	"github.com/redis/rueidis"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
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

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/orchestration")
	meshMsgThroughput, _ = meter.Int64Counter("mesh.message.throughput")
	meshLatency, _ = meter.Float64Histogram("mesh.latency", metric.WithUnit("ms"))
)

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
	isCloud := envBoolDefault("OHC_MULTITENANT", true)

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

type AgentCapabilities struct {
	AgentID              string   `json:"agent_id"`
	SupportedSkills      []string `json:"supported_skills"`
	MaxConcurrentTasks   int32    `json:"max_concurrent_tasks"`
}



type TeammateMesh interface {
	BroadcastTask(ctx context.Context, task Task) error
	SubscribeTasks(ctx context.Context) (<-chan Task, error)
	BroadcastCoordination(ctx context.Context, msg MeshMessage) error
	SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error)
}

type RedisTeammateMesh struct {
	client rueidis.Client
}

type RedisMeshTransport struct {
	client rueidis.Client
}

func NewRedisMeshTransport(redisURL string) (*RedisMeshTransport, error) {
	opt, err := rueidis.ParseURL(redisURL)
	if err != nil {
		return nil, err
	}
	c, err := rueidis.NewClient(opt)
	if err != nil {
		return nil, err
	}
	return &RedisMeshTransport{client: c}, nil
}

func (rm *RedisMeshTransport) BroadcastTask(ctx context.Context, task Task) error {
	data, err := json.Marshal(task)
	if err != nil {
		return err
	}
	cmd := rm.client.B().Publish().Channel("mesh:tasks").Message(string(data)).Build()
	return meshWithRetry(ctx, 3, func() error {
		return rm.client.Do(ctx, cmd).Error()
	})
}

func (rm *RedisMeshTransport) SubscribeTasks(ctx context.Context) (<-chan Task, error) {
	ch := make(chan Task, 100)

	go func() {
		err := rm.client.Receive(ctx, rm.client.B().Subscribe().Channel("mesh:tasks").Build(), func(msg rueidis.PubSubMessage) {
			var t Task
			if err := json.Unmarshal([]byte(msg.Message), &t); err == nil {
				select {
				case ch <- t:
				default:
					slog.Warn("RedisMeshTransport.SubscribeTasks channel full, dropping message")
				}
			}
		})
		if err != nil && err != context.Canceled {
			slog.Error("RedisMeshTransport.SubscribeTasks error", "err", err)
		}
		close(ch)
	}()
	return ch, nil
}

func (rm *RedisMeshTransport) BroadcastCoordination(ctx context.Context, msg MeshMessage) error {
	data, err := json.Marshal(msg)
	if err != nil {
		return err
	}
	cmd := rm.client.B().Publish().Channel("mesh:coordination").Message(string(data)).Build()
	return meshWithRetry(ctx, 3, func() error {
		return rm.client.Do(ctx, cmd).Error()
	})
}

func (rm *RedisMeshTransport) SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error) {
	ch := make(chan MeshMessage, 100)

	go func() {
		err := rm.client.Receive(ctx, rm.client.B().Subscribe().Channel("mesh:coordination").Build(), func(msg rueidis.PubSubMessage) {
			var m MeshMessage
			if err := json.Unmarshal([]byte(msg.Message), &m); err == nil {
				select {
				case ch <- m:
				default:
					slog.Warn("RedisMeshTransport.SubscribeCoordination channel full, dropping message")
				}
			}
		})
		if err != nil && err != context.Canceled {
			slog.Error("RedisMeshTransport.SubscribeCoordination error", "err", err)
		}
		close(ch)
	}()
	return ch, nil
}

func (rm *RedisMeshTransport) AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error {
	data, err := json.Marshal(caps)
	if err != nil {
		return err
	}
	cmd := rm.client.B().Publish().Channel("mesh:capabilities").Message(string(data)).Build()
	return meshWithRetry(ctx, 3, func() error {
		return rm.client.Do(ctx, cmd).Error()
	})
}

func (rm *RedisMeshTransport) SubscribeCapabilities(ctx context.Context) (<-chan pb.AgentCapabilities, error) {
	ch := make(chan pb.AgentCapabilities, 100)
	go func() {
		err := rm.client.Receive(ctx, rm.client.B().Subscribe().Channel("mesh:capabilities").Build(), func(msg rueidis.PubSubMessage) {
			var c pb.AgentCapabilities
			if err := json.Unmarshal([]byte(msg.Message), &c); err == nil {
				select {
				case ch <- c:
				default:
					slog.Warn("RedisMeshTransport.SubscribeCapabilities channel full, dropping message")
				}
			}
		})
		if err != nil && err != context.Canceled {
			slog.Error("RedisMeshTransport.SubscribeCapabilities error", "err", err)
		}
		close(ch)
	}()
	return ch, nil
}

func (rm *RedisMeshTransport) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	cmd := rm.client.B().Publish().Channel("mesh:events:" + topic).Message(string(payload)).Build()
	return meshWithRetry(ctx, 3, func() error {
		return rm.client.Do(ctx, cmd).Error()
	})
}

func (rm *RedisMeshTransport) SubscribeMeshEvents(ctx context.Context, topic string) (<-chan []byte, error) {
	ch := make(chan []byte, 100)
	go func() {
		err := rm.client.Receive(ctx, rm.client.B().Subscribe().Channel("mesh:events:" + topic).Build(), func(msg rueidis.PubSubMessage) {
			select {
			case ch <- []byte(msg.Message):
			default:
				slog.Warn("RedisMeshTransport.SubscribeMeshEvents channel full, dropping message")
			}
		})
		if err != nil && err != context.Canceled {
			slog.Error("RedisMeshTransport.SubscribeMeshEvents error", "err", err)
		}
		close(ch)
	}()
	return ch, nil
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

func meshWithRetry(ctx context.Context, maxRetries int, fn func() error) error {
	return resilience.WithRetry(ctx, maxRetries, 50*time.Millisecond, func(retryCtx context.Context) error {
		return fn()
	})
}

func (rm *RedisTeammateMesh) BroadcastTask(ctx context.Context, task Task) error {
	data, err := json.Marshal(task)
	if err != nil {
		return err
	}
	cmd := rm.client.B().Publish().Channel("mesh:tasks").Message(string(data)).Build()
	return meshWithRetry(ctx, 3, func() error {
		return rm.client.Do(ctx, cmd).Error()
	})
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

func (rm *RedisTeammateMesh) BroadcastCoordination(ctx context.Context, msg MeshMessage) error {
	data, err := json.Marshal(msg)
	if err != nil {
		return err
	}
	cmd := rm.client.B().Publish().Channel("mesh:coordination").Message(string(data)).Build()
	return meshWithRetry(ctx, 3, func() error {
		return rm.client.Do(ctx, cmd).Error()
	})
}

func (rm *RedisTeammateMesh) SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error) {
	ch := make(chan MeshMessage, 100)

	go func() {
		err := rm.client.Receive(ctx, rm.client.B().Subscribe().Channel("mesh:coordination").Build(), func(msg rueidis.PubSubMessage) {
			var m MeshMessage
			if err := json.Unmarshal([]byte(msg.Message), &m); err == nil {
				select {
				case ch <- m:
				default:
					slog.Warn("RedisTeammateMesh.SubscribeCoordination channel full, dropping message")
				}
			}
		})
		if err != nil && err != context.Canceled {
			slog.Error("RedisTeammateMesh.SubscribeCoordination error", "err", err)
		}
		close(ch)
	}()
	return ch, nil
}





type MeshTransport interface {
	BroadcastTask(ctx context.Context, task Task) error
	SubscribeTasks(ctx context.Context) (<-chan Task, error)
	BroadcastCoordination(ctx context.Context, msg MeshMessage) error
	SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error)
	AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error
	SubscribeCapabilities(ctx context.Context) (<-chan pb.AgentCapabilities, error)
	BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error
	SubscribeMeshEvents(ctx context.Context, topic string) (<-chan []byte, error)
}

const numShards = 16

type MemoryMeshTransport struct {
	db                  db.Provider
	broadcast           []chan Task
	persist             []chan Task
	mu                  []sync.RWMutex
	subs                []map[chan Task]struct{}
	coordBroadcast      []chan MeshMessage
	coordSubs           []map[chan MeshMessage]struct{}
	coordMu             []sync.RWMutex
	capsBroadcast       []chan pb.AgentCapabilities
	capsSubs            []map[chan pb.AgentCapabilities]struct{}
	capsMu              []sync.RWMutex
	eventsBroadcast     map[string][]chan []byte
	eventsSubs          map[string][]map[chan []byte]struct{}
	eventsMu            map[string][]sync.RWMutex
	eventsGlobalMu      sync.RWMutex
}

func NewMemoryMeshTransport(provider db.Provider) *MemoryMeshTransport {
	lm := &MemoryMeshTransport{
		db:                  provider,
		broadcast:           make([]chan Task, numShards),
		persist:             make([]chan Task, numShards),
		mu:                  make([]sync.RWMutex, numShards),
		subs:                make([]map[chan Task]struct{}, numShards),
		coordBroadcast:      make([]chan MeshMessage, numShards),
		coordSubs:           make([]map[chan MeshMessage]struct{}, numShards),
		coordMu:             make([]sync.RWMutex, numShards),
		capsBroadcast:       make([]chan pb.AgentCapabilities, numShards),
		capsSubs:            make([]map[chan pb.AgentCapabilities]struct{}, numShards),
		capsMu:              make([]sync.RWMutex, numShards),
		eventsBroadcast:     make(map[string][]chan []byte),
		eventsSubs:          make(map[string][]map[chan []byte]struct{}),
		eventsMu:            make(map[string][]sync.RWMutex),
	}

	for i := 0; i < numShards; i++ {
		lm.broadcast[i] = make(chan Task, 10000)
		lm.persist[i] = make(chan Task, 10000)
		lm.subs[i] = make(map[chan Task]struct{})
		lm.coordBroadcast[i] = make(chan MeshMessage, 10000)
		lm.coordSubs[i] = make(map[chan MeshMessage]struct{})
		lm.capsBroadcast[i] = make(chan pb.AgentCapabilities, 10000)
		lm.capsSubs[i] = make(map[chan pb.AgentCapabilities]struct{})

		for j := 0; j < 4; j++ {
			go lm.run(i)
			go lm.persistWorker(i)
		}
		go lm.runCoord(i)
		go lm.runCaps(i)
	}
	return lm
}

func (lm *MemoryMeshTransport) getShard(key string) int {
	var hash uint32
	for i := 0; i < len(key); i++ {
		hash = hash*31 + uint32(key[i])
	}
	return int(hash % numShards)
}

func (lm *MemoryMeshTransport) persistWorker(shardIdx int) {
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
			slog.Error("MemoryMeshTransport persist error", "err", err, "taskID", task.TaskID)
		}
	}
}

func (lm *MemoryMeshTransport) BroadcastTask(ctx context.Context, task Task) error {
	shardIdx := lm.getShard(task.TaskID)

	err := meshWithRetry(ctx, 3, func() error {
		select {
		case lm.persist[shardIdx] <- task:
			return nil
		default:
			return fmt.Errorf("MemoryMeshTransport persist channel full")
		}
	})
	if err != nil {
		slog.Warn("MemoryMeshTransport persist channel full, dropping persistence for task after retries", "taskID", task.TaskID)
	}

	_ = meshWithRetry(ctx, 3, func() error {
		select {
		case lm.broadcast[shardIdx] <- task:
			return nil
		default:
			return fmt.Errorf("MemoryMeshTransport broadcast channel full")
		}
	})

	return nil
}

func (lm *MemoryMeshTransport) SubscribeTasks(ctx context.Context) (<-chan Task, error) {
	ch := make(chan Task, 100)

	for i := 0; i < numShards; i++ {
		lm.mu[i].Lock()
		lm.subs[i][ch] = struct{}{}
		lm.mu[i].Unlock()
	}

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

func (lm *MemoryMeshTransport) run(shardIdx int) {
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

func (lm *MemoryMeshTransport) BroadcastCoordination(ctx context.Context, msg MeshMessage) error {
	shardIdx := lm.getShard(msg.AgentID)

	err := meshWithRetry(ctx, 3, func() error {
		select {
		case lm.coordBroadcast[shardIdx] <- msg:
			return nil
		default:
			return fmt.Errorf("MemoryMeshTransport coord broadcast channel full")
		}
	})

	if err != nil {
		slog.Warn("MemoryMeshTransport coord broadcast channel full, dropping message after retries")
	}
	return nil
}

func (lm *MemoryMeshTransport) SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error) {
	ch := make(chan MeshMessage, 100)

	for i := 0; i < numShards; i++ {
		lm.coordMu[i].Lock()
		lm.coordSubs[i][ch] = struct{}{}
		lm.coordMu[i].Unlock()
	}

	go func() {
		<-ctx.Done()
		for i := 0; i < numShards; i++ {
			lm.coordMu[i].Lock()
			delete(lm.coordSubs[i], ch)
			lm.coordMu[i].Unlock()
		}
		close(ch)
	}()

	return ch, nil
}

func (lm *MemoryMeshTransport) runCoord(shardIdx int) {
	for msg := range lm.coordBroadcast[shardIdx] {
		lm.coordMu[shardIdx].RLock()
		for ch := range lm.coordSubs[shardIdx] {
			select {
			case ch <- msg:
			default:
			}
		}
		lm.coordMu[shardIdx].RUnlock()
	}
}

func (lm *MemoryMeshTransport) AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error {
	shardIdx := lm.getShard(caps.GetAgentId())

	err := meshWithRetry(ctx, 3, func() error {
		select {
		case lm.capsBroadcast[shardIdx] <- caps:
			return nil
		default:
			return fmt.Errorf("MemoryMeshTransport caps broadcast channel full")
		}
	})

	if err != nil {
		slog.Warn("MemoryMeshTransport caps broadcast channel full, dropping message after retries")
	}
	return nil
}

func (lm *MemoryMeshTransport) SubscribeCapabilities(ctx context.Context) (<-chan pb.AgentCapabilities, error) {
	ch := make(chan pb.AgentCapabilities, 100)

	for i := 0; i < numShards; i++ {
		lm.capsMu[i].Lock()
		lm.capsSubs[i][ch] = struct{}{}
		lm.capsMu[i].Unlock()
	}

	go func() {
		<-ctx.Done()
		for i := 0; i < numShards; i++ {
			lm.capsMu[i].Lock()
			delete(lm.capsSubs[i], ch)
			lm.capsMu[i].Unlock()
		}
		close(ch)
	}()

	return ch, nil
}

func (lm *MemoryMeshTransport) runCaps(shardIdx int) {
	for msg := range lm.capsBroadcast[shardIdx] {
		lm.capsMu[shardIdx].RLock()
		for ch := range lm.capsSubs[shardIdx] {
			select {
			case ch <- msg:
			default:
			}
		}
		lm.capsMu[shardIdx].RUnlock()
	}
}

func (lm *MemoryMeshTransport) initTopic(topic string) {
	lm.eventsGlobalMu.Lock()
	defer lm.eventsGlobalMu.Unlock()
	if _, ok := lm.eventsBroadcast[topic]; !ok {
		lm.eventsBroadcast[topic] = make([]chan []byte, numShards)
		lm.eventsSubs[topic] = make([]map[chan []byte]struct{}, numShards)
		lm.eventsMu[topic] = make([]sync.RWMutex, numShards)
		for i := 0; i < numShards; i++ {
			lm.eventsBroadcast[topic][i] = make(chan []byte, 10000)
			lm.eventsSubs[topic][i] = make(map[chan []byte]struct{})
			go lm.runEvents(topic, i)
		}
	}
}

func (lm *MemoryMeshTransport) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	lm.initTopic(topic)
	shardIdx := lm.getShard(string(payload)) // hash payload for random shard since event ID isn't directly available

	lm.eventsGlobalMu.RLock()
	broadcastChan := lm.eventsBroadcast[topic][shardIdx]
	lm.eventsGlobalMu.RUnlock()

	err := meshWithRetry(ctx, 3, func() error {
		select {
		case broadcastChan <- payload:
			return nil
		default:
			return fmt.Errorf("MemoryMeshTransport events broadcast channel full")
		}
	})

	if err != nil {
		slog.Warn("MemoryMeshTransport events broadcast channel full, dropping message after retries")
	}
	return nil
}

func (lm *MemoryMeshTransport) SubscribeMeshEvents(ctx context.Context, topic string) (<-chan []byte, error) {
	lm.initTopic(topic)
	ch := make(chan []byte, 100)

	lm.eventsGlobalMu.RLock()
	muArray := lm.eventsMu[topic]
	subsArray := lm.eventsSubs[topic]
	for i := 0; i < numShards; i++ {
		muArray[i].Lock()
		subsArray[i][ch] = struct{}{}
		muArray[i].Unlock()
	}
	lm.eventsGlobalMu.RUnlock()

	go func() {
		<-ctx.Done()
		lm.eventsGlobalMu.RLock()
		if muArray, ok := lm.eventsMu[topic]; ok {
			subsArray := lm.eventsSubs[topic]
			for i := 0; i < numShards; i++ {
				muArray[i].Lock()
				delete(subsArray[i], ch)
				muArray[i].Unlock()
			}
		}
		lm.eventsGlobalMu.RUnlock()
		close(ch)
	}()

	return ch, nil
}

func (lm *MemoryMeshTransport) runEvents(topic string, shardIdx int) {
	lm.eventsGlobalMu.RLock()
	broadcastChan := lm.eventsBroadcast[topic][shardIdx]
	lm.eventsGlobalMu.RUnlock()

	for msg := range broadcastChan {
		lm.eventsGlobalMu.RLock()
		if muArray, ok := lm.eventsMu[topic]; ok {
			subsArray := lm.eventsSubs[topic]
			muArray[shardIdx].RLock()
			for ch := range subsArray[shardIdx] {
				select {
				case ch <- msg:
				default:
				}
			}
			muArray[shardIdx].RUnlock()
		}
		lm.eventsGlobalMu.RUnlock()
	}
}


type LocalTeammateMesh struct {
	db                  db.Provider
	broadcast           []chan Task
	persist             []chan Task
	mu                  []sync.RWMutex
	subs                []map[chan Task]struct{}
	coordBroadcast      []chan MeshMessage
	coordSubs           []map[chan MeshMessage]struct{}
	coordMu             []sync.RWMutex

	capsBroadcast       []chan pb.AgentCapabilities
	capsSubs            []map[chan pb.AgentCapabilities]struct{}
	capsMu              []sync.RWMutex

	eventsBroadcast     map[string][]chan []byte
	eventsSubs          map[string][]map[chan []byte]struct{}
	eventsMu            map[string][]sync.RWMutex
	eventsGlobalMu      sync.RWMutex
}

func NewLocalTeammateMesh(provider db.Provider) *LocalTeammateMesh {
	lm := &LocalTeammateMesh{
		db:                  provider,
		broadcast:           make([]chan Task, numShards),
		persist:             make([]chan Task, numShards),
		mu:                  make([]sync.RWMutex, numShards),
		subs:                make([]map[chan Task]struct{}, numShards),
		coordBroadcast:      make([]chan MeshMessage, numShards),
		coordSubs:           make([]map[chan MeshMessage]struct{}, numShards),
		coordMu:             make([]sync.RWMutex, numShards),
		capsBroadcast:       make([]chan pb.AgentCapabilities, numShards),
		capsSubs:            make([]map[chan pb.AgentCapabilities]struct{}, numShards),
		capsMu:              make([]sync.RWMutex, numShards),
		eventsBroadcast:     make(map[string][]chan []byte),
		eventsSubs:          make(map[string][]map[chan []byte]struct{}),
		eventsMu:            make(map[string][]sync.RWMutex),
	}

	// Phase 2 (Implementation): "Parallel Execution" hooks using Worker Threads for the OHC "Team Mesh"
	// We use sharding by taskID/agentID to reduce lock contention and maximize parallel throughput.
	for i := 0; i < numShards; i++ {
		lm.broadcast[i] = make(chan Task, 10000)
		lm.persist[i] = make(chan Task, 10000)
		lm.subs[i] = make(map[chan Task]struct{})
		lm.coordBroadcast[i] = make(chan MeshMessage, 10000)
		lm.coordSubs[i] = make(map[chan MeshMessage]struct{})
		lm.capsBroadcast[i] = make(chan pb.AgentCapabilities, 10000)
		lm.capsSubs[i] = make(map[chan pb.AgentCapabilities]struct{})

		// Spawn multiple worker threads per shard
		for j := 0; j < 4; j++ {
			go lm.run(i)
			go lm.persistWorker(i)
		}
		go lm.runCoord(i)
		go lm.runCaps(i)
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
	// Use backoff retry for persistence channel
	err := meshWithRetry(ctx, 3, func() error {
		select {
		case lm.persist[shardIdx] <- task:
			return nil
		default:
			return fmt.Errorf("LocalTeammateMesh persist channel full")
		}
	})
	if err != nil {
		slog.Warn("LocalTeammateMesh persist channel full, dropping persistence for task after retries", "taskID", task.TaskID)
	}

	// Broadcast locally to the specific shard
	// Use backoff retry for broadcast channel
	_ = meshWithRetry(ctx, 3, func() error {
		select {
		case lm.broadcast[shardIdx] <- task:
			return nil
		default:
			return fmt.Errorf("LocalTeammateMesh broadcast channel full")
		}
	})

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

func (lm *LocalTeammateMesh) BroadcastCoordination(ctx context.Context, msg MeshMessage) error {
	shardIdx := lm.getShard(msg.AgentID)

	// Use backoff retry for coord broadcast channel
	err := meshWithRetry(ctx, 3, func() error {
		select {
		case lm.coordBroadcast[shardIdx] <- msg:
			return nil
		default:
			return fmt.Errorf("LocalTeammateMesh coord broadcast channel full")
		}
	})

	if err != nil {
		slog.Warn("LocalTeammateMesh coord broadcast channel full, dropping message after retries")
	}
	return nil
}

func (lm *LocalTeammateMesh) SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error) {
	ch := make(chan MeshMessage, 100)

	for i := 0; i < numShards; i++ {
		lm.coordMu[i].Lock()
		lm.coordSubs[i][ch] = struct{}{}
		lm.coordMu[i].Unlock()
	}

	go func() {
		<-ctx.Done()
		for i := 0; i < numShards; i++ {
			lm.coordMu[i].Lock()
			delete(lm.coordSubs[i], ch)
			lm.coordMu[i].Unlock()
		}
		close(ch)
	}()

	return ch, nil
}

func (lm *LocalTeammateMesh) runCoord(shardIdx int) {
	for msg := range lm.coordBroadcast[shardIdx] {
		lm.coordMu[shardIdx].RLock()
		for ch := range lm.coordSubs[shardIdx] {
			select {
			case ch <- msg:
			default:
			}
		}
		lm.coordMu[shardIdx].RUnlock()
	}
}


func (lm *LocalTeammateMesh) AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error {
	shardIdx := lm.getShard(caps.GetAgentId())

	err := meshWithRetry(ctx, 3, func() error {
		select {
		case lm.capsBroadcast[shardIdx] <- caps:
			return nil
		default:
			return fmt.Errorf("LocalTeammateMesh caps broadcast channel full")
		}
	})

	if err != nil {
		slog.Warn("LocalTeammateMesh caps broadcast channel full, dropping message after retries")
	}
	return nil
}

func (lm *LocalTeammateMesh) SubscribeCapabilities(ctx context.Context) (<-chan pb.AgentCapabilities, error) {
	ch := make(chan pb.AgentCapabilities, 100)

	for i := 0; i < numShards; i++ {
		lm.capsMu[i].Lock()
		lm.capsSubs[i][ch] = struct{}{}
		lm.capsMu[i].Unlock()
	}

	go func() {
		<-ctx.Done()
		for i := 0; i < numShards; i++ {
			lm.capsMu[i].Lock()
			delete(lm.capsSubs[i], ch)
			lm.capsMu[i].Unlock()
		}
		close(ch)
	}()

	return ch, nil
}

func (lm *LocalTeammateMesh) runCaps(shardIdx int) {
	for msg := range lm.capsBroadcast[shardIdx] {
		lm.capsMu[shardIdx].RLock()
		for ch := range lm.capsSubs[shardIdx] {
			select {
			case ch <- msg:
			default:
			}
		}
		lm.capsMu[shardIdx].RUnlock()
	}
}

func (lm *LocalTeammateMesh) initTopic(topic string) {
	lm.eventsGlobalMu.Lock()
	defer lm.eventsGlobalMu.Unlock()
	if _, ok := lm.eventsBroadcast[topic]; !ok {
		lm.eventsBroadcast[topic] = make([]chan []byte, numShards)
		lm.eventsSubs[topic] = make([]map[chan []byte]struct{}, numShards)
		lm.eventsMu[topic] = make([]sync.RWMutex, numShards)
		for i := 0; i < numShards; i++ {
			lm.eventsBroadcast[topic][i] = make(chan []byte, 10000)
			lm.eventsSubs[topic][i] = make(map[chan []byte]struct{})
			go lm.runEvents(topic, i)
		}
	}
}

func (lm *LocalTeammateMesh) BroadcastMeshEvent(ctx context.Context, topic string, payload []byte) error {
	lm.initTopic(topic)
	shardIdx := lm.getShard(string(payload))

	lm.eventsGlobalMu.RLock()
	broadcastChan := lm.eventsBroadcast[topic][shardIdx]
	lm.eventsGlobalMu.RUnlock()

	err := meshWithRetry(ctx, 3, func() error {
		select {
		case broadcastChan <- payload:
			return nil
		default:
			return fmt.Errorf("LocalTeammateMesh events broadcast channel full")
		}
	})

	if err != nil {
		slog.Warn("LocalTeammateMesh events broadcast channel full, dropping message after retries")
	}
	return nil
}

func (lm *LocalTeammateMesh) SubscribeMeshEvents(ctx context.Context, topic string) (<-chan []byte, error) {
	lm.initTopic(topic)
	ch := make(chan []byte, 100)

	lm.eventsGlobalMu.RLock()
	muArray := lm.eventsMu[topic]
	subsArray := lm.eventsSubs[topic]
	for i := 0; i < numShards; i++ {
		muArray[i].Lock()
		subsArray[i][ch] = struct{}{}
		muArray[i].Unlock()
	}
	lm.eventsGlobalMu.RUnlock()

	go func() {
		<-ctx.Done()
		lm.eventsGlobalMu.RLock()
		if muArray, ok := lm.eventsMu[topic]; ok {
			subsArray := lm.eventsSubs[topic]
			for i := 0; i < numShards; i++ {
				muArray[i].Lock()
				delete(subsArray[i], ch)
				muArray[i].Unlock()
			}
		}
		lm.eventsGlobalMu.RUnlock()
		close(ch)
	}()

	return ch, nil
}

func (lm *LocalTeammateMesh) runEvents(topic string, shardIdx int) {
	lm.eventsGlobalMu.RLock()
	broadcastChan := lm.eventsBroadcast[topic][shardIdx]
	lm.eventsGlobalMu.RUnlock()

	for msg := range broadcastChan {
		lm.eventsGlobalMu.RLock()
		if muArray, ok := lm.eventsMu[topic]; ok {
			subsArray := lm.eventsSubs[topic]
			muArray[shardIdx].RLock()
			for ch := range subsArray[shardIdx] {
				select {
				case ch <- msg:
				default:
				}
			}
			muArray[shardIdx].RUnlock()
		}
		lm.eventsGlobalMu.RUnlock()
	}
}
