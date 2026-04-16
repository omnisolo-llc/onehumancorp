package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

// TestThinClientDegradation validates that the client fails gracefully
// when remote backend latency spikes (stress testing network connections without crashing).
func TestThinClientDegradation(t *testing.T) {
	sip, err := NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create SIPDB: %v", err)
	}
	defer sip.Close()
	ctx := context.Background()

	err = sip.UpsertMission(ctx, "latency-mission", "PENDING", `{"test":"latency"}`, true)
	if err != nil {
		t.Fatalf("failed to seed mission: %v", err)
	}

	// Create a remote endpoint that takes an absurdly long time to respond
	// mimicking severe latency / resource exhaustion on the Cloud side
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		time.Sleep(2 * time.Second)
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	// Thin Client should set short aggressive timeouts to degrade safely
	syncCtx, cancel := context.WithTimeout(ctx, 100*time.Millisecond)
	defer cancel()

	synced, err := sip.SyncMissions(syncCtx, server.URL)

	// Expect the context deadline to abort the sync cleanly
	if err == nil {
		t.Errorf("Expected context deadline exceeded error, got nil")
	}
	if synced != 0 {
		t.Errorf("Expected 0 synced missions under latency spike, got %d", synced)
	}
}
