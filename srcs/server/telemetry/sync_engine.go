package telemetry

import (
	"context"
	"encoding/json"
	"log/slog"
	"os"
	"time"
)

// HealthProvider defines the interface for collecting system health metrics.
type HealthProvider interface {
	GetCPUUsage(ctx context.Context) (float64, error)
	GetMemoryUsage(ctx context.Context) (float64, error)
}

// TelemetrySyncEngine orchestrates the synchronization of metrics, traces, and system health.
type TelemetrySyncEngine struct {
	syncDaemon     *SyncDaemon
	healthProvider HealthProvider
	heartbeatFreq  time.Duration
}

// NewTelemetrySyncEngine creates a new TelemetrySyncEngine.
func NewTelemetrySyncEngine(daemon *SyncDaemon, hp HealthProvider, freq time.Duration) *TelemetrySyncEngine {
	if freq == 0 {
		freq = 1 * time.Minute
	}
	return &TelemetrySyncEngine{
		syncDaemon:     daemon,
		healthProvider: hp,
		heartbeatFreq:  freq,
	}
}

// Start begins the synchronization and heartbeat loops.
func (e *TelemetrySyncEngine) Start(ctx context.Context) {
	// Start the underlying sync daemon for buffered metrics
	if e.syncDaemon != nil {
		go e.syncDaemon.Start(ctx)
	}

	// Start the heartbeat loop for system health
	go e.heartbeatLoop(ctx)
}

func (e *TelemetrySyncEngine) heartbeatLoop(ctx context.Context) {
	ticker := time.NewTicker(e.heartbeatFreq)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			e.sendHeartbeat(ctx)
		}
	}
}

func (e *TelemetrySyncEngine) sendHeartbeat(ctx context.Context) {
	if e.healthProvider == nil {
		return
	}

	cpu, err := e.healthProvider.GetCPUUsage(ctx)
	if err != nil {
		slog.Warn("Failed to get CPU usage for heartbeat", "error", err)
	}

	mem, err := e.healthProvider.GetMemoryUsage(ctx)
	if err != nil {
		slog.Warn("Failed to get memory usage for heartbeat", "error", err)
	}

	heartbeatData := map[string]interface{}{
		"cpu_usage_percent":    cpu,
		"memory_usage_percent": mem,
		"timestamp":           time.Now().UTC().Format(time.RFC3339),
		"hostname":            os.Getenv("HOSTNAME"),
	}

	payloadBytes, _ := json.Marshal(heartbeatData)

	// Buffer the heartbeat as a metric so it gets synced by the SyncDaemon
	if BufferMetricFunc != nil {
		_ = BufferMetricFunc(ctx, "system_heartbeat", string(payloadBytes))
	}
}
