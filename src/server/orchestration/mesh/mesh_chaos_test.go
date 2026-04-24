package mesh

import (
	"context"
	"fmt"
	"sync"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

// Chaos: test concurrent .agent-lock/ race conditions
func testLockChaos(t *testing.T, mesh TeammateMesh, testName string) {
	ctx := context.Background()
	lockKey := ".agent-lock/race-test"
	ttl := 2 * time.Second

	var wg sync.WaitGroup
	var mu sync.Mutex
	acquireCount := 0

	// 100 concurrent attempts to acquire the lock
	numAgents := 100
	wg.Add(numAgents)

	for i := 0; i < numAgents; i++ {
		go func() {
			defer wg.Done()
			acquired, err := mesh.AcquireLock(ctx, lockKey, ttl)
			if err != nil {
				// Errors might happen under heavy load, but we shouldn't get more than 1 acquire
				return
			}
			if acquired {
				mu.Lock()
				acquireCount++
				mu.Unlock()
			}
		}()
	}

	wg.Wait()

	if acquireCount != 1 {
		t.Errorf("[%s] Lock chaos test failed: expected exactly 1 successful acquire, got %d", testName, acquireCount)
	}

	// Verify lock is released correctly
	err := mesh.ReleaseLock(ctx, lockKey)
	if err != nil {
		t.Fatalf("[%s] Failed to release lock: %v", testName, err)
	}

	// Wait for any network delays or lock resets
	time.Sleep(100 * time.Millisecond)

	acquired, err := mesh.AcquireLock(ctx, lockKey, ttl)
	if err != nil {
		t.Fatalf("[%s] Failed to acquire lock after release: %v", testName, err)
	}
	if !acquired {
		t.Errorf("[%s] Expected to acquire lock after release", testName)
	}
}

// Chaos: test Pub/Sub message delivery under load
func testPubSubChaos(t *testing.T, mesh TeammateMesh, testName string) {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	topic := "chaos-topic"
	numMessages := 1000

	var wg sync.WaitGroup
	wg.Add(numMessages)

	var mu sync.Mutex
	receivedCount := 0

	sub, err := mesh.Subscribe(ctx, topic, func(msg []byte) {
		mu.Lock()
		receivedCount++
		mu.Unlock()
		wg.Done()
	})
	if err != nil {
		t.Fatalf("[%s] Failed to subscribe: %v", testName, err)
	}
	defer sub.Close()

	// Wait a bit for subscription to propagate (especially important for Redis)
	time.Sleep(200 * time.Millisecond)

	for i := 0; i < numMessages; i++ {
		payload := []byte(fmt.Sprintf("msg-%d", i))
		err := mesh.Publish(ctx, topic, payload)
		if err != nil {
			t.Fatalf("[%s] Failed to publish message: %v", testName, err)
		}
	}

	// Wait for messages, with a timeout
	done := make(chan struct{})
	go func() {
		wg.Wait()
		close(done)
	}()

	select {
	case <-done:
		// All messages received
	case <-time.After(5 * time.Second):
		t.Errorf("[%s] PubSub chaos test timeout: expected %d messages, got %d", testName, numMessages, receivedCount)
	}
}

// Chaos: test presence registration and reading
func testPresenceRegistration(t *testing.T, mesh TeammateMesh, testName string) {
	ctx := context.Background()

	err := mesh.RegisterPresence(ctx, "chaos-agent-1", "ACTIVE")
	if err != nil {
		t.Fatalf("[%s] Failed to register presence: %v", testName, err)
	}

	agents, err := mesh.GetActiveAgents(ctx)
	if err != nil {
		t.Fatalf("[%s] Failed to get active agents: %v", testName, err)
	}

	found := false
	for _, a := range agents {
		if a.AgentID == "chaos-agent-1" {
			found = true
			break
		}
	}

	if !found {
		t.Errorf("[%s] Expected to find chaos-agent-1 in active agents", testName)
	}
}

func TestMeshParityChaos(t *testing.T) {
	// Standalone / Local Mesh
	localMesh := NewLocalMesh()

	// Cloud / Redis Mesh
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer s.Close()

	client := redis.NewClient(&redis.Options{
		Addr: s.Addr(),
	})
	redisMesh := NewRedisMesh(client)

	// Run Parity Tests
	testLockChaos(t, localMesh, "LocalMesh")
	testLockChaos(t, redisMesh, "RedisMesh")

	testPubSubChaos(t, localMesh, "LocalMesh")
	testPubSubChaos(t, redisMesh, "RedisMesh")

	testPresenceRegistration(t, localMesh, "LocalMesh")
	testPresenceRegistration(t, redisMesh, "RedisMesh")
}
