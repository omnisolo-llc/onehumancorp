package agents

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func TestRedisPubSubTransport(t *testing.T) {
	// Start a mock Redis server
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer mr.Close()

	// Create Redis client
	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	// Channels for communication
	pubChan := "agent:1:to:agent:2"
	subChan := "agent:2:to:agent:1"

	// Create transports for two agents communicating with each other
	transport1 := NewRedisPubSubTransport(client, pubChan, subChan)
	defer transport1.Close()

	transport2 := NewRedisPubSubTransport(client, subChan, pubChan)
	defer transport2.Close()

	// Allow some time for subscriptions to propagate
	time.Sleep(50 * time.Millisecond)

	ctx := context.Background()
	msgToSend := []byte("hello from agent 1")

	// Send message from agent 1
	err = transport1.Send(ctx, msgToSend)
	if err != nil {
		t.Fatalf("Failed to send message: %v", err)
	}

	// Receive message at agent 2
	recvCtx, cancel := context.WithTimeout(ctx, 2*time.Second)
	defer cancel()

	receivedMsg, err := transport2.Receive(recvCtx)
	if err != nil {
		t.Fatalf("Failed to receive message: %v", err)
	}

	if string(receivedMsg) != string(msgToSend) {
		t.Errorf("Expected %q, got %q", string(msgToSend), string(receivedMsg))
	}
}

func TestRedisPubSubTransport_Receive_ContextCancelled(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	defer client.Close()

	transport := NewRedisPubSubTransport(client, "pub", "sub")
	defer transport.Close()

	ctx, cancel := context.WithCancel(context.Background())
	cancel() // Cancel immediately

	_, err = transport.Receive(ctx)
	if err == nil {
		t.Error("Expected error from Receive with cancelled context, got nil")
	}
	if err != context.Canceled {
		t.Errorf("Expected context.Canceled, got %v", err)
	}
}

func TestRedisPubSubTransport_Receive_ChannelClosed(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	transport := NewRedisPubSubTransport(client, "pub", "sub")

	// Close the transport, which closes the subscription and channel
	err = transport.Close()
	if err != nil {
		t.Fatalf("Failed to close transport: %v", err)
	}

	ctx := context.Background()
	_, err = transport.Receive(ctx)
	if err == nil {
		t.Error("Expected error from Receive after channel close, got nil")
	}
	if err != context.Canceled {
		t.Errorf("Expected context.Canceled, got %v", err)
	}

	// Clean up client at the end to avoid "use of closed network connection"
	// panics in other goroutines during shutdown
	client.Close()
}
