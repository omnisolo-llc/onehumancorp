package telemetry

import (
	"context"
	"encoding/json"
	"log/slog"
	"os"
	"sync"
	"time"
)

// MetricPayload represents a single buffered metric event
type MetricPayload struct {
	Type      string            `json:"type"`
	AgentID   string            `json:"agent_id,omitempty"`
	Role      string            `json:"role,omitempty"`
	Model     string            `json:"model,omitempty"`
	TokenType string            `json:"token_type,omitempty"`
	Count     int64             `json:"count,omitempty"`
	API       string            `json:"api,omitempty"`
	Extra     string            `json:"extra,omitempty"`
	Timestamp time.Time         `json:"timestamp"`
}

var (
	bufferMu        sync.RWMutex
	metricBuffer    []MetricPayload
	isStandalone    = os.Getenv("OHC_STANDALONE") == "true"
	maxBufferLimit  = 10000 // prevent OOM
	flushInterval   = 30 * time.Second
	flushCancel     context.CancelFunc
)

func init() {
	if isStandalone {
		startBackgroundFlusher()
	}
}

func startBackgroundFlusher() {
	bufferMu.Lock()
	if flushCancel != nil {
		bufferMu.Unlock()
		return
	}
	ctx, cancel := context.WithCancel(context.Background())
	flushCancel = cancel
	bufferMu.Unlock()

	go func() {
		ticker := time.NewTicker(flushInterval)
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				_ = FlushTelemetry(context.Background())
			}
		}
	}()
}

// bufferMetric locally buffers a metric if in standalone mode
func bufferMetric(payload MetricPayload) {
	bufferMu.RLock()
	standalone := isStandalone
	bufferMu.RUnlock()

	if !standalone {
		return
	}

	bufferMu.Lock()
	defer bufferMu.Unlock()

	// Drop metric if limit exceeded to prevent OOM
	if len(metricBuffer) >= maxBufferLimit {
		slog.Warn("telemetry buffer full, dropping metric", "type", payload.Type)
		return
	}

	metricBuffer = append(metricBuffer, payload)
}


// SetStandaloneMode overrides the standalone mode detection
func SetStandaloneMode(standalone bool) {
	bufferMu.Lock()
	isStandalone = standalone
	bufferMu.Unlock()

	if standalone {
		startBackgroundFlusher()
	} else {
		bufferMu.Lock()
		if flushCancel != nil {
			flushCancel()
			flushCancel = nil
		}
		bufferMu.Unlock()
	}
}

// SIPDBSyncInterface represents a minimal interface for syncing memory
type SIPDBSyncInterface interface {
	UpdateMemory(ctx context.Context, key, value string) error
	SyncMemory(ctx context.Context, key string) (string, error)
}

var (
	sipdbMu     sync.RWMutex
	globalSIPDB SIPDBSyncInterface
)

// SetSIPDB sets the SIPDB interface to use for syncing local metrics
func SetSIPDB(sipdb SIPDBSyncInterface) {
	sipdbMu.Lock()
	defer sipdbMu.Unlock()
	globalSIPDB = sipdb
}

// FlushTelemetry flushes the current metric buffer to the OHC-SIP Cloud DB
func FlushTelemetry(ctx context.Context) error {
	bufferMu.Lock()
	if len(metricBuffer) == 0 {
		bufferMu.Unlock()
		return nil
	}

	// Take a snapshot and clear the buffer
	snapshot := make([]MetricPayload, len(metricBuffer))
	copy(snapshot, metricBuffer)
	metricBuffer = nil
	bufferMu.Unlock()

	sipdbMu.RLock()
	db := globalSIPDB
	sipdbMu.RUnlock()

	if db != nil {
		// Read existing sync data so we don't blindly overwrite pending events
		var existing []MetricPayload
		existingStr, err := db.SyncMemory(ctx, "telemetry_sync")
		if err == nil && existingStr != "" {
			_ = json.Unmarshal([]byte(existingStr), &existing)
		}

		merged := append(existing, snapshot...)

		// To prevent unbounded storage growth in SIPDB json blobs, cap the telemetry history
		if len(merged) > maxBufferLimit {
			merged = merged[len(merged)-maxBufferLimit:]
		}

		data, err := json.Marshal(merged)
		if err != nil {
			return err
		}

		err = db.UpdateMemory(ctx, "telemetry_sync", string(data))
		if err != nil {
			// On failure, put them back
			bufferMu.Lock()
			metricBuffer = append(snapshot, metricBuffer...)
			bufferMu.Unlock()
			slog.Warn("failed to sync telemetry to SIPDB, buffered for retry", "error", err)
			return err
		}

		slog.Info("successfully synced telemetry to SIPDB", "count", len(snapshot), "total_synced", len(merged))
		return nil
	}

	// If no SIPDB, just clear the snapshot (we can't sync it)
	slog.Warn("no SIPDB configured for telemetry sync, discarding buffered metrics", "count", len(snapshot))

	return nil
}

// GetBufferedMetrics returns a copy of currently buffered metrics (mainly for testing)
func GetBufferedMetrics() []MetricPayload {
	bufferMu.Lock()
	defer bufferMu.Unlock()
	snapshot := make([]MetricPayload, len(metricBuffer))
	copy(snapshot, metricBuffer)
	return snapshot
}

// ClearBufferedMetrics clears the current metric buffer
func ClearBufferedMetrics() {
	bufferMu.Lock()
	defer bufferMu.Unlock()
	metricBuffer = nil
}
