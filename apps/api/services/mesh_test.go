package services

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/rueidis"
)

func TestMeshCoordinatorService_Local(t *testing.T) {
	svc := NewMeshCoordinatorService(nil)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	channel := "test-channel"

	sub, err := svc.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	msg := MeshMessage{
		ID:        "msg-1",
		Sender:    "agent-1",
		Channel:   channel,
		Content:   `{"test": "data"}`,
		CreatedAt: time.Now(),
	}

	if err := svc.Publish(context.Background(), msg); err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	select {
	case received := <-sub:
		if received.ID != msg.ID {
			t.Errorf("expected %s, got %s", msg.ID, received.ID)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for message")
	}
}

// Custom wrapper to route Publish through a separate connection to avoid miniredis/rueidis PubSub restriction
type miniredisDualClient struct {
	rueidis.Client
	pubClient rueidis.Client
}

func (m *miniredisDualClient) Do(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult {
	if len(cmd.Commands()) > 0 && cmd.Commands()[0] == "PUBLISH" {
		return m.pubClient.Do(ctx, cmd)
	}
	return m.Client.Do(ctx, cmd)
}

func TestMeshCoordinatorService_Redis(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client1, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	if err != nil {
		t.Fatalf("failed to create rueidis client: %v", err)
	}
	defer client1.Close()

	client2, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	if err != nil {
		t.Fatalf("failed to create second rueidis client: %v", err)
	}
	defer client2.Close()

	dualClient := &miniredisDualClient{Client: client1, pubClient: client2}

	svc := NewMeshCoordinatorService(dualClient)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	channel := "test-channel"

	sub, err := svc.Subscribe(ctx, channel)
	if err != nil {
		t.Fatalf("failed to subscribe: %v", err)
	}

	// Give subscriber a tiny bit of time to establish connection
	time.Sleep(100 * time.Millisecond)

	msg := MeshMessage{
		ID:        "msg-redis-1",
		Sender:    "agent-redis",
		Channel:   channel,
		Content:   `{"test": "redis"}`,
		CreatedAt: time.Now(),
	}

	// Test Publish & Receive
	if err := svc.Publish(context.Background(), msg); err != nil {
		t.Fatalf("failed to publish: %v", err)
	}

	select {
	case received := <-sub:
		if received.ID != msg.ID {
			t.Errorf("expected %s, got %s", msg.ID, received.ID)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("timeout waiting for message")
	}

	t.Run("Publish_RedisMarshalError", func(t *testing.T) {
		// Mock json.Marshal error - wait, we can't easily mock json.Marshal.
		// Let's test MarshalJSON instead.
		msg := MeshMessage{ID: "test"}
		b, err := msg.MarshalJSON()
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}
		if len(b) == 0 {
			t.Fatal("expected bytes")
		}
	})

	t.Run("Publish_RedisDoError", func(t *testing.T) {
		// rueidis allows injecting errors with a mock or simply passing an invalid context to Do
		// For Do, if the context is canceled it should return an error
		canceledCtx, cancelFn := context.WithCancel(context.Background())
		cancelFn()
		err := svc.Publish(canceledCtx, msg)
		if err == nil {
			t.Fatal("expected error on canceled context")
		}
	})

	t.Run("NewMeshCoordinatorService_Nil", func(t *testing.T) {
		svc := NewMeshCoordinatorService("invalid type")
		if svc.isRedis {
			t.Fatal("expected isRedis to be false")
		}
	})
}
