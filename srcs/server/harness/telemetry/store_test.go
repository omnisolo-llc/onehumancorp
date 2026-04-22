package telemetry

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/sdk/metric"
	"go.opentelemetry.io/otel/sdk/metric/metricdata"
)

func TestViolationStore_RecordViolation(t *testing.T) {
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Initialize test database provider
	provider := db.NewTestProvider(t)

	// Run migration to create table
	_, err := provider.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_violations (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			agent_id TEXT NOT NULL,
			session_id TEXT NOT NULL,
			violation_type TEXT NOT NULL,
			details JSONB,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	reader := metric.NewManualReader()
	meterProvider := metric.NewMeterProvider(metric.WithReader(reader))
	meter := meterProvider.Meter("test_meter")

	store, err := NewViolationStore(provider, meter)
	if err != nil {
		t.Fatalf("failed to create store: %v", err)
	}

	err = store.RecordViolation(ctx, "agent-1", "session-1", "file", "unauthorized access to /etc/shadow")
	if err != nil {
		t.Fatalf("failed to record violation: %v", err)
	}

	// Verify it was written to the DB
	var count int
	err = provider.QueryRow(ctx, "SELECT COUNT(*) FROM agent_violations WHERE tenant_id = $1", "org-123").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query db: %v", err)
	}

	if count != 1 {
		t.Errorf("expected 1 violation in db for org-123, got %d", count)
	}

	// Verify metrics were emitted
	var rm metricdata.ResourceMetrics
	err = reader.Collect(context.Background(), &rm)
	if err != nil {
		t.Fatalf("failed to collect metrics: %v", err)
	}

	if len(rm.ScopeMetrics) == 0 || len(rm.ScopeMetrics[0].Metrics) == 0 {
		t.Fatalf("expected metrics to be emitted, got none")
	}

	m := rm.ScopeMetrics[0].Metrics[0]
	if m.Name != "ohc_agent_violations_total" {
		t.Errorf("expected metric name 'ohc_agent_violations_total', got '%s'", m.Name)
	}

	data := m.Data.(metricdata.Sum[int64])
	if len(data.DataPoints) == 0 {
		t.Fatalf("expected data points, got none")
	}

	dp := data.DataPoints[0]
	if dp.Value != 1 {
		t.Errorf("expected metric value 1, got %d", dp.Value)
	}

	hasLabel := false
	for _, attr := range dp.Attributes.ToSlice() {
		if attr.Key == attribute.Key("type") && attr.Value.AsString() == "file" {
			hasLabel = true
			break
		}
	}

	if !hasLabel {
		t.Errorf("expected metric to have label type=file, got attributes: %v", dp.Attributes)
	}
}
