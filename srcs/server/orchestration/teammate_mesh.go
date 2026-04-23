package orchestration

import (
	"context"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

type TeammateMesh interface {
	Publish(ctx context.Context, channel string, message []byte) error
	Subscribe(ctx context.Context, channel string) (<-chan []byte, error)
	Unsubscribe(ctx context.Context, channel string) error
}

type LocalTeammateMesh struct {
	mu            sync.RWMutex
	subscriptions map[string]map[chan []byte]struct{}
}

func NewLocalTeammateMesh() *LocalTeammateMesh {
	return &LocalTeammateMesh{
		subscriptions: make(map[string]map[chan []byte]struct{}),
	}
}

func (l *LocalTeammateMesh) Publish(ctx context.Context, channel string, message []byte) error {
	l.mu.RLock()
	defer l.mu.RUnlock()

	subs, ok := l.subscriptions[channel]
	if !ok {
		return nil
	}

	for ch := range subs {
		select {
		case ch <- message:
		case <-ctx.Done():
			return ctx.Err()
		default:
			// Non-blocking
		}
	}
	return nil
}

func (l *LocalTeammateMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	l.mu.Lock()
	defer l.mu.Unlock()

	if _, ok := l.subscriptions[channel]; !ok {
		l.subscriptions[channel] = make(map[chan []byte]struct{})
	}

	ch := make(chan []byte, 100)
	l.subscriptions[channel][ch] = struct{}{}

	return ch, nil
}

func (l *LocalTeammateMesh) Unsubscribe(ctx context.Context, channel string) error {
	l.mu.Lock()
	defer l.mu.Unlock()

	subs, ok := l.subscriptions[channel]
	if ok {
		for ch := range subs {
			close(ch)
		}
		delete(l.subscriptions, channel)
	}
	return nil
}

type RedisTeammateMesh struct {
	client rueidis.Client
	mu     sync.Mutex
	subs   map[string]context.CancelFunc
}

func NewRedisTeammateMesh(client rueidis.Client) *RedisTeammateMesh {
	return &RedisTeammateMesh{
		client: client,
		subs:   make(map[string]context.CancelFunc),
	}
}

func (r *RedisTeammateMesh) Publish(ctx context.Context, channel string, message []byte) error {
	cmd := r.client.B().Publish().Channel(channel).Message(string(message)).Build()
	return r.client.Do(ctx, cmd).Error()
}

func (r *RedisTeammateMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	ch := make(chan []byte, 100)
	subCtx, cancel := context.WithCancel(ctx)

	r.mu.Lock()
	r.subs[channel] = cancel
	r.mu.Unlock()

	go func() {
		defer close(ch)
		err := r.client.Receive(subCtx, r.client.B().Subscribe().Channel(channel).Build(), func(msg rueidis.PubSubMessage) {
			select {
			case ch <- []byte(msg.Message):
			case <-subCtx.Done():
			}
		})
		_ = err
	}()

	return ch, nil
}

func (r *RedisTeammateMesh) Unsubscribe(ctx context.Context, channel string) error {
	r.mu.Lock()
	cancel, ok := r.subs[channel]
	if ok {
		cancel()
		delete(r.subs, channel)
	}
	r.mu.Unlock()
	return nil
}

func NewTeammateMesh(provider db.Provider, client rueidis.Client) TeammateMesh {
	if provider != nil && provider.IsSQLite() {
		return NewLocalTeammateMesh()
	}
	if client == nil {
		return NewLocalTeammateMesh()
	}
	return NewRedisTeammateMesh(client)
}
