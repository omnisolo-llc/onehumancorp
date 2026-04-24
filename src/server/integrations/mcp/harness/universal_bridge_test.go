package harness

import (
    "context"
    "io"
    "testing"
    "time"

    "github.com/alicebob/miniredis/v2"
    "github.com/redis/go-redis/v9"
)

func TestUniversalBridge_Local(t *testing.T) {
    pipeReader, pipeWriter := io.Pipe()

    bridge := NewUniversalBridge("LOCAL", pipeReader, pipeWriter, nil, "", "")

    ctx := context.Background()
    go func() {
        _ = bridge.Send(ctx, []byte("test-message"))
    }()

    msg, err := bridge.Receive(ctx)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if string(msg) != "test-message" {
        t.Fatalf("expected 'test-message', got %s", string(msg))
    }
    bridge.Close()
}

func TestUniversalBridge_Cloud(t *testing.T) {
    mr, err := miniredis.Run()
    if err != nil {
        t.Fatalf("failed to start miniredis: %v", err)
    }
    defer mr.Close()

    rdb := redis.NewClient(&redis.Options{
        Addr: mr.Addr(),
    })

    bridge1 := NewUniversalBridge("CLOUD", nil, nil, rdb, "test_channel", "test_channel")

    ctx := context.Background()

    go func() {
        time.Sleep(10 * time.Millisecond)
        _ = bridge1.Send(ctx, []byte("cloud-message"))
    }()

    msg, err := bridge1.Receive(ctx)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if string(msg) != "cloud-message" {
        t.Fatalf("expected 'cloud-message', got %s", string(msg))
    }

    bridge1.Close()
}
