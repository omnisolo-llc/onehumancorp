package telemetry

import (
	"context"
	"testing"
	"time"
)

type mockHealthProvider struct{}

func (m *mockHealthProvider) GetCPUUsage(ctx context.Context) (float64, error) {
	return 10.5, nil
}

func (m *mockHealthProvider) GetMemoryUsage(ctx context.Context) (float64, error) {
	return 20.0, nil
}

func TestTelemetrySyncEngine_Heartbeat(t *testing.T) {
	hp := &mockHealthProvider{}
	engine := NewTelemetrySyncEngine(nil, hp, 100*time.Millisecond)

	called := false
	BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		if metricType == "system_heartbeat" {
			called = true
		}
		return nil
	}
	defer func() { BufferMetricFunc = nil }()

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go engine.heartbeatLoop(ctx)

	time.Sleep(200 * time.Millisecond)

	if !called {
		t.Error("expected heartbeat to be buffered")
	}
}
