package sync

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/telemetry"
)

func TestMCPSyncClient_SyncDeltas_Success(t *testing.T) {
	// Setup telemetry
	_, err := telemetry.InitTelemetry()
	if err != nil {
		t.Logf("failed to init telemetry: %v", err)
	}

	// Mock server
	var receivedPayload syncDeltasPayload
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/v1/sync/mcp-deltas" {
			w.WriteHeader(http.StatusNotFound)
			return
		}
		if err := json.NewDecoder(r.Body).Decode(&receivedPayload); err != nil {
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	t.Run("Standalone Mode with Telemetry", func(t *testing.T) {
		os.Setenv("OHC_STANDALONE", "true")
		os.Setenv("OHC_TELEMETRY_ENABLED", "true")
		defer os.Unsetenv("OHC_STANDALONE")
		defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

		client := NewMCPSyncClient(server.URL)
		deltas := []SyncDelta{
			{ID: "d1", EntityID: "e1", Data: "{}", UpdatedAt: time.Now()},
		}

		err := client.SyncDeltas(context.Background(), deltas)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		if len(receivedPayload.Deltas) != 1 || receivedPayload.Deltas[0].ID != "d1" {
			t.Errorf("expected payload to match sent deltas")
		}
	})

	t.Run("Cloud Mode", func(t *testing.T) {
		os.Setenv("OHC_STANDALONE", "false")
		os.Setenv("OHC_TELEMETRY_ENABLED", "true")
		defer os.Unsetenv("OHC_STANDALONE")
		defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

		receivedPayload = syncDeltasPayload{} // reset
		client := NewMCPSyncClient(server.URL)
		deltas := []SyncDelta{
			{ID: "d2", EntityID: "e2", Data: "{}", UpdatedAt: time.Now()},
		}

		err := client.SyncDeltas(context.Background(), deltas)
		if err != nil {
			t.Fatalf("unexpected error: %v", err)
		}

		if len(receivedPayload.Deltas) != 1 || receivedPayload.Deltas[0].ID != "d2" {
			t.Errorf("expected payload to match sent deltas")
		}
	})
}

func TestMCPSyncClient_SyncDeltas_Failure(t *testing.T) {
	_, err := telemetry.InitTelemetry()
	if err != nil {
		t.Logf("failed to init telemetry: %v", err)
	}

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer server.Close()

	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	client := NewMCPSyncClient(server.URL)
	deltas := []SyncDelta{
		{ID: "d1", EntityID: "e1", Data: "{}", UpdatedAt: time.Now()},
	}

	err = client.SyncDeltas(context.Background(), deltas)
	if err == nil {
		t.Fatalf("expected error on 500 response, got nil")
	}
}

func TestMCPSyncClient_SyncDeltas_Empty(t *testing.T) {
	client := NewMCPSyncClient("http://invalid")
	err := client.SyncDeltas(context.Background(), []SyncDelta{})
	if err != nil {
		t.Fatalf("expected nil error for empty deltas, got %v", err)
	}
}
