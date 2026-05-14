package hybrid_sync

import (
	"context"
	"database/sql"
	"time"
	"encoding/json"
	"net/http"
	"bytes"
	"fmt"
)

type BackgroundWorker struct {
	db       *sql.DB
	cloudURL string
	interval time.Duration
}

func NewBackgroundWorker(db *sql.DB, cloudURL string, interval time.Duration) *BackgroundWorker {
	return &BackgroundWorker{
		db:       db,
		cloudURL: cloudURL,
		interval: interval,
	}
}

type Delta struct {
	ID        string `json:"id"`
	EntityID  string `json:"entity_id"`
	Data      string `json:"data"`
	UpdatedAt string `json:"updated_at"`
}

func (w *BackgroundWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.syncDeltas(ctx)
		}
	}
}

func (w *BackgroundWorker) syncDeltas(ctx context.Context) error {
	rows, err := w.db.QueryContext(ctx, "SELECT id, entity_id, data, updated_at FROM crdt_deltas WHERE synced = false LIMIT 100")
	if err != nil {
		return err
	}
	defer rows.Close()

	var deltas []Delta
	for rows.Next() {
		var d Delta
		if err := rows.Scan(&d.ID, &d.EntityID, &d.Data, &d.UpdatedAt); err != nil {
			return err
		}
		deltas = append(deltas, d)
	}

	if len(deltas) == 0 {
		return nil
	}

	payload := map[string]interface{}{"deltas": deltas}
	body, err := json.Marshal(payload)
	if err != nil {
		return err
	}

	req, err := http.NewRequestWithContext(ctx, "POST", w.cloudURL+"/api/v1/sync/mcp-deltas", bytes.NewBuffer(body))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		return fmt.Errorf("failed to sync, status: %d", resp.StatusCode)
	}

	for _, d := range deltas {
		_, _ = w.db.ExecContext(ctx, "UPDATE crdt_deltas SET synced = true WHERE id = $1", d.ID)
	}

	return nil
}
