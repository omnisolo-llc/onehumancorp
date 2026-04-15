package orchestration

import (
	"context"
	"fmt"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func BenchmarkLocalTeammateMesh_Broadcast(b *testing.B) {
	b.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	provider := db.NewTestProvider(b)
	defer provider.Close()

	ctx := context.Background()
	_, err := provider.Exec(ctx, `
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority VARCHAR NOT NULL DEFAULT 'P2',
			payload JSON,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		b.Fatalf("failed to create schema: %v", err)
	}

	mesh := NewLocalTeammateMesh(provider)

	// Subscribe one consumer
	_, err = mesh.SubscribeTasks(ctx)
	if err != nil {
		b.Fatalf("failed to subscribe: %v", err)
	}

	b.ResetTimer()
	b.RunParallel(func(pb *testing.PB) {
		i := 0
		for pb.Next() {
			task := Task{
				AgentID: "spiffe://onehumancorp.io/agent/bench",
				Action:  "BENCHMARK",
				Status:  "PENDING",
				TaskID:  fmt.Sprintf("task-%d", i),
			}
			_ = mesh.BroadcastTask(ctx, task)
			i++
		}
	})
}
