package kairos

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"sync"
	"time"

	"github.com/google/uuid"
	"github.com/redis/go-redis/v9"
	"google.golang.org/protobuf/proto"
)

type Subscription interface {
	Unsubscribe() error
}

type AgentPresence struct {
	AgentID string
	Status  string
}

type TeammateMesh interface {
	Publish(ctx context.Context, topic string, payload []byte) error
	Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error)
	AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error)
	ReleaseLock(ctx context.Context, key string) error
	RegisterPresence(ctx context.Context, agentID string, status string) error
	GetActiveAgents(ctx context.Context) ([]AgentPresence, error)
	Acknowledge(ctx context.Context, messageID string) error
	PublishWithAck(ctx context.Context, topic string, payload []byte) error
	PublishStateHandoff(ctx context.Context, payload []byte) error
	SubscribeStateHandoff(ctx context.Context, handler func(msg []byte)) (Subscription, error)
	Ping(ctx context.Context) error
	StartHealthResponder(ctx context.Context) (func(), error)
}

type redisSubscription struct {
	pubsub *redis.PubSub
}

func (r *redisSubscription) Unsubscribe() error {
	return r.pubsub.Close()
}

type RedisTeammateMesh struct {
	client redis.UniversalClient
}

func NewRedisTeammateMesh(client redis.UniversalClient) *RedisTeammateMesh {
	return &RedisTeammateMesh{
		client: client,
	}
}

func (m *RedisTeammateMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	return m.client.Publish(ctx, topic, payload).Err()
}

func (m *RedisTeammateMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	pubsub := m.client.Subscribe(ctx, topic)

	// Ensure subscription is active
	_, err := pubsub.Receive(ctx)
	if err != nil {
		return nil, err
	}

	go func() {
		ch := pubsub.Channel()
		for msg := range ch {
			handler([]byte(msg.Payload))
		}
	}()

	return &redisSubscription{pubsub: pubsub}, nil
}

func (m *RedisTeammateMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	return m.client.SetNX(ctx, key, "1", ttl).Result()
}

func (m *RedisTeammateMesh) ReleaseLock(ctx context.Context, key string) error {
	return m.client.Del(ctx, key).Err()
}

func (m *RedisTeammateMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	return m.client.HSet(ctx, "mesh:presence", agentID, status).Err()
}

func (m *RedisTeammateMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	res, err := m.client.HGetAll(ctx, "mesh:presence").Result()
	if err != nil {
		return nil, err
	}

	var agents []AgentPresence
	for k, v := range res {
		agents = append(agents, AgentPresence{
			AgentID: k,
			Status:  v,
		})
	}
	return agents, nil
}

func (m *RedisTeammateMesh) Acknowledge(ctx context.Context, messageID string) error {
	return m.client.Publish(ctx, "mesh:ack:"+messageID, []byte("ack")).Err()
}

func (m *RedisTeammateMesh) PublishWithAck(ctx context.Context, topic string, payload []byte) error {
	msgID := uuid.New().String()
	ackTopic := "mesh:ack:" + msgID

	ackCh := make(chan struct{})
	sub, err := m.Subscribe(ctx, ackTopic, func(msg []byte) {
		close(ackCh)
	})
	if err != nil {
		return err
	}
	defer sub.Unsubscribe()

	retries := 0
	backoff := 200 * time.Millisecond

	for {
		if retries > 10 {
			return fmt.Errorf("timeout waiting for ack on topic %s", topic)
		}

		event := &TeammateMeshEvent{
			AgentId: "sys",
			Action:  topic,
			Status:  "ok",
			Payload: payload,
			MsgId:   msgID,
		}
		eventBytes, err := proto.Marshal(event)
		if err != nil {
			return err
		}
		err = m.Publish(ctx, topic, eventBytes)
		if err != nil {
			return err
		}

		select {
		case <-ackCh:
			return nil
		case <-ctx.Done():
			return ctx.Err()
		case <-time.After(backoff):
			retries++
			backoff *= 2
		}
	}
}

func (m *RedisTeammateMesh) PublishStateHandoff(ctx context.Context, payload []byte) error {
	return m.PublishWithAck(ctx, "mesh:state:handoff", payload)
}

func (m *RedisTeammateMesh) SubscribeStateHandoff(ctx context.Context, handler func(msg []byte)) (Subscription, error) {
	return m.Subscribe(ctx, "mesh:state:handoff", handler)
}

func (m *RedisTeammateMesh) Ping(ctx context.Context) error {
	return m.PublishWithAck(ctx, "mesh:health:ping", []byte("ping"))
}

func (m *RedisTeammateMesh) StartHealthResponder(ctx context.Context) (func(), error) {
	sub, err := m.Subscribe(ctx, "mesh:health:ping", func(msg []byte) {
		var event TeammateMeshEvent
		if err := proto.Unmarshal(msg, &event); err == nil {
			m.Acknowledge(context.Background(), event.MsgId)
		}
	})
	if err != nil {
		return nil, err
	}
	return func() { sub.Unsubscribe() }, nil
}

type localSubInfo struct {
	id       int64
	handler  func(msg []byte)
	cancelCh chan struct{}
}

type localSubscription struct {
	mesh  *LocalTeammateMesh
	topic string
	id    int64
}

func (s *localSubscription) Unsubscribe() error {
	s.mesh.mu.Lock()
	defer s.mesh.mu.Unlock()

	subs := s.mesh.subs[s.topic]
	for i, sub := range subs {
		if sub.id == s.id {
			close(sub.cancelCh)
			s.mesh.subs[s.topic] = append(subs[:i], subs[i+1:]...)
			break
		}
	}
	return nil
}

type LocalTeammateMesh struct {
	mu       sync.RWMutex
	subs     map[string][]localSubInfo
	locks    map[string]time.Time
	presence map[string]string
	nextID   int64
}

func NewLocalTeammateMesh() *LocalTeammateMesh {
	return &LocalTeammateMesh{
		subs:     make(map[string][]localSubInfo),
		locks:    make(map[string]time.Time),
		presence: make(map[string]string),
	}
}

func (m *LocalTeammateMesh) Publish(ctx context.Context, topic string, payload []byte) error {
	m.mu.RLock()
	subs := m.subs[topic]
	subsCopy := make([]localSubInfo, len(subs))
	copy(subsCopy, subs)
	m.mu.RUnlock()

	dataCopy := make([]byte, len(payload))
	copy(dataCopy, payload)

	for _, sub := range subsCopy {
		go sub.handler(dataCopy)
	}

	return nil
}

func (m *LocalTeammateMesh) Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	id := m.nextID
	m.nextID++

	cancelCh := make(chan struct{})
	m.subs[topic] = append(m.subs[topic], localSubInfo{
		id:       id,
		handler:  handler,
		cancelCh: cancelCh,
	})

	return &localSubscription{
		mesh:  m,
		topic: topic,
		id:    id,
	}, nil
}

func (m *LocalTeammateMesh) AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	lockFile := filepath.Join(os.TempDir(), "ohc_mesh_lock_"+key)
	f, err := os.OpenFile(lockFile, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
	if err != nil {
		info, err := os.Stat(lockFile)
		if err == nil {
			if time.Since(info.ModTime()) > ttl {
				os.Remove(lockFile)
				f, err = os.OpenFile(lockFile, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
				if err == nil {
					f.Close()
					return true, nil
				}
			}
		}
		return false, nil
	}
	f.Close()
	return true, nil
}

func (m *LocalTeammateMesh) ReleaseLock(ctx context.Context, key string) error {
	lockFile := filepath.Join(os.TempDir(), "ohc_mesh_lock_"+key)
	os.Remove(lockFile)
	return nil
}

func (m *LocalTeammateMesh) RegisterPresence(ctx context.Context, agentID string, status string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.presence[agentID] = status
	return nil
}

func (m *LocalTeammateMesh) GetActiveAgents(ctx context.Context) ([]AgentPresence, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()

	var agents []AgentPresence
	for k, v := range m.presence {
		agents = append(agents, AgentPresence{
			AgentID: k,
			Status:  v,
		})
	}
	return agents, nil
}

func (m *LocalTeammateMesh) Acknowledge(ctx context.Context, messageID string) error {
	return m.Publish(ctx, "mesh:ack:"+messageID, []byte("ack"))
}

func (m *LocalTeammateMesh) PublishWithAck(ctx context.Context, topic string, payload []byte) error {
	msgID := uuid.New().String()
	ackTopic := "mesh:ack:" + msgID

	ackCh := make(chan struct{})
	sub, err := m.Subscribe(ctx, ackTopic, func(msg []byte) {
		close(ackCh)
	})
	if err != nil {
		return err
	}
	defer sub.Unsubscribe()

	retries := 0
	backoff := 20 * time.Millisecond

	for {
		if retries > 10 {
			return fmt.Errorf("timeout waiting for ack on topic %s", topic)
		}

		event := &TeammateMeshEvent{
			AgentId: "sys",
			Action:  topic,
			Status:  "ok",
			Payload: payload,
			MsgId:   msgID,
		}
		eventBytes, err := proto.Marshal(event)
		if err != nil {
			return err
		}
		err = m.Publish(ctx, topic, eventBytes)
		if err != nil {
			return err
		}

		select {
		case <-ackCh:
			return nil
		case <-ctx.Done():
			return ctx.Err()
		case <-time.After(backoff):
			retries++
			backoff *= 2
		}
	}
}

func (m *LocalTeammateMesh) PublishStateHandoff(ctx context.Context, payload []byte) error {
	return m.PublishWithAck(ctx, "mesh:state:handoff", payload)
}

func (m *LocalTeammateMesh) SubscribeStateHandoff(ctx context.Context, handler func(msg []byte)) (Subscription, error) {
	return m.Subscribe(ctx, "mesh:state:handoff", handler)
}

func (m *LocalTeammateMesh) Ping(ctx context.Context) error {
	return m.PublishWithAck(ctx, "mesh:health:ping", []byte("ping"))
}

func (m *LocalTeammateMesh) StartHealthResponder(ctx context.Context) (func(), error) {
	sub, err := m.Subscribe(ctx, "mesh:health:ping", func(msg []byte) {
		var event TeammateMeshEvent
		if err := proto.Unmarshal(msg, &event); err == nil {
			m.Acknowledge(context.Background(), event.MsgId)
		}
	})
	if err != nil {
		return nil, err
	}
	return func() { sub.Unsubscribe() }, nil
}
