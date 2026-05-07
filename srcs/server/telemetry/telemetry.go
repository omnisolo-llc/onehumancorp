package telemetry

import (
	"database/sql"
	"encoding/json"
	"log"
	"os"
	"sync"
	"time"

	"github.com/google/uuid"
)

var (
	telemetryDB       *sql.DB
	isStandalone      bool
	telemetryEnabled  bool
	metricChan        chan metricEvent
	initOnce          sync.Once
	stopFlushChan     chan struct{}
)

type metricEvent struct {
	Name       string
	Value      float64
	Attributes map[string]string
	CreatedAt  time.Time
}

// InitTelemetry initializes the telemetry system.
// It checks OHC_STANDALONE and configures opt-in telemetry.
func InitTelemetry(db *sql.DB) {
	initOnce.Do(func() {
		telemetryDB = db
		if os.Getenv("OHC_STANDALONE") == "true" {
			isStandalone = true
			if os.Getenv("OHC_TELEMETRY_ENABLED") == "true" {
				telemetryEnabled = true
			} else {
				telemetryEnabled = false
			}
		} else {
			isStandalone = false
			telemetryEnabled = true
		}

		if isStandalone && telemetryEnabled && telemetryDB != nil {
			metricChan = make(chan metricEvent, 10000)
			stopFlushChan = make(chan struct{})
			go flushMetrics()
		}
	})
}

// ShutdownTelemetry cleanly stops the telemetry buffer flush routine.
func ShutdownTelemetry() {
	if stopFlushChan != nil {
		close(stopFlushChan)
	}
}

func flushMetrics() {
	ticker := time.NewTicker(500 * time.Millisecond)
	defer ticker.Stop()

	var batch []metricEvent

	for {
		select {
		case <-stopFlushChan:
			// Flush any remaining metrics before shutting down
			if len(batch) > 0 {
				insertBatch(batch)
			}
			return
		case event, ok := <-metricChan:
			if !ok {
				insertBatch(batch)
				return
			}
			batch = append(batch, event)
			if len(batch) >= 100 {
				insertBatch(batch)
				batch = batch[:0]
			}
		case <-ticker.C:
			if len(batch) > 0 {
				insertBatch(batch)
				batch = batch[:0]
			}
		}
	}
}

func insertBatch(batch []metricEvent) {
	if len(batch) == 0 || telemetryDB == nil {
		return
	}

	tx, err := telemetryDB.Begin()
	if err != nil {
		log.Printf("Failed to begin transaction for telemetry_buffer: %v", err)
		return
	}
	defer tx.Rollback()

	stmt, err := tx.Prepare(`INSERT INTO telemetry_buffer (id, metric_name, metric_value, attributes, created_at) VALUES (?, ?, ?, ?, ?)`)
	if err != nil {
		log.Printf("Failed to prepare statement for telemetry_buffer: %v", err)
		return
	}
	defer stmt.Close()

	for _, event := range batch {
		attrJson, _ := json.Marshal(event.Attributes)
		id := uuid.New().String()
		_, err := stmt.Exec(id, event.Name, event.Value, string(attrJson), event.CreatedAt.Format(time.RFC3339Nano))
		if err != nil {
			log.Printf("Failed to insert metric into telemetry_buffer: %v", err)
		}
	}

	if err := tx.Commit(); err != nil {
		log.Printf("Failed to commit telemetry_buffer batch: %v", err)
	}
}

// InterceptMetric intercepts the metric and writes it to the local SQLite buffer if in standalone mode.
// Returns true if intercepted (and thus shouldn't be emitted directly).
func InterceptMetric(name string, value float64, attributes map[string]string) bool {
	if !telemetryEnabled {
		return true // Telemetry disabled, don't emit
	}

	if isStandalone {
		if metricChan != nil {
			select {
			case metricChan <- metricEvent{Name: name, Value: value, Attributes: attributes, CreatedAt: time.Now()}:
			default:
				log.Printf("Telemetry buffer full, dropping metric: %s", name)
			}
		}
		return true // Handled by standalone buffer
	}

	return false // Emit normally
}

// ResetForTest resets the telemetry state for unit testing.
func ResetForTest() {
	initOnce = sync.Once{}
	telemetryDB = nil
	isStandalone = false
	telemetryEnabled = false
	if stopFlushChan != nil {
		close(stopFlushChan)
		stopFlushChan = nil
	}
	if metricChan != nil {
		close(metricChan)
		metricChan = nil
	}
}
