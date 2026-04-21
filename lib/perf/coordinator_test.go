package perf

import (
	"context"
	"fmt"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestParallelUpdateMemory(t *testing.T) {
	db, err := orchestration.NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to init SQLite SIPDB: %v", err)
	}
	coord := NewCoordinator(db)

	updates := make(map[string]string)
	for i := 0; i < 10; i++ {
		updates[fmt.Sprintf("key-%d", i)] = fmt.Sprintf("val-%d", i)
	}

	err = coord.ParallelUpdateMemory(context.Background(), updates)
	if err != nil {
		t.Fatalf("ParallelUpdateMemory failed: %v", err)
	}
}
