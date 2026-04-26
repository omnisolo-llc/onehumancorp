package kairos

import (
    "context"
    "testing"
    "time"
    "github.com/onehumancorp/mono/src/server/db"
)

func TestKairosSharedTaskRepo(t *testing.T) {
    ctx := context.Background()
    provider := db.NewTestProvider(t)

    // Create the table just like the other tests do, in case migrations drop it.
    _, err := provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            agent_id TEXT,
            status TEXT,
            payload TEXT,
            created_at DATETIME
        );
    `)
    if err != nil {
        t.Fatalf("failed to create table: %v", err)
    }

    repo := NewSharedTaskRepo(provider)
    task := &SharedTask{
        ID: "test-uuid",
        AgentID: "agent-1",
        Status: "PENDING",
        Payload: []byte(`{"hello":"world"}`),
        CreatedAt: time.Now().Truncate(time.Second).UTC(),
    }

    if err := repo.Insert(ctx, task); err != nil {
        t.Fatalf("failed to insert: %v", err)
    }

    fetched, err := repo.Get(ctx, task.ID)
    if err != nil {
        t.Fatalf("failed to get: %v", err)
    }

    if fetched.ID != task.ID || fetched.AgentID != task.AgentID || fetched.Status != task.Status {
        t.Errorf("mismatch: %+v != %+v", fetched, task)
    }
    if string(fetched.Payload) != string(task.Payload) {
        t.Errorf("payload mismatch: %s != %s", string(fetched.Payload), string(task.Payload))
    }
    if !fetched.CreatedAt.Equal(task.CreatedAt) {
        t.Errorf("created_at mismatch: %v != %v", fetched.CreatedAt, task.CreatedAt)
    }
}
