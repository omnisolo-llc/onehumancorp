package interop

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"os"
	"sync"
	"time"

	"github.com/google/uuid"
	"github.com/redis/rueidis"
)

// HandoffProtocol defines the interface for state handoff between Cloud and Standalone modes.
type HandoffProtocol interface {
	ExportState(ctx context.Context, sessionID string, state *State) error
	ImportState(ctx context.Context, sessionID string) (*State, error)
}

// NewHandoffProtocol returns a new HandoffProtocol depending on the execution mode.
func NewHandoffProtocol() (HandoffProtocol, error) {
	redisURL := os.Getenv("REDIS_URL")
	if redisURL != "" && os.Getenv("OHC_STANDALONE") != "true" {
		opts, err := rueidis.ParseURL(redisURL)
		if err != nil {
			slog.Warn("failed to parse REDIS_URL, falling back to memory handoff", "error", err)
			return &MemoryHandoff{states: make(map[string]*State)}, nil
		}
		c, err := rueidis.NewClient(opts)
		if err != nil {
			slog.Warn("failed to connect to redis, falling back to memory handoff", "error", err)
			return &MemoryHandoff{states: make(map[string]*State)}, nil
		}
		slog.Info("HandoffProtocol initialized in Cloud mode (Redis)")
		return &CloudHandoff{client: c}, nil
	}

	slog.Info("HandoffProtocol initialized in Standalone mode (In-Memory)")
	return &MemoryHandoff{
		states: make(map[string]*State),
	}, nil
}

// MemoryHandoff provides an in-memory implementation for Standalone mode.
type MemoryHandoff struct {
	mu     sync.RWMutex
	states map[string]*State
}

func (m *MemoryHandoff) ExportState(ctx context.Context, sessionID string, state *State) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	select {
	case <-ctx.Done():
		return ctx.Err()
	default:
	}

	if state == nil {
		return fmt.Errorf("state cannot be nil")
	}

	// Deep copy data to ensure isolation
	copiedData := make(map[string]interface{})
	for k, v := range state.Data {
		copiedData[k] = v
	}

	copiedState := &State{
		ID:    state.ID,
		Data:  copiedData,
		Owner: state.Owner,
	}

	if copiedState.ID == "" {
		copiedState.ID = uuid.New().String()
	}

	m.states[sessionID] = copiedState
	return nil
}

func (m *MemoryHandoff) ImportState(ctx context.Context, sessionID string) (*State, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()

	select {
	case <-ctx.Done():
		return nil, ctx.Err()
	default:
	}

	state, ok := m.states[sessionID]
	if !ok {
		return nil, fmt.Errorf("state not found for session %s", sessionID)
	}

	// Deep copy data to ensure isolation
	copiedData := make(map[string]interface{})
	for k, v := range state.Data {
		copiedData[k] = v
	}

	return &State{
		ID:    state.ID,
		Data:  copiedData,
		Owner: state.Owner,
	}, nil
}

// CloudHandoff provides a Redis-backed implementation for Cloud mode.
type CloudHandoff struct {
	client rueidis.Client
}

func (c *CloudHandoff) ExportState(ctx context.Context, sessionID string, state *State) error {
	if state == nil {
		return fmt.Errorf("state cannot be nil")
	}

	stateBytes, err := json.Marshal(state)
	if err != nil {
		return fmt.Errorf("failed to marshal state: %w", err)
	}

	key := fmt.Sprintf("handoff:%s", sessionID)
	// SET key value EX 86400 (24 hours TTL)
	cmd := c.client.B().Set().Key(key).Value(string(stateBytes)).Ex(24 * time.Hour).Build()
	return c.client.Do(ctx, cmd).Error()
}

func (c *CloudHandoff) ImportState(ctx context.Context, sessionID string) (*State, error) {
	key := fmt.Sprintf("handoff:%s", sessionID)
	cmd := c.client.B().Get().Key(key).Build()

	resp := c.client.Do(ctx, cmd)
	if err := resp.Error(); err != nil {
		if rueidis.IsRedisNil(err) {
			return nil, fmt.Errorf("state not found for session %s", sessionID)
		}
		return nil, fmt.Errorf("failed to get state from redis: %w", err)
	}

	stateStr, err := resp.ToString()
	if err != nil {
		return nil, fmt.Errorf("failed to parse state from redis: %w", err)
	}

	var state State
	if err := json.Unmarshal([]byte(stateStr), &state); err != nil {
		return nil, fmt.Errorf("failed to unmarshal state: %w", err)
	}

	return &state, nil
}
