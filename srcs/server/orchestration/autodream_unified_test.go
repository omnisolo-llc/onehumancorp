package orchestration

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
)

func setupUnifiedTestDB(t *testing.T) db.Provider {
	provider := db.NewTestProvider(t)

	// Create autodream_memories_master table
	_, err := provider.Exec(context.Background(), `CREATE TABLE IF NOT EXISTS autodream_memories_master (
		id VARCHAR PRIMARY KEY,
		organization_id VARCHAR NOT NULL,
		memory_type TEXT NOT NULL,
		content TEXT NOT NULL,
		embedding TEXT,
		source_task_id VARCHAR,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("failed to create autodream_memories_master: %v", err)
	}

	// Create agent_session_data table
	_, err = provider.Exec(context.Background(), `CREATE TABLE IF NOT EXISTS agent_session_data (
		session_id TEXT PRIMARY KEY,
		agent_id TEXT NOT NULL,
		context_data TEXT NOT NULL,
		created_at TEXT DEFAULT CURRENT_TIMESTAMP,
		last_accessed TEXT DEFAULT CURRENT_TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("failed to create agent_session_data: %v", err)
	}

	// Create shared_tasks_decomposition table
	_, err = provider.Exec(context.Background(), `CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		status TEXT NOT NULL,
		payload TEXT
	)`)
	if err != nil {
		t.Fatalf("failed to create shared_tasks_decomposition: %v", err)
	}

    // Create sub_agent_queue table for QueueManager
    _, err = provider.Exec(context.Background(), `CREATE TABLE IF NOT EXISTS sub_agent_queue (
        id TEXT PRIMARY KEY,
        organization_id TEXT NOT NULL,
        parent_task_id TEXT,
        payload TEXT,
        status TEXT,
        worker_id TEXT,
        created_at TEXT,
        updated_at TEXT
    )`)
	if err != nil {
		t.Fatalf("failed to create sub_agent_queue: %v", err)
	}

	return provider
}

func TestAutoDreamWorker_Unified_InjectAndSearch(t *testing.T) {
	provider := setupUnifiedTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	err := worker.InjectTruth(ctx, "org-1", "test-type", "test content", nil)
	if err != nil {
		t.Fatalf("InjectTruth failed: %v", err)
	}

	results, err := worker.SearchTruth(ctx, "org-1", "query", 10)
	if err != nil {
		t.Fatalf("SearchTruth failed: %v", err)
	}

	if len(results) != 1 {
		t.Errorf("expected 1 result, got %d", len(results))
	} else if results[0] != "test content" {
		t.Errorf("expected 'test content', got '%s'", results[0])
	}
}

func TestAutoDreamWorker_Unified_Pruning(t *testing.T) {
	provider := setupUnifiedTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	// Insert stale session (older than 30 days)
	_, err := provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('sess-stale', 'agent-1', 'stale-data', datetime('now', '-31 days'))")
	if err != nil {
		t.Fatalf("failed to insert stale session: %v", err)
	}

	// Insert fresh session
	_, err = provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('sess-fresh', 'agent-1', 'fresh-data', datetime('now'))")
	if err != nil {
		t.Fatalf("failed to insert fresh session: %v", err)
	}

	err = worker.PruneStaleSessions(ctx)
	if err != nil {
		t.Fatalf("PruneStaleSessions failed: %v", err)
	}

	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count sessions: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 session remaining, got %d", count)
	}
}

func TestAutoDreamWorker_Unified_ProcessCompletedTasks(t *testing.T) {
	provider := setupUnifiedTestDB(t)
	worker := NewAutoDreamWorker(provider)
	ctx := context.Background()

	payload := "task result payload"
	_, err := provider.Exec(ctx, "INSERT INTO shared_tasks_decomposition (id, organization_id, status, payload) VALUES ('task-1', 'org-1', 'DONE', ?)", payload)
	if err != nil {
		t.Fatalf("failed to insert completed task: %v", err)
	}

	err = worker.ProcessCompletedTasks(ctx)
	if err != nil {
		t.Fatalf("ProcessCompletedTasks failed: %v", err)
	}

	// Verify task archived
	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM shared_tasks_decomposition WHERE id = 'task-1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query task status: %v", err)
	}
	if status != "ARCHIVED" {
		t.Errorf("expected status 'ARCHIVED', got '%s'", status)
	}

	// Verify memory injected
	results, _ := worker.SearchTruth(ctx, "org-1", "query", 10)
	if len(results) != 1 {
		t.Errorf("expected 1 memory injected, got %d", len(results))
	}
}

func TestAutoDreamWorker_Unified_QueuePruning(t *testing.T) {
    provider := setupUnifiedTestDB(t)
    qm := queue.NewQueueManager(provider)
    worker := NewAutoDreamWorker(provider)
    worker.SetQueueManager(qm)
    ctx := context.Background()

    // Insert stale session
	provider.Exec(ctx, "INSERT INTO agent_session_data (session_id, agent_id, context_data, last_accessed) VALUES ('sess-stale', 'agent-1', 'stale-data', datetime('now', '-31 days'))")

    err := worker.EnqueuePruneJob(ctx, "system")
    if err != nil {
        t.Fatalf("EnqueuePruneJob failed: %v", err)
    }

    // Poll job
    job, err := qm.Poll(ctx, "worker-1")
    if err != nil || job == nil {
        t.Fatalf("Poll failed: %v, job: %v", err, job)
    }

    // Handle job
    err = worker.HandlePruneJob(ctx, job)
    if err != nil {
        t.Fatalf("HandlePruneJob failed: %v", err)
    }

    // Verify pruning
	var count int
	provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_session_data").Scan(&count)
	if count != 0 {
		t.Errorf("expected 0 sessions after job processing, got %d", count)
	}
}
