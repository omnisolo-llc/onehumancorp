package hybrid_sync

import (
	"bytes"
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/hub"
)

type RAGSyncDaemon struct {
	svc          hub.RAGSyncService
	cloudBaseURL string
	httpClient   *http.Client
	pollInterval time.Duration
}

func NewRAGSyncDaemon(svc hub.RAGSyncService, cloudBaseURL string, pollInterval time.Duration) *RAGSyncDaemon {
	return &RAGSyncDaemon{
		svc:          svc,
		cloudBaseURL: cloudBaseURL,
		httpClient:   &http.Client{Timeout: 10 * time.Second},
		pollInterval: pollInterval,
	}
}

func (d *RAGSyncDaemon) Start(ctx context.Context) {
	ticker := time.NewTicker(d.pollInterval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			d.sync(ctx)
		}
	}
}

func (d *RAGSyncDaemon) sync(ctx context.Context) {
	records, err := d.svc.FetchPendingSyncs(ctx, 100)
	if err != nil {
		slog.Error("Failed to fetch pending RAG syncs", "error", err)
		return
	}

	if len(records) == 0 {
		return
	}

	payload := struct {
		Records []hub.RAGSyncRecord `json:"records"`
	}{
		Records: records,
	}

	body, err := json.Marshal(payload)
	if err != nil {
		slog.Error("Failed to marshal RAG sync payload", "error", err)
		return
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, d.cloudBaseURL+"/api/sync/rag", bytes.NewReader(body))
	if err != nil {
		slog.Error("Failed to create RAG sync request", "error", err)
		return
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := d.httpClient.Do(req)
	if err != nil {
		slog.Error("Failed to send RAG sync request", "error", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusOK {
		var ids []string
		for _, r := range records {
			ids = append(ids, r.ID)
		}
		if err := d.svc.MarkSynced(ctx, ids); err != nil {
			slog.Error("Failed to mark RAG records as synced", "error", err)
		}
	} else {
		slog.Error("Cloud rejected RAG sync", "status", resp.StatusCode)
	}
}
