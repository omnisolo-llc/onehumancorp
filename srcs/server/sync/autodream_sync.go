package sync

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"log"
	"net/http"
	"os"
	"strings"
	"time"

	"onehumancorp/srcs/server/telemetry"
)

type AutoDreamSyncEngine struct {
	db       *sql.DB
	isSQLite bool
}

func NewAutoDreamSyncEngine(db *sql.DB) *AutoDreamSyncEngine {
	dbURL := os.Getenv("DATABASE_URL")
	if dbURL == "" {
		dbURL = "sqlite://file::memory:?mode=memory"
	}
	return &AutoDreamSyncEngine{
		db:       db,
		isSQLite: strings.HasPrefix(dbURL, "sqlite"),
	}
}

type AutoDream struct {
	ID        string `json:"id"`
	Content   string `json:"content"`
	Embedding []byte `json:"embedding,omitempty"`
}

func (e *AutoDreamSyncEngine) ProcessForecastTick(ctx context.Context) error {
	if !e.isSQLite {
		return nil
	}

	rows, err := e.db.QueryContext(ctx, "SELECT id, content, embedding FROM embedding_cache WHERE synced_to_cloud = false LIMIT 100")
	if err != nil {
		telemetry.SyncFailedCount.Inc()
		return err
	}
	defer rows.Close()

	var records []AutoDream
	var ids []string
	for rows.Next() {
		var id string
		var content string
		var embedding []byte
		if err := rows.Scan(&id, &content, &embedding); err != nil {
			log.Printf("failed to scan row: %v", err)
			continue
		}
		records = append(records, AutoDream{
			ID:        id,
			Content:   content,
			Embedding: embedding,
		})
		ids = append(ids, id)
	}

	if len(records) == 0 {
		return nil
	}

	payload, err := json.Marshal(records)
	if err != nil {
		telemetry.SyncFailedCount.Inc()
		return err
	}

	cloudURL := os.Getenv("OHC_CORE_URL")
	if cloudURL == "" {
		cloudURL = "http://localhost:8080"
	}

	req, err := http.NewRequestWithContext(ctx, "POST", cloudURL+"/api/v1/sync/autodream", bytes.NewBuffer(payload))
	if err != nil {
		telemetry.SyncFailedCount.Inc()
		return err
	}
	req.Header.Set("Content-Type", "application/json")

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		telemetry.SyncFailedCount.Inc()
		log.Printf("failed to sync to cloud: %v", err)
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		telemetry.SyncFailedCount.Inc()
		log.Printf("failed to sync to cloud, status code: %d", resp.StatusCode)
		return nil
	}

	for _, id := range ids {
		_, err := e.db.ExecContext(ctx, "UPDATE embedding_cache SET synced_to_cloud = true WHERE id = ?", id)
		if err != nil {
			telemetry.SyncFailedCount.Inc()
			log.Printf("failed to update record %s: %v", id, err)
			continue
		}
		telemetry.SyncCompletedCount.Inc()
	}

	return nil
}

func (e *AutoDreamSyncEngine) Start(ctx context.Context, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			if err := e.ProcessForecastTick(ctx); err != nil {
				log.Printf("error processing forecast tick: %v", err)
			}
		}
	}
}
