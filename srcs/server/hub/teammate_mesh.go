package hub

import (
	"context"
	"fmt"
	"sync"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

type HubMessage struct {
	ID        string    `json:"id"`
	Payload   string    `json:"payload"`
	Timestamp time.Time `json:"timestamp"`
}

type TeammateMeshService interface {
	PublishMessage(ctx context.Context, topic string, msg HubMessage) error
	Subscribe(ctx context.Context, topic string) (<-chan HubMessage, error)
}

// This is a stub implementation representing the WebSockets/gRPC/Redis Pub/Sub
// communication layer requested in the mission brief.
type defaultTeammateMeshService struct {
	mu          sync.Mutex
	subscribers map[string][]chan HubMessage
}

func NewTeammateMeshService() TeammateMeshService {
	return &defaultTeammateMeshService{
		subscribers: make(map[string][]chan HubMessage),
	}
}

func (s *defaultTeammateMeshService) PublishMessage(ctx context.Context, topic string, msg HubMessage) error {
	s.mu.Lock()
	defer s.mu.Unlock()
	subs, ok := s.subscribers[topic]
	if !ok {
		return nil
	}
	for _, sub := range subs {
		select {
		case sub <- msg:
		default:
			// Dropped message to slow consumer
			fmt.Printf("Warning: message dropped for topic %s\n", topic)
		}
	}
	return nil
}

func (s *defaultTeammateMeshService) Subscribe(ctx context.Context, topic string) (<-chan HubMessage, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	ch := make(chan HubMessage, 100)
	s.subscribers[topic] = append(s.subscribers[topic], ch)

	// Clean up when context is done
	go func() {
		<-ctx.Done()
		s.mu.Lock()
		defer s.mu.Unlock()
		subs := s.subscribers[topic]
		for i, sub := range subs {
			if sub == ch {
				s.subscribers[topic] = append(subs[:i], subs[i+1:]...)
				close(ch)
				break
			}
		}
	}()

	return ch, nil
}

var (
	MessagesPublishedTotal metric.Int64Counter
)

func init() {
	meter := otel.Meter("srcs/server/hub/teammate_mesh")
	var err error
	MessagesPublishedTotal, err = meter.Int64Counter("teammate_mesh_messages_published", metric.WithDescription("Total number of messages published"))
	if err != nil {
		panic(err)
	}
}
