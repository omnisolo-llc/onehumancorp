package telemetry

import (
	"context"
	"testing"
)

func TestSQLiteExporter_ExportMetric(t *testing.T) {
	provider := &MockProvider{}
	exporter := NewSQLiteExporter(provider)

	labels := map[string]interface{}{"foo": "bar"}
	err := exporter.ExportMetric(context.Background(), "my_metric", 42.0, labels)

	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(provider.ExecCalls) != 4 {
		t.Fatalf("expected 4 exec call args, got %d", len(provider.ExecCalls))
	}

	if str, ok := provider.ExecCalls[0].(string); !ok || str != "my_metric" {
		t.Fatalf("expected metric_name my_metric, got %v", provider.ExecCalls[0])
	}
}

func TestSQLiteExporter_ExportMetric_NoLabels(t *testing.T) {
	provider := &MockProvider{}
	exporter := NewSQLiteExporter(provider)

	ch := make(chan int)
	labels := map[string]interface{}{"foo": ch}

	err := exporter.ExportMetric(context.Background(), "my_metric_2", 12.0, labels)

	if err != nil {
		t.Fatalf("expected no error even with bad labels, got %v", err)
	}

	if len(provider.ExecCalls) != 4 {
		t.Fatalf("expected 4 exec call args, got %d", len(provider.ExecCalls))
	}
}

func TestSQLiteExporter_BufferMetric(t *testing.T) {
	provider := &MockProvider{}
	exporter := NewSQLiteExporter(provider)

	err := exporter.BufferMetric("my_buffer_metric", "counter", 10.0, nil)

	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if len(provider.ExecCalls) != 4 {
		t.Fatalf("expected 4 exec call args, got %d", len(provider.ExecCalls))
	}

	if str, ok := provider.ExecCalls[0].(string); !ok || str != "my_buffer_metric" {
		t.Fatalf("expected metric_name my_buffer_metric, got %v", provider.ExecCalls[0])
	}
}
