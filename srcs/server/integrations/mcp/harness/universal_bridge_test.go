package harness

import (
	"bytes"
	"context"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/redis/go-redis/v9"
	"github.com/alicebob/miniredis/v2"
)

func TestNewUniversalBridge_Local(t *testing.T) {
	os.Setenv("OHC_EXECUTION_MODE", "local")
	var inBuf, outBuf bytes.Buffer
	bridge := NewUniversalBridge(&inBuf, &outBuf, "test_chan")
	if _, ok := bridge.(*LocalTransport); !ok {
		t.Errorf("Expected LocalTransport, got %T", bridge)
	}
}

func TestNewUniversalBridge_Cloud(t *testing.T) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer s.Close()

	os.Setenv("OHC_EXECUTION_MODE", "cloud")
	os.Setenv("REDIS_URL", "redis://"+s.Addr())

	var inBuf, outBuf bytes.Buffer
	bridge := NewUniversalBridge(&inBuf, &outBuf, "test_chan")
	if _, ok := bridge.(*CloudTransport); !ok {
		t.Errorf("Expected CloudTransport, got %T", bridge)
	}
}

func TestNewUniversalBridge_Cloud_BadURL(t *testing.T) {
	os.Setenv("OHC_EXECUTION_MODE", "cloud")
	os.Setenv("REDIS_URL", "invalid_url")

	var inBuf, outBuf bytes.Buffer
	bridge := NewUniversalBridge(&inBuf, &outBuf, "test_chan")
	if _, ok := bridge.(*LocalTransport); !ok {
		t.Errorf("Expected LocalTransport fallback for bad url, got %T", bridge)
	}

	// Test nil panic fallback
	bridgeNil := NewUniversalBridge(nil, nil, "test_chan")
	if bridgeNil != nil {
		t.Errorf("Expected nil bridge when fallback fails due to nil stdio")
	}
}

func TestLocalTransport_SendReceive(t *testing.T) {
	var inBuf, outBuf bytes.Buffer
	inBuf.WriteString("mock local response\n")

	transport := NewLocalTransport(&inBuf, &outBuf)
	ctx := context.Background()

	err := transport.Send(ctx, []byte("test"))
	if err != nil {
		t.Errorf("Expected no error on send, got %v", err)
	}
	if outBuf.String() != "test\n" {
		t.Errorf("Unexpected output: %s", outBuf.String())
	}

	res, err := transport.Receive(ctx)
	if err != nil {
		t.Errorf("Expected no error on receive, got %v", err)
	}
	if string(res) != "mock local response\n" {
		t.Errorf("Unexpected response: %s", string(res))
	}

	_, err = transport.Receive(ctx)
	if err == nil {
		t.Errorf("Expected error on receive when empty")
	}

	err = transport.Close()
	if err != nil {
		t.Errorf("Expected no error on close")
	}
}

func TestCloudTransport_SendReceive(t *testing.T) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer s.Close()

	client := redis.NewClient(&redis.Options{
		Addr: s.Addr(),
	})

	transport := NewCloudTransport(client, "test_channel")
	ctx := context.Background()

	time.Sleep(50 * time.Millisecond)

	go func() {
		time.Sleep(50 * time.Millisecond)
		// Emulate a response sent back via the res channel
		client.Publish(ctx, "test_channel_res", []byte(`{"result":"success"}`))
	}()

	// Send request via req channel
	err = transport.Send(ctx, []byte(`{"method":"test"}`))
	if err != nil {
		t.Errorf("Expected no error on send: %v", err)
	}

	// Receive response via res channel
	res, err := transport.Receive(ctx)
	if err != nil {
		t.Errorf("Expected no error on receive: %v", err)
	}
	if string(res) != `{"result":"success"}` {
		t.Errorf("Unexpected response: %s", string(res))
	}

	err = transport.Close()
	if err != nil {
		t.Errorf("Expected no error on close")
	}
}

func TestTempDirUsage(t *testing.T) {
	dir := t.TempDir()
	path := filepath.Join(dir, "dummy.txt")
	os.WriteFile(path, []byte("dummy"), 0644)

	file, err := os.OpenFile(path, os.O_RDWR, 0644)
	if err != nil {
		t.Fatalf("Failed to open temp file: %v", err)
	}
	defer file.Close()

	transport := NewLocalTransport(file, file)
	ctx := context.Background()

	err = transport.Send(ctx, []byte("temp file test"))
	if err != nil {
		t.Errorf("Send to temp file failed: %v", err)
	}

	file.Seek(0, 0)

	res, err := transport.Receive(ctx)
	if err != nil {
		t.Errorf("Receive from temp file failed: %v", err)
	}
	if string(res) != "temp file test\n" {
		t.Errorf("Unexpected read from temp file: %s", string(res))
	}
}
