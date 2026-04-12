package orchestration

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestBridgeManager(t *testing.T) {
	provider := (db.Provider)(nil)

	bm := NewBridgeManager(provider, nil)
	if bm == nil {
		t.Fatal("Expected BridgeManager to not be nil")
	}

	ctx := context.Background()

	err := bm.Connect(ctx)
	if err != nil {
		t.Fatalf("Expected nil error, got %v", err)
	}

	err = bm.Start(ctx)
	if err != nil {
	    t.Fatalf("Expected start to succeed, got %v", err)
	}

	err = bm.HandleInboundEvent(ctx, []byte(`{"id":"task-123"}`))
	if err != nil {
	    t.Fatalf("Expected inbound event handling to succeed, got %v", err)
	}

	bm.Stop()
}
