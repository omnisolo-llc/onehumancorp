package msgbus_test

import (
	"context"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/msgbus"
)

// helper: create a MemoryBus via New() with default config.
func newTestBus(t *testing.T) msgbus.Bus {
	t.Helper()
	b, err := msgbus.New(msgbus.Config{})
	if err != nil {
		t.Fatalf("New: %v", err)
	}
	t.Cleanup(func() { _ = b.Close() })
	return b
}

func TestMemoryBus_PublishSubscribe(t *testing.T) {
	b := newTestBus(t)

	received := make(chan msgbus.Message, 4)
	cancel, err := b.Subscribe("test.topic", func(msg msgbus.Message) {
		received <- msg
	})
	if err != nil {
		t.Fatalf("Subscribe: %v", err)
	}
	defer cancel()

	want := []byte("hello world")
	if err := b.Publish(context.Background(), msgbus.Message{Topic: "test.topic", Payload: want}); err != nil {
		t.Fatalf("Publish: %v", err)
	}

	select {
	case got := <-received:
		if string(got.Payload) != string(want) {
			t.Errorf("payload: got %q, want %q", got.Payload, want)
		}
		if got.Topic != "test.topic" {
			t.Errorf("topic: got %q", got.Topic)
		}
	case <-time.After(time.Second):
		t.Fatal("timed out waiting for message")
	}
}

func TestMemoryBus_Unsubscribe(t *testing.T) {
	b := newTestBus(t)

	var count atomic.Int32
	cancel, err := b.Subscribe("unsub.topic", func(_ msgbus.Message) {
		count.Add(1)
	})
	if err != nil {
		t.Fatalf("Subscribe: %v", err)
	}

	_ = b.Publish(context.Background(), msgbus.Message{Topic: "unsub.topic", Payload: []byte("before")})
	time.Sleep(10 * time.Millisecond)
	if count.Load() != 1 {
		t.Fatalf("expected 1 message before unsub, got %d", count.Load())
	}

	cancel()

	_ = b.Publish(context.Background(), msgbus.Message{Topic: "unsub.topic", Payload: []byte("after")})
	time.Sleep(10 * time.Millisecond)
	if count.Load() != 1 {
		t.Errorf("expected no message after unsub, count=%d", count.Load())
	}
}

func TestMemoryBus_MultipleSubscribers(t *testing.T) {
	b := newTestBus(t)

	const subscribers = 5
	channels := make([]chan struct{}, subscribers)
	cancels := make([]func(), subscribers)
	for i := range subscribers {
		ch := make(chan struct{}, 1)
		channels[i] = ch
		cancel, err := b.Subscribe("multi.topic", func(_ msgbus.Message) {
			ch <- struct{}{}
		})
		if err != nil {
			t.Fatalf("Subscribe[%d]: %v", i, err)
		}
		cancels[i] = cancel
	}
	defer func() {
		for _, c := range cancels {
			c()
		}
	}()

	_ = b.Publish(context.Background(), msgbus.Message{Topic: "multi.topic", Payload: []byte("broadcast")})

	for i, ch := range channels {
		select {
		case <-ch:
		case <-time.After(time.Second):
			t.Errorf("subscriber %d did not receive message", i)
		}
	}
}

func TestMemoryBus_IsolatesTopics(t *testing.T) {
	b := newTestBus(t)

	var received atomic.Int32
	cancel, _ := b.Subscribe("topic.A", func(_ msgbus.Message) {
		received.Add(1)
	})
	defer cancel()

	// Publish to a different topic.
	_ = b.Publish(context.Background(), msgbus.Message{Topic: "topic.B", Payload: []byte("nope")})
	time.Sleep(20 * time.Millisecond)

	if received.Load() != 0 {
		t.Errorf("should not receive messages for different topic; got %d", received.Load())
	}
}

func TestMemoryBus_NoSubscribers(t *testing.T) {
	b := newTestBus(t)
	// Should not panic or return error.
	if err := b.Publish(context.Background(), msgbus.Message{Topic: "ghost", Payload: []byte("nobody here")}); err != nil {
		t.Errorf("Publish to empty topic: %v", err)
	}
}

func TestMemoryBus_ConcurrentPublish(t *testing.T) {
	b := newTestBus(t)

	const msgs = 100
	var wg sync.WaitGroup
	var count atomic.Int32

	cancel, _ := b.Subscribe("concurrent", func(_ msgbus.Message) {
		count.Add(1)
	})
	defer cancel()

	for i := range msgs {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			_ = b.Publish(context.Background(), msgbus.Message{
				Topic:   "concurrent",
				Payload: []byte("msg"),
			})
		}(i)
	}

	wg.Wait()
	// Give handlers time to run.
	deadline := time.Now().Add(2 * time.Second)
	for time.Now().Before(deadline) {
		if count.Load() >= msgs {
			break
		}
		time.Sleep(5 * time.Millisecond)
	}

	if count.Load() != msgs {
		t.Errorf("expected %d messages, got %d", msgs, count.Load())
	}
}

func TestMemoryBus_Close(t *testing.T) {
	b, err := msgbus.New(msgbus.Config{})
	if err != nil {
		t.Fatal(err)
	}
	if err := b.Close(); err != nil {
		t.Errorf("Close: %v", err)
	}
}

func TestNew_DefaultIsMemory(t *testing.T) {
	b, err := msgbus.New(msgbus.Config{})
	if err != nil {
		t.Fatalf("New default: %v", err)
	}
	defer b.Close()

	if _, ok := b.(*msgbus.MemoryBus); !ok {
		t.Errorf("expected *MemoryBus, got %T", b)
	}
}

func TestNew_ExplicitMemory(t *testing.T) {
	b, err := msgbus.New(msgbus.Config{Backend: msgbus.BackendMemory})
	if err != nil {
		t.Fatalf("New memory: %v", err)
	}
	defer b.Close()

	if _, ok := b.(*msgbus.MemoryBus); !ok {
		t.Errorf("expected *MemoryBus, got %T", b)
	}
}

func TestMemoryBus_PayloadBytes(t *testing.T) {
	b := newTestBus(t)

	received := make(chan []byte, 1)
	cancel, _ := b.Subscribe("bytes.topic", func(m msgbus.Message) {
		received <- m.Payload
	})
	defer cancel()

	payload := []byte{0x01, 0x02, 0xFF, 0x00}
	_ = b.Publish(context.Background(), msgbus.Message{Topic: "bytes.topic", Payload: payload})

	select {
	case got := <-received:
		if len(got) != len(payload) {
			t.Errorf("payload len: got %d, want %d", len(got), len(payload))
		}
		for i := range payload {
			if got[i] != payload[i] {
				t.Errorf("payload[%d]: got %x, want %x", i, got[i], payload[i])
			}
		}
	case <-time.After(time.Second):
		t.Fatal("timed out")
	}
}

func TestMemoryBus_MultipleUnsubscribeSafe(t *testing.T) {
	b := newTestBus(t)
	cancel, _ := b.Subscribe("safe.unsub", func(_ msgbus.Message) {})
	// Should not panic on double-cancel.
	cancel()
	cancel()
}

// TestBusInterface verifies that all concrete types satisfy Bus.
func TestBusInterface(t *testing.T) {
	var _ msgbus.Bus = (*msgbus.MemoryBus)(nil)
	var _ msgbus.Bus = (*msgbus.NATSBus)(nil)
	var _ msgbus.Bus = (*msgbus.RedisBus)(nil)
}
