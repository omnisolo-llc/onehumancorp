package orchestration

import (
    "context"
    "testing"
)

func TestAutoDreamAdvanced_PruneStaleAgentSessions(t *testing.T) {
    provider := setupTestDB(t)
    advanced := NewAutoDreamAdvanced(provider, nil)

    _, err := provider.Exec(context.Background(), `CREATE TABLE IF NOT EXISTS agent_session_data (
        session_id TEXT PRIMARY KEY,
        agent_id TEXT NOT NULL,
        context_data TEXT NOT NULL,
        created_at TEXT DEFAULT CURRENT_TIMESTAMP,
        last_accessed TEXT DEFAULT CURRENT_TIMESTAMP
    )`)
    if err != nil {
        t.Fatalf("failed to create agent_session_data: %v", err)
    }

    _, err = provider.Exec(context.Background(), "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('sess-1', 'agent-1', 'data', datetime('now', '-40 days'))")
    if err != nil {
        t.Fatalf("failed to insert stale session: %v", err)
    }

    err = advanced.PruneStaleAgentSessions(context.Background())
    if err != nil {
        t.Fatalf("failed to prune: %v", err)
    }

    var count int
    err = provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
    if err != nil {
        t.Fatalf("failed to count: %v", err)
    }
    if count != 0 {
        t.Errorf("expected 0 sessions, got %d", count)
    }
}

func TestAutoDreamAdvanced_InjectTruth(t *testing.T) {
    provider := setupTestDB(t)
    advanced := NewAutoDreamAdvanced(provider, nil)

    err := advanced.InjectTruth(context.Background(), "org-1", "agent-1", "truth content")
    if err != nil {
        t.Fatalf("failed to inject truth: %v", err)
    }
}
