package kairos

import (
	"context"
	"fmt"
	"time"

	"github.com/go-redsync/redsync/v4"
	"github.com/go-redsync/redsync/v4/redis/goredis/v9"
	"github.com/redis/go-redis/v9"
)

const (
	ChannelTaskCreated  = "mesh:events:task_created"
	ChannelStatusUpdate = "mesh:events:status_update"
	ChannelMailAgent    = "mesh:mail:agent_%s"
)

// TeammateMesh provides real-time communication and distributed locking for agents
type TeammateMesh interface {
	PublishTaskCreated(ctx context.Context, missionID string) error
	PublishStatusUpdate(ctx context.Context, missionID string, status MissionStatus) error
	PublishMail(ctx context.Context, agentID string, message string) error

	SubscribeTaskCreated(ctx context.Context) <-chan string
	SubscribeStatusUpdate(ctx context.Context) <-chan string
	SubscribeMail(ctx context.Context, agentID string) <-chan string

	AcquireLock(ctx context.Context, resourceName string, ttl time.Duration) (*redsync.Mutex, error)
}

type redisTeammateMesh struct {
	client *redis.Client
	rsync  *redsync.Redsync
}

// NewTeammateMesh creates a new instance of TeammateMesh using Redis
func NewTeammateMesh(client *redis.Client) TeammateMesh {
	pool := goredis.NewPool(client)
	rs := redsync.New(pool)
	return &redisTeammateMesh{
		client: client,
		rsync:  rs,
	}
}

func (m *redisTeammateMesh) PublishTaskCreated(ctx context.Context, missionID string) error {
	return m.client.Publish(ctx, ChannelTaskCreated, missionID).Err()
}

func (m *redisTeammateMesh) PublishStatusUpdate(ctx context.Context, missionID string, status MissionStatus) error {
	payload := fmt.Sprintf("%s:%s", missionID, status)
	return m.client.Publish(ctx, ChannelStatusUpdate, payload).Err()
}

func (m *redisTeammateMesh) PublishMail(ctx context.Context, agentID string, message string) error {
	channel := fmt.Sprintf(ChannelMailAgent, agentID)
	return m.client.Publish(ctx, channel, message).Err()
}

func (m *redisTeammateMesh) SubscribeTaskCreated(ctx context.Context) <-chan string {
	pubsub := m.client.Subscribe(ctx, ChannelTaskCreated)
	ch := make(chan string)

	go func() {
		defer pubsub.Close()
		defer close(ch)
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-pubsub.Channel():
				if !ok {
					return
				}
				select {
				case ch <- msg.Payload:
				case <-ctx.Done():
					return
				}
			}
		}
	}()
	return ch
}

func (m *redisTeammateMesh) SubscribeStatusUpdate(ctx context.Context) <-chan string {
	pubsub := m.client.Subscribe(ctx, ChannelStatusUpdate)
	ch := make(chan string)

	go func() {
		defer pubsub.Close()
		defer close(ch)
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-pubsub.Channel():
				if !ok {
					return
				}
				select {
				case ch <- msg.Payload:
				case <-ctx.Done():
					return
				}
			}
		}
	}()
	return ch
}

func (m *redisTeammateMesh) SubscribeMail(ctx context.Context, agentID string) <-chan string {
	channel := fmt.Sprintf(ChannelMailAgent, agentID)
	pubsub := m.client.Subscribe(ctx, channel)
	ch := make(chan string)

	go func() {
		defer pubsub.Close()
		defer close(ch)
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-pubsub.Channel():
				if !ok {
					return
				}
				select {
				case ch <- msg.Payload:
				case <-ctx.Done():
					return
				}
			}
		}
	}()
	return ch
}

func (m *redisTeammateMesh) AcquireLock(ctx context.Context, resourceName string, ttl time.Duration) (*redsync.Mutex, error) {
	mutexName := fmt.Sprintf("mesh:lock:%s", resourceName)
	mutex := m.rsync.NewMutex(mutexName, redsync.WithExpiry(ttl))

	if err := mutex.LockContext(ctx); err != nil {
		return nil, fmt.Errorf("failed to acquire lock for %s: %w", resourceName, err)
	}
	return mutex, nil
}
