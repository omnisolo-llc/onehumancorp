package kairos

import (
	"context"
	"encoding/json"

	"github.com/redis/go-redis/v9"
)

type TaskEvent struct {
	MissionID string `json:"mission_id"`
	EventType string `json:"event_type"`
	Payload   string `json:"payload"`
}

type TeammateMesh interface {
	PublishEvent(ctx context.Context, channel string, event *TaskEvent) error
	SubscribeToChannel(ctx context.Context, channel string) (<-chan *TaskEvent, error)
	Close() error
}

type teammateMeshImpl struct {
	client *redis.Client
}

func NewTeammateMesh(redisURL string) (TeammateMesh, error) {
	opts, err := redis.ParseURL(redisURL)
	if err != nil {
		return nil, err
	}

	client := redis.NewClient(opts)
	// Ping to verify connection
	if err := client.Ping(context.Background()).Err(); err != nil {
		return nil, err
	}

	return &teammateMeshImpl{
		client: client,
	}, nil
}

func (m *teammateMeshImpl) PublishEvent(ctx context.Context, channel string, event *TaskEvent) error {
data, _ := json.Marshal(event)
	return m.client.Publish(ctx, channel, data).Err()
}

func (m *teammateMeshImpl) SubscribeToChannel(ctx context.Context, channel string) (<-chan *TaskEvent, error) {
	pubsub := m.client.Subscribe(ctx, channel)

	// Ensure we can receive at least one message to verify subscription
	_, err := pubsub.Receive(ctx)
	if err != nil {
		return nil, err
	}

	ch := pubsub.Channel()
	eventCh := make(chan *TaskEvent, 100)

	go func() {
		defer close(eventCh)
		defer pubsub.Close()

		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				var event TaskEvent
				if err := json.Unmarshal([]byte(msg.Payload), &event); err == nil {
					eventCh <- &event
				} else {
					_ = err
				}
			}
		}
	}()

	return eventCh, nil
}

func (m *teammateMeshImpl) Close() error {
	return m.client.Close()
}
