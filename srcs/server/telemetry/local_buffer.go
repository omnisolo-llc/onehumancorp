package telemetry

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strings"
	"time"

	"github.com/google/uuid"
)

func IsSensitiveKey(key string) bool {
	k := strings.ToLower(key)
	return strings.Contains(k, "password") ||
		strings.Contains(k, "secret_key") ||
		strings.Contains(k, "api_key") ||
		strings.Contains(k, "token") ||
		strings.Contains(k, "auth") ||
		strings.Contains(k, "cookie") ||
		strings.Contains(k, "credential") ||
		strings.Contains(k, "email") ||
		strings.Contains(k, "phone") ||
		strings.Contains(k, "ssn") ||
		strings.Contains(k, "address") ||
		strings.Contains(k, "first_name") ||
		strings.Contains(k, "last_name") ||
		strings.Contains(k, "full_name") ||
		strings.Contains(k, "pii") ||
		strings.Contains(k, "tenant_id") ||
		strings.Contains(k, "organization_id") ||
		strings.Contains(k, "session_id") ||
		strings.Contains(k, "payload") ||
		strings.Contains(k, "credit_card") ||
		strings.Contains(k, "cvv") ||
		strings.Contains(k, "dob") ||
		strings.Contains(k, "birth") ||
		strings.Contains(k, "passport") ||
		strings.Contains(k, "bank_account") ||
		strings.Contains(k, "stripe") ||
		strings.Contains(k, "billing") ||
		strings.Contains(k, "ip_address") ||
		strings.Contains(k, "mac_address") ||
		strings.Contains(k, "geolocation")
}

func isEmail(s string) bool {
	return strings.Contains(s, "@") && strings.Contains(s, ".")
}

func RedactInterfacePII(val interface{}) interface{} {
	switch v := val.(type) {
	case map[string]interface{}:
		newMap := make(map[string]interface{})
		for k, val := range v {
			if IsSensitiveKey(k) {
				newMap[k] = "[REDACTED]"
			} else {
				newMap[k] = RedactInterfacePII(val)
			}
		}
		return newMap
	case []interface{}:
		newArr := make([]interface{}, len(v))
		for i, val := range v {
			newArr[i] = RedactInterfacePII(val)
		}
		return newArr
	case string:
		if isEmail(v) {
			return "[EMAIL_REDACTED]"
		}
		return v
	default:
		return v
	}
}

// MetricPoint represents a single OpenTelemetry metric data point to be buffered
type MetricPoint struct {
	ID            string                 `json:"id"`
	MetricName    string                 `json:"metric_name"`
	Value         float64                `json:"value"`
	Attributes    map[string]interface{} `json:"attributes"`
	Timestamp     time.Time              `json:"timestamp"`
	SyncedToCloud bool                   `json:"synced_to_cloud"`
}

// TelemetrySyncEngine handles buffering telemetry data locally
// and syncing it to the cloud when online.
type TelemetrySyncEngine struct {
	db          *sql.DB
	remoteURL   string
	httpClient  *http.Client
}

func NewTelemetrySyncEngine(db *sql.DB, remoteURL string) *TelemetrySyncEngine {
	return &TelemetrySyncEngine{
		db:         db,
		remoteURL:  remoteURL,
		httpClient: &http.Client{Timeout: 5 * time.Second},
	}
}

// BufferMetric stores a metric locally in SQLite
func (e *TelemetrySyncEngine) BufferMetric(ctx context.Context, name string, value float64, attrs map[string]interface{}) error {
	id := uuid.New().String()
	redactedAttrs := RedactInterfacePII(attrs)
	attrBytes, err := json.Marshal(redactedAttrs)
	if err != nil {
		return fmt.Errorf("failed to marshal attributes: %w", err)
	}

	query := `INSERT INTO local_telemetry_metrics (id, metric_name, value, attributes, timestamp, synced_to_cloud)
	          VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, FALSE)`
	_, err = e.db.ExecContext(ctx, query, id, name, value, string(attrBytes))
	if err != nil {
		return fmt.Errorf("failed to insert metric: %w", err)
	}
	return nil
}

// SyncPendingMetrics attempts to send buffered metrics to the cloud observability endpoint
func (e *TelemetrySyncEngine) SyncPendingMetrics(ctx context.Context) error {
	rows, err := e.db.QueryContext(ctx, "SELECT id, metric_name, value, attributes, timestamp FROM local_telemetry_metrics WHERE synced_to_cloud = FALSE LIMIT 100")
	if err != nil {
		return fmt.Errorf("failed to query pending metrics: %w", err)
	}
	defer rows.Close()

	var pending []MetricPoint
	for rows.Next() {
		var pt MetricPoint
		var attrStr string
		if err := rows.Scan(&pt.ID, &pt.MetricName, &pt.Value, &attrStr, &pt.Timestamp); err != nil {
			log.Printf("failed to scan metric row: %v", err)
			continue
		}
		if err := json.Unmarshal([]byte(attrStr), &pt.Attributes); err != nil {
			log.Printf("failed to unmarshal attributes for metric %s: %v", pt.ID, err)
			continue
		}
		pending = append(pending, pt)
	}
	if err := rows.Err(); err != nil {
		return fmt.Errorf("rows iteration error: %w", err)
	}

	for _, pt := range pending {
		if err := e.syncToCloud(ctx, pt); err != nil {
			log.Printf("failed to sync metric %s: %v", pt.ID, err)
			continue
		}
		// Mark synced
		_, err := e.db.ExecContext(ctx, "UPDATE local_telemetry_metrics SET synced_to_cloud = TRUE WHERE id = $1", pt.ID)
		if err != nil {
			log.Printf("failed to mark metric %s as synced: %v", pt.ID, err)
		}
	}

	return nil
}

func (e *TelemetrySyncEngine) syncToCloud(ctx context.Context, pt MetricPoint) error {
	payload, err := json.Marshal(pt)
	if err != nil {
		return err
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, e.remoteURL, bytes.NewReader(payload))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := e.httpClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	return nil
}

// StartSyncDaemon periodically attempts to flush the local SQLite telemetry table
func (e *TelemetrySyncEngine) StartSyncDaemon(ctx context.Context, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if err := e.SyncPendingMetrics(ctx); err != nil {
				log.Printf("error syncing metrics: %v", err)
			}
		}
	}
}
