package telemetry

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
)

// InitStandaloneBuffer configures the telemetry system to buffer metrics locally in SQLite.
func InitStandaloneBuffer(db *sql.DB) {
	BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
		var data map[string]interface{}
		if err := json.Unmarshal([]byte(payload), &data); err == nil {
			redactedData := RedactInterfacePII(data)
			if redactedBytes, err := json.Marshal(redactedData); err == nil {
				payload = string(redactedBytes)
			}
		}

		_, err := db.ExecContext(ctx, "INSERT INTO local_telemetry_buffer (metric_type, payload) VALUES ($1, $2)", metricType, payload)
		if err != nil {
			return fmt.Errorf("failed to buffer metric %s: %w", metricType, err)
		}
		return nil
	}
}
