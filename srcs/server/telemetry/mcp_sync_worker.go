package telemetry

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strings"
	"time"

	"onehumancorp/srcs/server/db"
)

type McpSyncWorker struct {
	dbProvider db.Provider
	syncInterval time.Duration
	endpointURL string
	client *http.Client
}

func NewMcpSyncWorker(provider db.Provider, interval time.Duration, endpointURL string, client *http.Client) *McpSyncWorker {
	if client == nil {
		client = &http.Client{Timeout: 10 * time.Second}
	}
	return &McpSyncWorker{
		dbProvider: provider,
		syncInterval: interval,
		endpointURL: endpointURL,
		client: client,
	}
}

func (w *McpSyncWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.syncInterval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.syncMetrics(ctx)
		}
	}
}

type MetricPayload struct {
	MetricName string `json:"metric_name"`
	Value float64 `json:"value"`
	Labels map[string]interface{} `json:"labels"`
	Timestamp string `json:"timestamp"`
}

func (w *McpSyncWorker) syncMetrics(ctx context.Context) {
	// Query for pending metrics
	rows, err := w.dbProvider.Query("SELECT id, metric_name, value, labels_json, timestamp FROM telemetry_buffer WHERE sync_status = 'pending' LIMIT 100")
	if err != nil {
		log.Printf("Failed to query pending metrics: %v\n", err)
		return
	}
	defer rows.Close()

	var ids []int
	var payloads []MetricPayload

	for rows.Next() {
		var id int
		var metricName string
		var value float64
		var labelsJson string
		var timestamp string

		err = rows.Scan(&id, &metricName, &value, &labelsJson, &timestamp)
		if err != nil {
			log.Printf("Failed to scan metric row: %v\n", err)
			continue
		}

		var labels map[string]interface{}
		if err := json.Unmarshal([]byte(labelsJson), &labels); err != nil {
			labels = make(map[string]interface{})
		}

		payloads = append(payloads, MetricPayload{
			MetricName: metricName,
			Value: value,
			Labels: labels,
			Timestamp: timestamp,
		})

		ids = append(ids, id)
	}

	if err = rows.Err(); err != nil {
		log.Printf("Error iterating over metric rows: %v\n", err)
		return
	}

	if len(payloads) == 0 {
		return
	}

	jsonData, err := json.Marshal(payloads)
	if err != nil {
		log.Printf("Failed to marshal payloads: %v\n", err)
		return
	}

	req, err := http.NewRequestWithContext(ctx, "POST", w.endpointURL, bytes.NewBuffer(jsonData))
	if err != nil {
		log.Printf("Failed to create request: %v\n", err)
		return
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := w.client.Do(req)
	if err != nil {
		log.Printf("Failed to send metrics to cloud: %v\n", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 200 && resp.StatusCode < 300 {
		if len(ids) > 0 {
			placeholders := make([]string, len(ids))
			args := make([]interface{}, len(ids))
			for i, id := range ids {
				placeholders[i] = "?"
				args[i] = id
			}
			query := fmt.Sprintf("DELETE FROM telemetry_buffer WHERE id IN (%s)", strings.Join(placeholders, ","))
			_, err = w.dbProvider.Exec(query, args...)
			if err != nil {
				log.Printf("Failed to delete synced metrics: %v\n", err)
			}
		}
	} else {
		log.Printf("Cloud endpoint returned status: %d\n", resp.StatusCode)
	}
}
