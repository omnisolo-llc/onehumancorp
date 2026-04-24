package telemetry

import (
	"encoding/json"

	"context"
	"os"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/push"
	"go.opentelemetry.io/otel/metric"

"database/sql"
	"fmt"
	"regexp"
	"time"
)

var validTableName = regexp.MustCompile(`^[a-zA-Z_][a-zA-Z0-9_]*$`)

// UpdateRemoteStoreWithLWW updates a remote Postgres store using LWW (Last-Writer-Wins) conflict resolution.
// It uses an upsert query condition: WHERE excluded.updated_at > [table].updated_at
func UpdateRemoteStoreWithLWW(ctx context.Context, db *sql.DB, tableName string, id string, payload string, updatedAt time.Time) error {
	if !validTableName.MatchString(tableName) {
		return fmt.Errorf("invalid table name: %s", tableName)
	}

	query := fmt.Sprintf(`
		INSERT INTO "%s" (id, payload, updated_at)
		VALUES ($1, $2, $3)
		ON CONFLICT (id) DO UPDATE
		SET payload = excluded.payload, updated_at = excluded.updated_at
		WHERE excluded.updated_at > "%s".updated_at
	`, tableName, tableName)

	_, err := db.ExecContext(ctx, query, id, payload, updatedAt)
	return err
}


var (
	bridgeMessagesSentTotal     metric.Int64Counter
	bridgeMessagesReceivedTotal metric.Int64Counter
	bridgeStatusGauge           metric.Int64UpDownCounter
)

func initBridgeMetrics() {
	var err error
	if meter == nil {
		return // telemetry not fully initialized, wait for InitTelemetry // telemetry not fully initialized, wait for InitTelemetry
	}

	bridgeMessagesSentTotal, err = meter.Int64Counter(
		"ohc_mesh_bridge_messages_sent_total",
		metric.WithDescription("Total number of Teammate Mesh bridge messages sent"),
	)
	if err != nil {
		panic(err)
	}

	bridgeMessagesReceivedTotal, err = meter.Int64Counter(
		"ohc_mesh_bridge_messages_received_total",
		metric.WithDescription("Total number of Teammate Mesh bridge messages received"),
	)
	if err != nil {
		panic(err)
	}

	bridgeStatusGauge, err = meter.Int64UpDownCounter(
		"ohc_mesh_bridge_status_gauge",
		metric.WithDescription("Active Teammate Mesh bridge connections"),
	)
	if err != nil {
		panic(err)
	}
}

func RecordBridgeMessageSent(ctx context.Context) {
	if BufferMetricFunc != nil {
		payloadBytes, _ := json.Marshal(RedactInterfacePII(map[string]interface{}{}))
		_ = BufferMetricFunc(ctx, "bridge_message_sent", string(payloadBytes))
	}
	if bridgeMessagesSentTotal != nil {
		bridgeMessagesSentTotal.Add(ctx, 1)
	}
}

func RecordBridgeMessageReceived(ctx context.Context) {
	if BufferMetricFunc != nil {
		payloadBytes, _ := json.Marshal(RedactInterfacePII(map[string]interface{}{}))
		_ = BufferMetricFunc(ctx, "bridge_message_received", string(payloadBytes))
	}
	if bridgeMessagesReceivedTotal != nil {
		bridgeMessagesReceivedTotal.Add(ctx, 1)
	}
}

func RecordBridgeStatus(ctx context.Context, active int64) {
	if BufferMetricFunc != nil {
		payloadBytes, _ := json.Marshal(RedactInterfacePII(map[string]interface{}{"active": active}))
		_ = BufferMetricFunc(ctx, "bridge_status", string(payloadBytes))
	}
	if bridgeStatusGauge != nil {
		bridgeStatusGauge.Add(ctx, active)
	}
}

func PushMetrics(ctx context.Context, jobName string) error {
	url := os.Getenv("PROMETHEUS_PUSHGATEWAY_URL")
	if url == "" {
		return nil
	}
	return push.New(url, jobName).Gatherer(prometheus.DefaultGatherer).Push()
}
