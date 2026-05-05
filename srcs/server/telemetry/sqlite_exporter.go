package telemetry

import (
	"context"
	"encoding/json"
	"fmt"
	"onehumancorp/srcs/server/db"
	"time"
)

type SQLiteExporter struct {
	dbProvider db.Provider
}

func NewSQLiteExporter(provider db.Provider) *SQLiteExporter {
	return &SQLiteExporter{dbProvider: provider}
}

func (e *SQLiteExporter) ExportMetric(ctx context.Context, metricName string, value float64, labels map[string]interface{}) error {
	labelsJSON, err := json.Marshal(labels)
	if err != nil {
		labelsJSON = []byte("{}")
	}

	_, err = e.dbProvider.Exec(
		"INSERT INTO telemetry_buffer (metric_name, value, labels_json, timestamp, sync_status) VALUES (?, ?, ?, ?, 'pending')",
		metricName, value, string(labelsJSON), time.Now().UTC().Format(time.RFC3339),
	)

	if err != nil {
		return fmt.Errorf("failed to insert metric into sqlite buffer: %w", err)
	}

	return nil
}

// BufferMetric implements the TelemetryClient interface used by HybridContextTool
func (e *SQLiteExporter) BufferMetric(metricName string, metricType string, value float64, labels map[string]interface{}) error {
	// We append metricType to labels to ensure no information is lost, since our buffer table does not have a metric_type column
	if labels == nil {
		labels = make(map[string]interface{})
	}
	labels["metric_type"] = metricType
	return e.ExportMetric(context.Background(), metricName, value, labels)
}
