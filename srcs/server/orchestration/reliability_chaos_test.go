package orchestration

import (
	"context"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
	"time"
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
	sip, _ := NewSIPDB(":memory:")
	defer sip.Close()
	ctx := context.Background()

	err := sip.UpsertMission(ctx, "network-mission", "PENDING", `{"test":"partition"}`, true)
	if err != nil {
		t.Fatalf("failed to seed mission: %v", err)
	}

	// Use an invalid port to simulate connection refused / network partition
	synced, err := sip.SyncMissions(ctx, "http://127.0.0.1:1")
	if err == nil {
		t.Error("Expected error for network partition, got nil")
	}
	if synced != 0 {
		t.Errorf("Expected 0 synced missions, got %d", synced)
	}

	// Verify local state is still PENDING
	missions, err := sip.GetPendingMissions(ctx, "ANY")
	if err != nil {
		t.Fatalf("failed to get pending missions: %v", err)
	}
	found := false
	for _, m := range missions {
		if m.ID == "network-mission" {
			found = true
			break
		}
	}
	if !found {
		t.Error("Mission should still be PENDING locally after failed sync")
	}
}

func TestSIPDB_SQLiteLockContention_Chaos(t *testing.T) {
	// SQLite :memory: doesn't easily support multi-connection locking tests in the same process
	// with the current setup, but we can mock the behavior by manually triggering withSipRetry
	// logic or using a file-based DB with a manual lock.

	tempDir, err := os.MkdirTemp("", "sip-chaos-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)
	dbPath := filepath.Join(tempDir, "chaos.db")

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
