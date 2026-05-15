package orchestration

import (
	"context"
	"encoding/json"
	"fmt"

	"sync"
)

type MeshMessage struct {
	AgentID   string          `json:"agent_id"`
	Channel   string          `json:"channel"`
	EventType string          `json:"event_type"`
	Data      json.RawMessage `json:"data"`
}

type TeammateMesh interface {
	Publish(ctx context.Context, msg MeshMessage) error
	Subscribe(ctx context.Context, channel string) (<-chan MeshMessage, error)
}

// LegacyTeammateMesh is a mock implementation of the legacy mesh.
type LegacyTeammateMesh struct {
	mu          sync.Mutex
	subscribers map[string][]chan MeshMessage
}

func NewLegacyTeammateMesh() *LegacyTeammateMesh {
	return &LegacyTeammateMesh{
		subscribers: make(map[string][]chan MeshMessage),
	}
}

func (m *LegacyTeammateMesh) Publish(ctx context.Context, msg MeshMessage) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	subs := m.subscribers[msg.Channel]
	for _, sub := range subs {
		select {
		case sub <- msg:
		default:
			// Dropped
		}
	}
	return nil
}

func (m *LegacyTeammateMesh) Subscribe(ctx context.Context, channel string) (<-chan MeshMessage, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	ch := make(chan MeshMessage, 100)
	m.subscribers[channel] = append(m.subscribers[channel], ch)

	// Add keepalive to remove subscription
	go func() {
		<-ctx.Done()
		m.mu.Lock()
		defer m.mu.Unlock()
		subs := m.subscribers[channel]
		for i, sub := range subs {
			if sub == ch {
				m.subscribers[channel] = append(subs[:i], subs[i+1:]...)
				close(ch)
				break
			}
		}
	}()

	return ch, nil
}

// CentrifugeNode simulates the connection to the Centrifuge event stream API.
type CentrifugeNode struct {
	mu          sync.Mutex
	subscribers map[string][]chan MeshMessage
}

func NewCentrifugeNode() *CentrifugeNode {
	return &CentrifugeNode{
		subscribers: make(map[string][]chan MeshMessage),
	}
}

func (c *CentrifugeNode) Publish(ctx context.Context, channel string, data []byte) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	var msg MeshMessage
	if err := json.Unmarshal(data, &msg); err != nil {
		return err
	}

	subs := c.subscribers[channel]
	for _, sub := range subs {
		select {
		case sub <- msg:
		default:
		}
	}
	return nil
}

func (c *CentrifugeNode) Subscribe(ctx context.Context, channel string) (<-chan MeshMessage, error) {
	c.mu.Lock()
	defer c.mu.Unlock()

	ch := make(chan MeshMessage, 100)
	c.subscribers[channel] = append(c.subscribers[channel], ch)

	// Keep alive go routine that closes the channel when context is done
	go func() {
		<-ctx.Done()
		c.mu.Lock()
		defer c.mu.Unlock()
		subs := c.subscribers[channel]
		for i, sub := range subs {
			if sub == ch {
				c.subscribers[channel] = append(subs[:i], subs[i+1:]...)
				close(ch)
				break
			}
		}
	}()

	return ch, nil
}

type V2TeammateMesh struct {
	centrifuge *CentrifugeNode
}

func NewV2TeammateMesh(c *CentrifugeNode) *V2TeammateMesh {
	return &V2TeammateMesh{
		centrifuge: c,
	}
}

func (m *V2TeammateMesh) Publish(ctx context.Context, msg MeshMessage) error {
	data, err := json.Marshal(msg)
	if err != nil {
		return fmt.Errorf("failed to marshal mesh message: %w", err)
	}
	return m.centrifuge.Publish(ctx, msg.Channel, data)
}

func (m *V2TeammateMesh) Subscribe(ctx context.Context, channel string) (<-chan MeshMessage, error) {
	return m.centrifuge.Subscribe(ctx, channel)
}
