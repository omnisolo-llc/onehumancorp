package harness

import (
	"bytes"
	"context"
	"os"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func TestNewUniversalBridge_Cloud_EmptyRedisURL(t *testing.T) {
	os.Setenv("OHC_EXECUTION_MODE", "cloud")
	os.Setenv("REDIS_URL", "")

	var inBuf, outBuf bytes.Buffer
	bridge := NewUniversalBridge(&inBuf, &outBuf, "test_chan")
	if bridge == nil {
		t.Errorf("Expected bridge, got nil")
	}
}

func TestNewUniversalBridge_Local_NilStdio(t *testing.T) {
	os.Setenv("OHC_EXECUTION_MODE", "local")
	bridge := NewUniversalBridge(nil, nil, "test_chan")
	if bridge != nil {
		t.Errorf("Expected nil bridge, got %T", bridge)
	}
}

func TestCloudTransport_ReceiveError(t *testing.T) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	client := redis.NewClient(&redis.Options{Addr: s.Addr()})
	transport := NewCloudTransport(client, "test_channel")

	// Close miniredis to force error
	s.Close()

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	_, err = transport.Receive(ctx)
	if err == nil {
		t.Errorf("Expected error on receive, got nil")
	}
}

func TestRequestAgentTokens(t *testing.T) {
	ctx := context.Background()
	allowed, err := RequestAgentTokens(ctx, "test-tenant", 1)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
	// We expect true because it's a soft-fail or local mode empty string init
	if !allowed {
		t.Errorf("Expected allowed to be true")
	}
}
