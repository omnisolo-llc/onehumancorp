package nats

import (
	"context"
	"testing"
	"time"

	"github.com/nats-io/nats-server/v2/server"
	"github.com/nats-io/nats-server/v2/test"

)

func RunTestServer() *server.Server {
	opts := &test.DefaultTestOptions
	opts.Port = -1
	return test.RunServer(opts)
}

func TestNatsMesh_PublishSubscribe(t *testing.T) {
	s := RunTestServer()
	defer s.Shutdown()

	mesh, err := NewNatsMesh(s.ClientURL())
	if err != nil {
		t.Fatalf("Expected no error connecting to test server, got %v", err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	subject := "test.channel"
	ch, err := mesh.Subscribe(ctx, subject)
	if err != nil {
		t.Fatalf("Expected no error subscribing, got %v", err)
	}

	msg := []byte("hello nats")
	err = mesh.Publish(ctx, subject, msg)
	if err != nil {
		t.Fatalf("Expected no error publishing, got %v", err)
	}

	select {
	case received := <-ch:
		if string(received) != string(msg) {
			t.Errorf("Expected %q, got %q", msg, received)
		}
	case <-ctx.Done():
		t.Fatal("Timed out waiting for message")
	}
}
