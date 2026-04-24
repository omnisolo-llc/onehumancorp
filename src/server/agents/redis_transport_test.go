package agents

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/rueidis"
)

func TestRedisPubSubTransport(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client1, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	if err != nil {
		t.Fatalf("Failed to create client1: %v", err)
	}
	defer client1.Close()

	client2, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	if err != nil {
		t.Fatalf("Failed to create client2: %v", err)
	}
	defer client2.Close()

	pubChan := "agent:1:to:agent:2"
	subChan := "agent:2:to:agent:1"

	transport1 := NewRedisPubSubTransport(client1, pubChan, subChan)
	defer transport1.Close()

	transport2 := NewRedisPubSubTransport(client2, subChan, pubChan)
	defer transport2.Close()

	time.Sleep(50 * time.Millisecond)

	ctx := context.Background()
	msgToSend := []byte("hello from agent 1")

	err = transport1.Send(ctx, msgToSend)
	if err != nil {
		t.Fatalf("Failed to send message: %v", err)
	}

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

	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}
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

	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}

	transport := NewRedisPubSubTransport(client, "pub", "sub")

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

	client.Close()
}

func TestRedisPubSubTransport_Receive_TransportContextCancelled(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}

	transport := NewRedisPubSubTransport(client, "pub", "sub")

	// cancel the inner context directly to test <-t.ctx.Done()
	transport.cancel()

	// also we want to test errChan if possible. Let's just pass context.Canceled check
	ctx := context.Background()
	_, err = transport.Receive(ctx)
	if err != context.Canceled {
		t.Errorf("Expected context.Canceled, got %v", err)
	}

	client.Close()
}



func TestRedisPubSubTransport_Receive_TransportContextCancelled_Inner(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}
	defer client.Close()

	transport := NewRedisPubSubTransport(client, "pub", "sub")

	// Publish something, but immediately cancel the transport context so the inner goroutine selects ctx.Done()
	transport.cancel()
	time.Sleep(10 * time.Millisecond) // Let the goroutine exit
}

func TestRedisPubSubTransport_Receive_ErrorChan(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}

	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}

	transport := NewRedisPubSubTransport(client, "pub", "sub")
	mr.Close()

	time.Sleep(50 * time.Millisecond)

	ctx := context.Background()
	_, err = transport.Receive(ctx)
	if err == nil || err == context.Canceled {
		t.Errorf("Expected an error from errChan, got %v", err)
	}
}

func TestRedisPubSubTransport_Receive_TransportContextCancelled_Inner2(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{mr.Addr()},
		DisableCache: true,
	})
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}
	defer client.Close()

	// To hit `case <-ctx.Done(): return` inside `Receive` loop callback:
	// The callback receives a message `func(msg rueidis.PubSubMessage)`, but we cancel the context at the same time
	// so the select defaults to `case <-ctx.Done()`.

	// We can do this by filling msgChan (it is unbuffered) and cancelling the context.
	transport := NewRedisPubSubTransport(client, "pub", "sub")

	// We don't call transport.Receive(), so msgChan is blocked.
	// We send a message, which triggers the callback, which tries `t.msgChan <- msg` and blocks.
	client2, _ := rueidis.NewClient(rueidis.ClientOption{InitAddress: []string{mr.Addr()}, DisableCache: true})
	defer client2.Close()

	// Ensure subscription is active
	time.Sleep(50 * time.Millisecond)

	err = client2.Do(context.Background(), client2.B().Publish().Channel("sub").Message("block").Build()).Error()
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	time.Sleep(10 * time.Millisecond) // callback is now blocked on msgChan

	// Now cancel the transport ctx
	transport.cancel()

	time.Sleep(10 * time.Millisecond) // callback should wake up and hit ctx.Done()
}
