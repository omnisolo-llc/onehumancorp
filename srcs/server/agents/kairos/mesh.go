package kairos

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/redis/go-redis/v9"
)

type TeammateMesh interface {
	PublishTaskCreated(ctx context.Context, mission *Mission) error
	PublishStatusUpdate(ctx context.Context, missionID uuid.UUID, status string) error
	PublishDirectMessage(ctx context.Context, agentID string, message []byte) error

	SubscribeTaskCreated(ctx context.Context) (<-chan *Mission, error)
	SubscribeStatusUpdate(ctx context.Context) (<-chan StatusUpdateEvent, error)
	SubscribeDirectMessages(ctx context.Context, agentID string) (<-chan []byte, error)
}

type StatusUpdateEvent struct {
	MissionID uuid.UUID `json:"mission_id"`
	Status    string    `json:"status"`
	Timestamp time.Time `json:"timestamp"`
}

// PubSubSubscription abstracts the redis.PubSub struct for hermetic testing.
type PubSubSubscription interface {
	Receive(ctx context.Context) (interface{}, error)
	Channel() <-chan *redis.Message
	Close() error
}

type realPubSub struct {
	ps *redis.PubSub
}

func (r *realPubSub) Receive(ctx context.Context) (interface{}, error) {
	return r.ps.Receive(ctx)
}

func (r *realPubSub) Channel() <-chan *redis.Message {
	return r.ps.Channel()
}

func (r *realPubSub) Close() error {
	return r.ps.Close()
}

// RedisPubSub abstracts redis.Client for hermetic testing.
type RedisPubSub interface {
	Publish(ctx context.Context, channel string, message interface{}) *redis.IntCmd
	Subscribe(ctx context.Context, channels ...string) PubSubSubscription
}

type realRedisClient struct {
	client *redis.Client
}

func (r *realRedisClient) Publish(ctx context.Context, channel string, message interface{}) *redis.IntCmd {
	return r.client.Publish(ctx, channel, message)
}

func (r *realRedisClient) Subscribe(ctx context.Context, channels ...string) PubSubSubscription {
	return &realPubSub{ps: r.client.Subscribe(ctx, channels...)}
}

type teammateMeshImpl struct {
	client RedisPubSub
}

const (
	TaskCreatedChannel = "mesh:events:task_created"
	StatusUpdateChannel = "mesh:events:status_update"
	DirectMessagePrefix = "mesh:mail:agent_"
)

func NewTeammateMesh(client *redis.Client) TeammateMesh {
	return &teammateMeshImpl{
		client: &realRedisClient{client: client},
	}
}

// NewTeammateMeshWithInterface creates a mesh with an abstracted client for unit testing.
func NewTeammateMeshWithInterface(client RedisPubSub) TeammateMesh {
	return &teammateMeshImpl{
		client: client,
	}
}

func (m *teammateMeshImpl) PublishTaskCreated(ctx context.Context, mission *Mission) error {
	payload, err := json.Marshal(mission)
	if err != nil {
		return fmt.Errorf("failed to marshal mission: %w", err)
	}
	return m.client.Publish(ctx, TaskCreatedChannel, payload).Err()
}

func (m *teammateMeshImpl) PublishStatusUpdate(ctx context.Context, missionID uuid.UUID, status string) error {
	event := StatusUpdateEvent{
		MissionID: missionID,
		Status:    status,
		Timestamp: time.Now().UTC(),
	}
	payload, err := json.Marshal(event)
	if err != nil {
		return fmt.Errorf("failed to marshal status update event: %w", err)
	}
	return m.client.Publish(ctx, StatusUpdateChannel, payload).Err()
}

func (m *teammateMeshImpl) PublishDirectMessage(ctx context.Context, agentID string, message []byte) error {
	channel := DirectMessagePrefix + agentID
	return m.client.Publish(ctx, channel, message).Err()
}

func (m *teammateMeshImpl) SubscribeTaskCreated(ctx context.Context) (<-chan *Mission, error) {
	pubsub := m.client.Subscribe(ctx, TaskCreatedChannel)

	_, err := pubsub.Receive(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to subscribe: %w", err)
	}

	ch := pubsub.Channel()
	missionCh := make(chan *Mission)

	go func() {
		defer pubsub.Close()
		defer close(missionCh)
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				var mission Mission
				if err := json.Unmarshal([]byte(msg.Payload), &mission); err != nil {
					continue
				}

				// Optional non-blocking send to avoid unread blocks, but typical usage is consuming directly
				select {
				case missionCh <- &mission:
				case <-ctx.Done():
					return
				}
			}
		}
	}()

	return missionCh, nil
}

func (m *teammateMeshImpl) SubscribeStatusUpdate(ctx context.Context) (<-chan StatusUpdateEvent, error) {
	pubsub := m.client.Subscribe(ctx, StatusUpdateChannel)
	_, err := pubsub.Receive(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to subscribe: %w", err)
	}

	ch := pubsub.Channel()
	eventCh := make(chan StatusUpdateEvent)

	go func() {
		defer pubsub.Close()
		defer close(eventCh)
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				var event StatusUpdateEvent
				if err := json.Unmarshal([]byte(msg.Payload), &event); err != nil {
					continue
				}

				select {
				case eventCh <- event:
				case <-ctx.Done():
					return
				}
			}
		}
	}()

	return eventCh, nil
}

func (m *teammateMeshImpl) SubscribeDirectMessages(ctx context.Context, agentID string) (<-chan []byte, error) {
	channel := DirectMessagePrefix + agentID
	pubsub := m.client.Subscribe(ctx, channel)
	_, err := pubsub.Receive(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to subscribe: %w", err)
	}

	ch := pubsub.Channel()
	msgCh := make(chan []byte)

	go func() {
		defer pubsub.Close()
		defer close(msgCh)
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				select {
				case msgCh <- []byte(msg.Payload):
				case <-ctx.Done():
					return
				}
			}
		}
	}()

	return msgCh, nil
}
