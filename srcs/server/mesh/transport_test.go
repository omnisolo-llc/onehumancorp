package mesh

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/rueidis"
)

func TestMemoryTransport(t *testing.T) {
	t.Run("Publish and Subscribe", func(t *testing.T) {
		ctx, cancel := context.WithCancel(context.Background())
		defer cancel()

		transport := NewMemoryTransport()
		channel := "test_channel"
		payload := []byte("hello mesh")

		sub, err := transport.Subscribe(ctx, channel)
		if err != nil {
			t.Fatalf("Subscribe failed: %v", err)
		}

		err = transport.Publish(ctx, channel, payload)
		if err != nil {
			t.Fatalf("Publish failed: %v", err)
		}

		select {
		case msg := <-sub:
			if string(msg) != string(payload) {
				t.Errorf("Expected %s, got %s", payload, msg)
			}
		case <-time.After(1 * time.Second):
			t.Fatal("Timeout waiting for message")
		}
	})

	t.Run("Publish with Context Cancelled", func(t *testing.T) {
		ctx, cancel := context.WithCancel(context.Background())
		transport := NewMemoryTransport()
		channel := "test_channel"

		cancel() // Cancel context

		// Create a mock subscriber that would block
		transport.mu.Lock()
		ch := make(chan []byte)
		transport.subscribers[channel] = append(transport.subscribers[channel], ch)
		transport.mu.Unlock()

		err := transport.Publish(ctx, channel, []byte("msg"))
		if err == nil || err != context.Canceled {
			t.Errorf("Expected context.Canceled, got %v", err)
		}
	})

	t.Run("Subscribe with Context Cancelled", func(t *testing.T) {
		ctx, cancel := context.WithCancel(context.Background())
		transport := NewMemoryTransport()
		channel := "test_channel"

		sub, _ := transport.Subscribe(ctx, channel)

		cancel()

		// Wait for the goroutine to clean up and close the channel
		select {
		case _, ok := <-sub:
			if ok {
				t.Fatal("Expected channel to be closed")
			}
		case <-time.After(1 * time.Second):
			t.Fatal("Timeout waiting for channel closure")
		}
	})

	t.Run("Publish to channel with no subscribers", func(t *testing.T) {
		ctx := context.Background()
		transport := NewMemoryTransport()

		err := transport.Publish(ctx, "empty_channel", []byte("msg"))
		if err != nil {
			t.Errorf("Expected nil error when publishing to empty channel, got %v", err)
		}
	})
}

func TestRedisTransport(t *testing.T) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer s.Close()

	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress: []string{s.Addr()},
		DisableCache: true,
	})
	if err != nil {
		t.Fatalf("failed to connect to miniredis: %v", err)
	}
	defer client.Close()

	t.Run("Publish and Subscribe", func(t *testing.T) {
		transport := NewRedisTransport(client)

		ctx, cancel := context.WithCancel(context.Background())
		defer cancel()

		channel := "test_channel"
		payload := []byte("hello redis")

		sub, err := transport.Subscribe(ctx, channel)
		if err != nil {
			t.Fatalf("Subscribe failed: %v", err)
		}

		// Wait briefly for subscription to activate
		time.Sleep(100 * time.Millisecond)

		// Must use a separate client to publish because miniredis correctly simulates that
		// a client in SUBSCRIBE mode cannot execute PUBLISH.
		pubClient, err := rueidis.NewClient(rueidis.ClientOption{
			InitAddress: []string{s.Addr()},
			DisableCache: true,
		})
		if err != nil {
			t.Fatalf("failed to connect to miniredis for publish: %v", err)
		}
		defer pubClient.Close()
		pubTransport := NewRedisTransport(pubClient)

		err = pubTransport.Publish(ctx, channel, payload)
		if err != nil {
			t.Fatalf("Publish failed: %v", err)
		}

		select {
		case msg := <-sub:
			if string(msg) != string(payload) {
				t.Errorf("Expected %s, got %s", payload, msg)
			}
		case <-time.After(1 * time.Second):
			t.Fatal("Timeout waiting for message")
		}
	})

	t.Run("Subscribe with Context Cancelled", func(t *testing.T) {
		transport := NewRedisTransport(client)

		ctx, cancel := context.WithCancel(context.Background())
		channel := "test_channel_cancel"

		sub, err := transport.Subscribe(ctx, channel)
		if err != nil {
			t.Fatalf("Subscribe failed: %v", err)
		}

		cancel()

		// Wait for the goroutine to clean up and close the channel
		select {
		case _, ok := <-sub:
			if ok {
				t.Fatal("Expected channel to be closed")
			}
		case <-time.After(1 * time.Second):
			t.Fatal("Timeout waiting for channel closure")
		}
	})
}
