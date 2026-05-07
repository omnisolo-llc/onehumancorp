package telemetry

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
	"strings"
	"time"
)

type SyncWorker struct {
	db           *sql.DB
	cloudURL     string
	syncInterval time.Duration
	httpClient   *http.Client
	backoff      time.Duration
}

type TelemetryRecord struct {
	ID          string  `json:"id"`
	MetricName  string  `json:"metric_name"`
	MetricValue float64 `json:"metric_value"`
	Attributes  string  `json:"attributes"`
	CreatedAt   string  `json:"created_at"`
}

func NewSyncWorker(db *sql.DB, cloudURL string, syncInterval time.Duration) *SyncWorker {
	return &SyncWorker{
		db:           db,
		cloudURL:     cloudURL,
		syncInterval: syncInterval,
		httpClient: &http.Client{
			Timeout: 10 * time.Second,
		},
		backoff: 0,
	}
}

func (w *SyncWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.syncInterval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			// Process as many batches as possible while there are records
			for {
				if w.backoff > 0 {
					select {
					case <-ctx.Done():
						return
					case <-time.After(w.backoff):
						// Continue after backoff
					}
				}

				select {
				case <-ctx.Done():
					return
				default:
				}

				hasMore := w.sync()
				if !hasMore {
					break
				}
			}
		}
	}
}

func (w *SyncWorker) sync() bool {
	if !telemetryEnabled || !isStandalone {
		return false // Nothing to do if telemetry is disabled or we are not in standalone mode
	}

	rows, err := w.db.Query("SELECT id, metric_name, metric_value, attributes, created_at FROM telemetry_buffer LIMIT 500")
	if err != nil {
		log.Printf("Failed to query telemetry_buffer: %v", err)
		return false
	}
	defer rows.Close()

	var records []TelemetryRecord
	var ids []string
	for rows.Next() {
		var rec TelemetryRecord
		if err := rows.Scan(&rec.ID, &rec.MetricName, &rec.MetricValue, &rec.Attributes, &rec.CreatedAt); err != nil {
			log.Printf("Failed to scan telemetry_buffer row: %v", err)
			continue
		}
		records = append(records, rec)
		ids = append(ids, rec.ID)
	}

	if len(records) == 0 {
		return false // Nothing to sync
	}

	payload, err := json.Marshal(records)
	if err != nil {
		log.Printf("Failed to marshal telemetry records: %v", err)
		return false
	}

	req, err := http.NewRequest("POST", w.cloudURL, bytes.NewBuffer(payload))
	if err != nil {
		log.Printf("Failed to create telemetry sync request: %v", err)
		return false
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("X-OHC-Conflict-Resolution", "force-local")

	resp, err := w.httpClient.Do(req)
	if err != nil {
		log.Printf("Failed to send telemetry records to cloud: %v", err)
		w.applyBackoff()
		return false
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 200 && resp.StatusCode < 300 {
		// Successful sync, delete records from buffer
		w.resetBackoff()

		if len(ids) > 0 {
			placeholders := make([]string, len(ids))
			args := make([]interface{}, len(ids))
			for i, id := range ids {
				placeholders[i] = "?"
				args[i] = id
			}

			query := "DELETE FROM telemetry_buffer WHERE id IN (" + strings.Join(placeholders, ",") + ")"
			_, err := w.db.Exec(query, args...)
			if err != nil {
				log.Printf("Failed to delete synced records from telemetry_buffer: %v", err)
			}
		}

		return len(records) == 500 // Return true if there might be more records
	} else {
		log.Printf("Cloud rejected telemetry records. Status code: %d", resp.StatusCode)
		w.applyBackoff()
		return false
	}
}

func (w *SyncWorker) applyBackoff() {
	if w.backoff == 0 {
		w.backoff = 1 * time.Second
	} else {
		w.backoff *= 2
		if w.backoff > 5*time.Minute {
			w.backoff = 5 * time.Minute
		}
	}
}

func (w *SyncWorker) resetBackoff() {
	w.backoff = 0
}
