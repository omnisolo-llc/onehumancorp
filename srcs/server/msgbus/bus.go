// Package msgbus provides a dual-backend message bus for the OHC agent system.
//
// Two backends are supported:
//   - NATS (nats.go): used for standalone desktop deployments.  An embedded
//     NATS server is started in-process so no external daemon is required.
//   - Redis/Valkey pub/sub: used for large-scale cluster deployments where
//     a Redis-compatible store is already available.
//
// Both backends implement the same Bus interface, so callers are fully
// decoupled from the underlying transport.
package msgbus

import (
	"context"
	"time"
)

// Backend selects the underlying message-bus implementation.
type Backend string

const (
	// BackendNATS uses an in-process NATS server.  Best for standalone desktop.
	BackendNATS Backend = "nats"
	// BackendRedis uses a Redis/Valkey server.  Best for cluster deployments.
	BackendRedis Backend = "redis"
	// BackendMemory uses an in-process channel bus.  Default; no external deps.
	BackendMemory Backend = "memory"
)

// Config holds the configuration for creating a Bus.
type Config struct {
	// Backend selects the transport.  Defaults to BackendMemory.
	Backend Backend

	// NATS settings (BackendNATS only).
	// NATSURLs is the list of NATS server URLs.
	// When empty and Backend==BackendNATS, an embedded server is started.
	NATSURLs []string

	// Redis settings (BackendRedis only).
	// RedisAddr is the Redis/Valkey server address (e.g. "localhost:6379").
	RedisAddr     string
	RedisPassword string
	RedisDB       int

	// PublishTimeout is the maximum time to wait for a message to be
	// acknowledged by the broker.  Defaults to 5 seconds.
	PublishTimeout time.Duration
}

// Message is a single message on a topic.
type Message struct {
	Topic   string
	Payload []byte
}

// Handler is called for each message received on a subscribed topic.
type Handler func(msg Message)

// Bus is the transport-agnostic message-bus interface.
//
// Implementations must be safe to call from multiple goroutines.
type Bus interface {
	// Publish sends a message to a topic.
	Publish(ctx context.Context, msg Message) error

	// Subscribe registers a handler for all messages on topic.
	// The returned cancel function unsubscribes the handler.
	Subscribe(topic string, handler Handler) (cancel func(), err error)

	// Close shuts down the bus and releases all resources.
	Close() error
}

// New creates a Bus using the provided Config.
// It returns a MemoryBus if no backend is specified (BackendMemory or zero value).
func New(cfg Config) (Bus, error) {
	if cfg.PublishTimeout == 0 {
		cfg.PublishTimeout = 5 * time.Second
	}
	switch cfg.Backend {
	case BackendNATS:
		return newNATSBus(cfg)
	case BackendRedis:
		return newRedisBus(cfg)
	default:
		return newMemoryBus(), nil
	}
}
