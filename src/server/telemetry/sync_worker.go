package main

import (
	"bytes"
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
	"os"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

type Metric struct {
	ID         int       `json:"-"`
	MetricName string    `json:"metric_name"`
	MetricType string    `json:"metric_type"`
	Value      float32   `json:"value"`
	LabelsJSON string    `json:"-"`
	Labels     any       `json:"labels"`
	Timestamp  time.Time `json:"timestamp"`
}

func SyncMetricsToCloud(db *sql.DB, cloudURL string) error {
	rows, err := db.Query("SELECT id, metric_name, metric_type, value, labels_json, timestamp FROM local_telemetry_buffer WHERE sync_status = 'pending' LIMIT 100")
	if err != nil {
		return err
	}
	defer rows.Close()

	var metrics []Metric
	var ids []int
	for rows.Next() {
		var m Metric
		var ts string
		if err := rows.Scan(&m.ID, &m.MetricName, &m.MetricType, &m.Value, &m.LabelsJSON, &ts); err != nil {
			log.Printf("Failed to scan metric: %v", err)
			continue
		}

		parsedTime, err := time.Parse(time.RFC3339Nano, ts)
		if err == nil {
			m.Timestamp = parsedTime
		}

		var labels any
		json.Unmarshal([]byte(m.LabelsJSON), &labels)
		m.Labels = labels

		metrics = append(metrics, m)
		ids = append(ids, m.ID)
	}

	if len(metrics) == 0 {
		return nil
	}

	payload, err := json.Marshal(metrics)
	if err != nil {
		return err
	}

	resp, err := http.Post(cloudURL+"/api/telemetry/sync", "application/json", bytes.NewBuffer(payload))
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode == 200 {
		for _, id := range ids {
			db.Exec("UPDATE local_telemetry_buffer SET sync_status = 'synced' WHERE id = ?", id)
		}
		log.Printf("Successfully synced %d metrics to cloud", len(metrics))
	} else {
		log.Printf("Failed to sync metrics, status code: %d", resp.StatusCode)
	}

	return nil
}

func main() {
	dbURL := os.Getenv("DATABASE_URL")
	if dbURL == "" {
		dbURL = "sqlite://ohc-standalone.db"
	}

	// remove "sqlite://" prefix for go-sqlite3
	if len(dbURL) > 9 && dbURL[:9] == "sqlite://" {
		dbURL = dbURL[9:]
	}

	cloudURL := os.Getenv("CLOUD_URL")
	if cloudURL == "" {
		cloudURL = "https://cloud.onehumancorp.com"
	}

	db, err := sql.Open("sqlite3", dbURL)
	if err != nil {
		log.Fatalf("Failed to open database: %v", err)
	}
	defer db.Close()

	ticker := time.NewTicker(60 * time.Second)
	for range ticker.C {
		if err := SyncMetricsToCloud(db, cloudURL); err != nil {
			log.Printf("Error syncing telemetry: %v", err)
		}
	}
}
