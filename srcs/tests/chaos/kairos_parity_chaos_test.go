package chaos_test

import (
	"context"
	"fmt"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration/kairos"
)

// TestKairosMesh_ConcurrencyChaos verifies KAIROS TeammateMesh under high-concurrency stress
// replicating the parity between MemoryMesh and eventual RedisMesh implementations.
func TestKairosMesh_ConcurrencyChaos(t *testing.T) {
	mesh := kairos.NewMemoryMesh()
	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	channelName := "kairos-chaos-test"
	sub, err := mesh.Subscribe(channelName)
	if err != nil {
		t.Fatalf("Failed to subscribe to mesh: %v", err)
	}

	var wg sync.WaitGroup
	publishCount := 100
	for i := 0; i < publishCount; i++ {
		wg.Add(1)
		go func(idx int) {
			defer wg.Done()
			msg := fmt.Sprintf("chaos-message-%d", idx)
			err := mesh.Publish(channelName, []byte(msg))
			if err != nil {
				t.Errorf("Failed to publish message: %v", err)
			}
		}(i)
	}

	wg.Wait()

	receivedCount := 0
	timeout := time.After(2 * time.Second)
loop:
	for {
		select {
		case <-sub:
			receivedCount++
			if receivedCount == publishCount {
				break loop
			}
		case <-timeout:
			break loop
		case <-ctx.Done():
			break loop
		}
	}

	if receivedCount != publishCount {
		t.Fatalf("Expected to receive %d messages, got %d", publishCount, receivedCount)
	}
}
