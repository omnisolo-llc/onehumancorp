package interop

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"sync"
	"time"

	"github.com/redis/rueidis"
)

// HealthMonitor defines the interface for monitoring cross-mode health and status.
type HealthMonitor interface {
	ReportHealth(ctx context.Context, agentID string, status string) error
	CheckHealth(ctx context.Context, agentID string) (string, error)
}

// NewHealthMonitor returns a new HealthMonitor depending on the execution mode.
func NewHealthMonitor() (HealthMonitor, error) {
	redisURL := os.Getenv("REDIS_URL")
	if redisURL != "" && os.Getenv("OHC_STANDALONE") != "true" {
		opts, err := rueidis.ParseURL(redisURL)
		if err != nil {
			slog.Warn("failed to parse REDIS_URL, falling back to memory health monitor", "error", err)
			return &MemoryHealthMonitor{statuses: make(map[string]healthEntry)}, nil
		}
		c, err := rueidis.NewClient(opts)
		if err != nil {
			slog.Warn("failed to connect to redis, falling back to memory health monitor", "error", err)
			return &MemoryHealthMonitor{statuses: make(map[string]healthEntry)}, nil
		}
		slog.Info("HealthMonitor initialized in Cloud mode (Redis)")
		return &CloudHealthMonitor{client: c}, nil
	}

	slog.Info("HealthMonitor initialized in Standalone mode (In-Memory)")
	return &MemoryHealthMonitor{
		statuses: make(map[string]healthEntry),
	}, nil
}

type healthEntry struct {
	status    string
	timestamp time.Time
}

// MemoryHealthMonitor provides an in-memory implementation for Standalone mode.
type MemoryHealthMonitor struct {
	mu       sync.RWMutex
	statuses map[string]healthEntry
}

func (m *MemoryHealthMonitor) ReportHealth(ctx context.Context, agentID string, status string) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	select {
	case <-ctx.Done():
		return ctx.Err()
	default:
	}

	m.statuses[agentID] = healthEntry{
		status:    status,
		timestamp: time.Now(),
	}
	return nil
}

func (m *MemoryHealthMonitor) CheckHealth(ctx context.Context, agentID string) (string, error) {
	m.mu.RLock()
	defer m.mu.RUnlock()

	select {
	case <-ctx.Done():
		return "", ctx.Err()
	default:
	}

	entry, ok := m.statuses[agentID]
	if !ok {
		return "", fmt.Errorf("health status not found for agent %s", agentID)
	}

	// Consider dead if no heartbeat in last 5 minutes
	if time.Since(entry.timestamp) > 5*time.Minute {
		return "DEAD", nil
	}

	return entry.status, nil
}

// CloudHealthMonitor provides a Redis-backed implementation for Cloud mode.
type CloudHealthMonitor struct {
	client rueidis.Client
}

func (c *CloudHealthMonitor) ReportHealth(ctx context.Context, agentID string, status string) error {
	key := fmt.Sprintf("health:%s", agentID)
	// SET key value EX 300 (5 minutes TTL for health probes)
	cmd := c.client.B().Set().Key(key).Value(status).Ex(5 * time.Minute).Build()
	return c.client.Do(ctx, cmd).Error()
}

func (c *CloudHealthMonitor) CheckHealth(ctx context.Context, agentID string) (string, error) {
	key := fmt.Sprintf("health:%s", agentID)
	cmd := c.client.B().Get().Key(key).Build()

	resp := c.client.Do(ctx, cmd)
	if err := resp.Error(); err != nil {
		if rueidis.IsRedisNil(err) {
			return "DEAD", nil // Key expired or not found means agent is dead
		}
		return "", fmt.Errorf("failed to get health status from redis: %w", err)
	}

	status, err := resp.ToString()
	if err != nil {
		return "", fmt.Errorf("failed to parse health status: %w", err)
	}

	return status, nil
}
