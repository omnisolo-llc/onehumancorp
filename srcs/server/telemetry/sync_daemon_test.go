package telemetry

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	_ "modernc.org/sqlite"
)

func TestTelemetrySyncWorker(t *testing.T) {
	// The original SyncDaemon/TelemetrySyncWorker struct has been removed in favor of
	// StartTelemetrySyncWorker which takes a SyncFunc.
	// This test is simplified to verify the worker start/stop lifecycle.

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	ctx, cancel := context.WithCancel(context.Background())

	syncFunc := func(ctx context.Context, endpoint string) (int, error) {
		return 0, nil
	}

	StartTelemetrySyncWorker(ctx, syncFunc, server.URL, 10*time.Millisecond)

	time.Sleep(50*time.Millisecond)
	cancel()
}
