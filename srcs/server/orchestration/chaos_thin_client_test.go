package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func TestThinClient_GracefulFailure(t *testing.T) {
	sip, _ := NewSIPDB(":memory:")
	defer sip.Close()
	ctx := context.Background()

	// Seed a mission
	err := sip.UpsertMission(ctx, "thin-client-mission", "PENDING", `{"test":"thin"}`, true)
	if err != nil {
		t.Fatalf("failed to seed mission: %v", err)
	}

	t.Run("Remote Endpoint Unreachable", func(t *testing.T) {
		// Simulate a remote endpoint that is down
		remoteURL := "http://localhost:12345/api/v1/sync"

		syncCtx, cancel := context.WithTimeout(ctx, 100*time.Millisecond)
		defer cancel()

		synced, err := sip.SyncMissions(syncCtx, remoteURL)
		if err == nil {
			t.Errorf("Expected error for unreachable endpoint, got nil")
		}
		if synced != 0 {
			t.Errorf("Expected 0 synced missions, got %d", synced)
		}
	})

	t.Run("Remote Endpoint Slow Response", func(t *testing.T) {
		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			time.Sleep(100 * time.Millisecond)
			w.WriteHeader(http.StatusOK)
		}))
		defer server.Close()

		syncCtx, cancel := context.WithTimeout(ctx, 50*time.Millisecond)
		defer cancel()

		synced, err := sip.SyncMissions(syncCtx, server.URL)
		if err == nil {
			t.Errorf("Expected timeout error, got nil")
		}
		if synced != 0 {
			t.Errorf("Expected 0 synced missions, got %d", synced)
		}
	})
}
