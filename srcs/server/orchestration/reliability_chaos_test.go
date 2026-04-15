package orchestration

import (
	"context"
	"fmt"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"testing"
	"time"
	"database/sql"
	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestSIPDB_SyncMissions_Chaos(t *testing.T) {
	// Setup a mock SIPDB with in-memory SQLite
	sip, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sip.Close()

	ctx := context.Background()

	// Seed some missions
	for i := 0; i < 5; i++ {
		missionID := fmt.Sprintf("chaos-mission-%d", i)
		err := sip.UpsertMission(ctx, missionID, "PENDING", `{"test":"chaos"}`, true)
		if err != nil {
			t.Fatalf("failed to seed mission: %v", err)
		}
	}

	// Case 1: Remote endpoint is flaky (returns 500)
	server500 := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer server500.Close()

	synced, err := sip.SyncMissions(ctx, server500.URL)
	if err == nil {
		t.Log("SyncMissions handled 500 status gracefully (returned no error as it continues to next mission)")
	}
	if synced != 0 {
		t.Errorf("Expected 0 synced missions on 500 error, got %d", synced)
	}

	// Case 2: Remote endpoint is slow/timeout
	serverTimeout := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(200 * time.Millisecond) // Longer than our test ctx might like if we were strict
		w.WriteHeader(http.StatusOK)
	}))
	defer serverTimeout.Close()

	shortCtx, cancel := context.WithTimeout(ctx, 50*time.Millisecond)
	defer cancel()

	synced, err = sip.SyncMissions(shortCtx, serverTimeout.URL)
	if err != nil {
		t.Logf("SyncMissions correctly timed out: %v", err)
	}
	if synced != 0 {
		t.Errorf("Expected 0 synced missions on timeout, got %d", synced)
	}

	// Case 3: Database is locked during sync (simulated via another connection if possible,
	// but here we just verify that the logic doesn't crash)
	// Success case
	serverOk := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer serverOk.Close()

	synced, err = sip.SyncMissions(ctx, serverOk.URL)
	if err != nil {
		t.Errorf("SyncMissions failed on valid endpoint: %v", err)
	}
	if synced != 5 {
		t.Errorf("Expected 5 synced missions, got %d", synced)
	}
}

func TestSIPDB_SyncMissions_NetworkPartition(t *testing.T) {
	sip, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sip.Close()
	ctx := context.Background()

	err = sip.UpsertMission(ctx, "partition-1", "PENDING", `{"test":"partition"}`, true)
	if err != nil {
		t.Fatalf("failed to seed mission: %v", err)
	}

	// Simulate complete partition by using a URL that will instantly fail dial
	synced, err := sip.SyncMissions(ctx, "http://127.0.0.1:0")
	if err == nil {
		t.Errorf("Expected network error on partition, got nil")
	}
	if synced != 0 {
		t.Errorf("Expected 0 synced on partition, got %d", synced)
	}

	// Make sure the mission is still pending and wasn't lost
	var status string
	err = sip.db.QueryRow(ctx, "SELECT status FROM agent_missions WHERE id = 'partition-1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to verify local mission status: %v", err)
	}
	if status != "PENDING" {
		t.Errorf("Expected status to remain PENDING during partition, got %s", status)
	}
}

func TestWithSipRetry_Chaos(t *testing.T) {
	// Standalone SQLite contention simulation
	dbPath := filepath.Join(t.TempDir(), "chaos.db")
	sip, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sip.Close()

	ctx := context.Background()

	// Manually inject a "database is locked" error into withSipRetry
	calls := 0
	err = withSipRetry(ctx, func() error {
		calls++
		if calls < 2 {
			return fmt.Errorf("database is locked")
		}
		return nil
	})

	if err != nil {
		t.Errorf("withSipRetry failed to recover from transient lock: %v", err)
	}
	if calls != 2 {
		t.Errorf("Expected 2 calls due to retry, got %d", calls)
	}
}

func TestSIPDB_PruneStaleMissions_Parity(t *testing.T) {
	sip, _ := NewSIPDB(":memory:")
	defer sip.Close()
	ctx := context.Background()

	// Insert an old mission
	oldTime := time.Now().Add(-48 * time.Hour).UTC().Format("2006-01-02 15:04:05")
	_, err := sip.db.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, created_at, organization_id) VALUES ($1, $2, $3, $4, $5)",
		"old-mission", "PENDING", "{}", oldTime, "system")
	if err != nil {
		t.Fatalf("failed to insert old mission: %v", err)
	}

	// Prune with 24h threshold
	err = sip.PruneStaleMissions(ctx, 24*time.Hour)
	if err != nil {
		t.Errorf("PruneStaleMissions failed: %v", err)
	}

	// Verify it was marked as FAILED first (sanitization) then deleted (pruning)
	// Actually PruneStaleMissions marks as FAILED then DELETES in the same call if it matches both.
	var status string
	err = sip.db.QueryRow(ctx, "SELECT status FROM agent_missions WHERE id = 'old-mission'").Scan(&status)
	if err != nil && err.Error() != "sql: no rows in result set" {
		t.Errorf("Unexpected error querying pruned mission: %v", err)
	}
	if err == nil {
		t.Errorf("Mission 'old-mission' should have been pruned/deleted")
	}
}

func TestSharedTaskOrchestrator_Chaos(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer sqlDB.Close()
	prov := db.NewSqliteProvider(sqlDB)

	// Create the orchestrator
	orchestrator := NewSharedTaskOrchestrator(prov)

	ctx := context.Background()

	// Setup necessary tables
	_, err = prov.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            agent_id TEXT,
            priority TEXT NOT NULL DEFAULT 'P2',
            payload TEXT,
            parent_plan_id TEXT,
            dependencies TEXT NOT NULL DEFAULT '[]',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    `)
	if err != nil {
		t.Fatalf("failed to create shared_tasks: %v", err)
	}

	_, err = prov.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS state_machine_transitions (
            id TEXT PRIMARY KEY,
            entity_id TEXT NOT NULL,
            entity_type TEXT NOT NULL,
            from_state TEXT NOT NULL,
            to_state TEXT NOT NULL,
            agent_id TEXT,
            reason TEXT,
            occurred_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    `)
	if err != nil {
		t.Fatalf("failed to create state_machine_transitions: %v", err)
	}

	// Chaos 1: Try to acquire a task when there are none, shouldn't crash
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})
	task, err := orchestrator.ClaimTask(ctxWithClaims, "agent-1")
	if err != nil {
		t.Errorf("Expected nil error acquiring task when none exist: %v", err)
	}
	if task != nil {
		t.Errorf("Expected nil task")
	}

	// Chaos 2: Transition a non-existent task, shouldn't crash
	err = orchestrator.TransitionTask(ctxWithClaims, "non-existent-task", "agent-1", "PENDING", "IN_PROGRESS", "chaos test")
	if err == nil {
		t.Errorf("Expected error transitioning non-existent task")
	}
}
