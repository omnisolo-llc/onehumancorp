package services

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	"github.com/redis/rueidis"
)

type MeshMessage struct {
	ID        string    `json:"id"`
	Sender    string    `json:"sender"`
	Recipient *string   `json:"recipient,omitempty"`
	Channel   string    `json:"channel"`
	Content   string    `json:"content"`
	CreatedAt time.Time `json:"created_at"`
}

type MeshCoordinatorService struct {
	isRedis     bool
	redisClient rueidis.Client

	// Local channel fallback state
	mu          sync.RWMutex
	subscribers map[string][]chan MeshMessage
}

func NewMeshCoordinatorService(redisClient interface{}) *MeshCoordinatorService {
	if redisClient != nil {
		if client, ok := redisClient.(rueidis.Client); ok {
			return &MeshCoordinatorService{
				isRedis:     true,
				redisClient: client,
			}
		}
	}

	return &MeshCoordinatorService{
		isRedis:     false,
		subscribers: make(map[string][]chan MeshMessage),
	}
}

func (s *MeshCoordinatorService) Publish(ctx context.Context, msg MeshMessage) error {
	if s.isRedis {
		b, err := json.Marshal(msg)
		if err != nil {
			return fmt.Errorf("failed to marshal mesh message: %w", err)
		}

		cmd := s.redisClient.B().Publish().Channel(msg.Channel).Message(string(b)).Build()
		res := s.redisClient.Do(ctx, cmd)
		if err := res.Error(); err != nil {
			return fmt.Errorf("redis publish failed: %w", err)
		}
		return nil
	}

	s.mu.RLock()
	defer s.mu.RUnlock()

	for _, ch := range s.subscribers[msg.Channel] {
		select {
		case ch <- msg:
		case <-time.After(10 * time.Millisecond):
			// Drop if blocked
		}
	}

	return nil
}

func (s *MeshCoordinatorService) Subscribe(ctx context.Context, channel string) (<-chan MeshMessage, error) {
	ch := make(chan MeshMessage, 100)

	if s.isRedis {
		cmd := s.redisClient.B().Subscribe().Channel(channel).Build()

		go func() {
			err := s.redisClient.Receive(ctx, cmd, func(msg rueidis.PubSubMessage) {
				var meshMsg MeshMessage
				if err := json.Unmarshal([]byte(msg.Message), &meshMsg); err == nil {
					select {
					case ch <- meshMsg:
					case <-ctx.Done():
					}
				}
			})
			if err != nil {
				// We don't have a great way to return this error from the background routine,
				// but in a production system we'd log it.
			}
			close(ch)
		}()

		return ch, nil
	}

	s.mu.Lock()
	s.subscribers[channel] = append(s.subscribers[channel], ch)
	s.mu.Unlock()

	go func() {
		<-ctx.Done()
		s.mu.Lock()
		defer s.mu.Unlock()

		subs := s.subscribers[channel]
		for i, c := range subs {
			if c == ch {
				s.subscribers[channel] = append(subs[:i], subs[i+1:]...)
				break
			}
		}
		close(ch)
	}()

	return ch, nil
}

func (m *MeshMessage) MarshalJSON() ([]byte, error) {
	type Alias MeshMessage
	return json.Marshal(&struct {
		*Alias
	}{
		Alias: (*Alias)(m),
	})
}
