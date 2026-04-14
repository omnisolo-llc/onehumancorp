package mesh

import (
	"context"
	"errors"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/redis/rueidis"
	"go.opentelemetry.io/otel"
)

var (
	meter             = otel.Meter("github.com/onehumancorp/mono/srcs/server/api/mesh")
	broadcastCount, _ = meter.Int64Counter("mesh.broadcast.count")
	subscribeCount, _ = meter.Int64Counter("mesh.subscribe.count")
)

type TeammateMeshService interface {
	BroadcastIntent(ctx context.Context, intent string) error
	Subscribe(ctx context.Context) (<-chan string, error)
}

type RedisMeshService struct {
	client  rueidis.Client
	channel string
}

func NewRedisMeshService(client rueidis.Client, channel string) *RedisMeshService {
	return &RedisMeshService{
		client:  client,
		channel: channel,
	}
}

func (s *RedisMeshService) BroadcastIntent(ctx context.Context, intent string) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return errors.New("unauthorized: missing claims")
	}

	broadcastCount.Add(ctx, 1)

	cmd := s.client.B().Publish().Channel(s.channel).Message(intent).Build()
	return s.client.Do(ctx, cmd).Error()
}

func (s *RedisMeshService) Subscribe(ctx context.Context) (<-chan string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	subscribeCount.Add(ctx, 1)

	out := make(chan string, 100)

	go func() {
		defer close(out)

		err := s.client.Receive(ctx, s.client.B().Subscribe().Channel(s.channel).Build(), func(msg rueidis.PubSubMessage) {
			select {
			case out <- msg.Message:
			case <-ctx.Done():
			}
		})
		if err != nil && err != context.Canceled {
			// In a real application, handle error logging
		}
	}()

	return out, nil
}

type MemoryMeshService struct {
	subscribers map[chan string]struct{}
	mu          sync.RWMutex
}

func NewMemoryMeshService() *MemoryMeshService {
	return &MemoryMeshService{
		subscribers: make(map[chan string]struct{}),
	}
}

func (s *MemoryMeshService) BroadcastIntent(ctx context.Context, intent string) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return errors.New("unauthorized: missing claims")
	}

	broadcastCount.Add(ctx, 1)

	s.mu.RLock()
	defer s.mu.RUnlock()

	for sub := range s.subscribers {
		select {
		case sub <- intent:
		case <-time.After(10 * time.Millisecond): // Drop if blocked
		}
	}
	return nil
}

func (s *MemoryMeshService) Subscribe(ctx context.Context) (<-chan string, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	subscribeCount.Add(ctx, 1)

	out := make(chan string, 100)

	s.mu.Lock()
	s.subscribers[out] = struct{}{}
	s.mu.Unlock()

	go func() {
		<-ctx.Done()
		s.mu.Lock()
		delete(s.subscribers, out)
		s.mu.Unlock()
		close(out)
	}()

	return out, nil
}
