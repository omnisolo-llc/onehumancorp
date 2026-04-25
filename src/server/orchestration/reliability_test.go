package orchestration

import (
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

func TestSanitizeMissions_StuckTransition(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	if err := initializeTables(provider); err != nil {
		t.Fatalf("failed to initialize tables: %v", err)
	}

	sip, _ := NewSIPDBWithProvider(provider, "system")

	// 1. Insert a mission that is > 1h old but < 2h old (should become STUCK)
	stuckTime := time.Now().Add(-90 * time.Minute).UTC().Format("2006-01-02 15:04:05")
	_, err := provider.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, created_at, updated_at, organization_id) VALUES (?, ?, ?, ?, ?, ?)",
		"mission-stuck", "PENDING", "{}", stuckTime, stuckTime, "system")
	if err != nil {
		t.Fatalf("failed to insert mission: %v", err)
	}

	// 2. Insert a mission that is > 2h old (should become FAILED if ageThreshold=2h)
	failTime := time.Now().Add(-3 * time.Hour).UTC().Format("2006-01-02 15:04:05")
	_, err = provider.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, created_at, updated_at, organization_id) VALUES (?, ?, ?, ?, ?, ?)",
		"mission-fail", "PENDING", "{}", failTime, failTime, "system")
	if err != nil {
		t.Fatalf("failed to insert mission: %v", err)
	}

	// 3. Prune missions with 2h threshold
	err = sip.SanitizeMissions(ctx, 2*time.Hour)
	if err != nil {
		t.Fatalf("SanitizeMissions failed: %v", err)
	}

	// Verify transitions
	var status string
	err = provider.QueryRow(ctx, "SELECT status FROM agent_missions WHERE id = ?", "mission-stuck").Scan(&status)
	if err != nil {
		t.Errorf("failed to query mission-stuck: %v", err)
	} else if status != "PENDING" {
		t.Errorf("expected status PENDING, got %s", status)
	}

	// mission-fail should be deleted because it's FAILED and > 2h old
	err = provider.QueryRow(ctx, "SELECT status FROM agent_missions WHERE id = ?", "mission-fail").Scan(&status)
	if err == nil {
		t.Errorf("expected mission-fail to be deleted, but it still exists with status %s", status)
	} else if err != sql.ErrNoRows {
		t.Errorf("unexpected error querying mission-fail: %v", err)
	}
}

func TestCheckHealth_CloudConnectivity(t *testing.T) {
	ctx := context.Background()
	hub := NewHub()

	// Test Cloud Mode (Default)
	probe, err := hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("CheckHealth failed: %v", err)
	}
	if !probe.CloudConnected {
		t.Error("expected CloudConnected to be true in cloud mode")
	}

	// Test Standalone Mode - Success Case
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/health" {
			w.WriteHeader(http.StatusOK)
		}
	}))
	defer ts.Close()

	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_CORE_URL", ts.URL)
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_CORE_URL")

	probe, err = hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("CheckHealth standalone failed: %v", err)
	}
	if !probe.CloudConnected {
		t.Error("expected CloudConnected to be true in standalone mode when server is up")
	}

	// Test Standalone Mode - Failure Case
	os.Setenv("OHC_CORE_URL", "http://invalid-url-that-does-not-exist.local")
	probe, err = hub.CheckHealth(ctx)
	if err != nil {
		t.Fatalf("CheckHealth standalone fail-case failed: %v", err)
	}
	if probe.CloudConnected {
		t.Error("expected CloudConnected to be false in standalone mode when server is unreachable")
	}
}
