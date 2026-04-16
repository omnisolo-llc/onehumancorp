package chaos

import (
	"context"
	"fmt"
	"math/rand"
	"time"
	"os"
)

// ChaosInjector defines the interface for injecting chaos into the system
type ChaosInjector interface {
	// InjectNetworkLatency simulates network delay
	InjectNetworkLatency(ctx context.Context, latency time.Duration) error

	// DropDatabaseConnections simulates database connection drops
	DropDatabaseConnections(ctx context.Context, percent float64) error

	// SimulateResourceExhaustion simulates host machine resource exhaustion
	SimulateResourceExhaustion(ctx context.Context) error

	// CorruptStateFiles simulates corruption of internal orchestrator state files
	CorruptStateFiles(ctx context.Context, path string) error
}

// DefaultInjector provides a basic implementation of ChaosInjector
type DefaultInjector struct {}

// NewDefaultInjector creates a new DefaultInjector
func NewDefaultInjector() *DefaultInjector {
	return &DefaultInjector{}
}

// InjectNetworkLatency implements ChaosInjector
func (i *DefaultInjector) InjectNetworkLatency(ctx context.Context, latency time.Duration) error {
	// Simulate network latency by sleeping
	select {
	case <-time.After(latency):
		return nil
	case <-ctx.Done():
		return ctx.Err()
	}
}

// DropDatabaseConnections implements ChaosInjector
func (i *DefaultInjector) DropDatabaseConnections(ctx context.Context, percent float64) error {
	if percent < 0 || percent > 1.0 {
		return fmt.Errorf("percent must be between 0.0 and 1.0")
	}

	// Randomly determine if connection should be dropped based on percentage
	if rand.Float64() < percent {
		return fmt.Errorf("database connection dropped (chaos injection)")
	}

	return nil
}

// SimulateResourceExhaustion implements ChaosInjector
func (i *DefaultInjector) SimulateResourceExhaustion(ctx context.Context) error {
	// Simulate CPU exhaustion by running a tight loop for a short duration
	// To prevent actual system hangs, we limit this to 500ms
	exhaustionDuration := 500 * time.Millisecond

	// Create a done channel for the timeout
	done := make(chan struct{})

	go func() {
		// Tight loop
		for {
			select {
			case <-done:
				return
			default:
				// Do some math to consume CPU
				_ = rand.Float64() * rand.Float64()
			}
		}
	}()

	// Wait for the duration or context cancellation
	select {
	case <-time.After(exhaustionDuration):
		close(done)
		return nil
	case <-ctx.Done():
		close(done)
		return ctx.Err()
	}
}

// CorruptStateFiles simulates corruption of internal orchestrator state files
func (i *DefaultInjector) CorruptStateFiles(ctx context.Context, path string) error {
	// Check if file exists
	if _, err := os.Stat(path); os.IsNotExist(err) {
		return fmt.Errorf("file does not exist: %s", path)
	}

	// Read original file content
	content, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("failed to read file %s: %w", path, err)
	}

	// Corrupt file by appending random garbage
	corruptedData := append(content, []byte("\n<chaos>corrupted_by_chaos_injector</chaos>\n")...)

	// Write corrupted content back
	err = os.WriteFile(path, corruptedData, 0644)
	if err != nil {
		return fmt.Errorf("failed to corrupt file %s: %w", path, err)
	}

	return nil
}
