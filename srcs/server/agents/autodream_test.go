package agents

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestAutoDreamEngine(t *testing.T) {
	ctx := context.Background()
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer provider.Close()

	err = provider.RunMigrations(ctx)
	if err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	engine := NewAutoDreamEngine(provider.Provider, "dummy-key")
	orchestration.ResetGlobalCircuitBreakerForTest()
	// Let's not run start which does ticker, just run method directly
	// And we shouldn't actually call minimax with dummy key if we don't want to block, but the fallback/error will just continue
	engine.consolidateMemories(ctx)
}
