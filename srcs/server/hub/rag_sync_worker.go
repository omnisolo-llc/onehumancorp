package hub

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type RAGSyncWorker struct {
	svc          RAGSyncService
	cloudBaseURL string
	client       *http.Client
	apiKey       string
}

func NewRAGSyncWorker(svc RAGSyncService, cloudBaseURL, apiKey string) *RAGSyncWorker {
	return &RAGSyncWorker{
		svc:          svc,
		cloudBaseURL: cloudBaseURL,
		apiKey:       apiKey,
		client:       &http.Client{Timeout: 30 * time.Second},
	}
}

func (w *RAGSyncWorker) Run(ctx context.Context, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.syncPendingRecords(ctx)
		}
	}
}

func (w *RAGSyncWorker) syncPendingRecords(ctx context.Context) {
	records, err := w.svc.FetchPendingSyncs(ctx, 100)
	if err != nil {
		log.Printf("RAGSyncWorker failed to fetch pending syncs: %v", err)
		return
	}

	if len(records) == 0 {
		return
	}

	payload, err := json.Marshal(records)
	if err != nil {
		log.Printf("RAGSyncWorker failed to marshal records: %v", err)
		return
	}

	req, err := http.NewRequestWithContext(ctx, "POST", w.cloudBaseURL+"/api/v1/sync/rag", bytes.NewReader(payload))
	if err != nil {
		log.Printf("RAGSyncWorker failed to create request: %v", err)
		return
	}

	req.Header.Set("Content-Type", "application/json")
	if w.apiKey != "" {
		req.Header.Set("Authorization", "Bearer "+w.apiKey)
	}

	resp, err := w.client.Do(req)
	if err != nil {
		log.Printf("RAGSyncWorker failed to execute request: %v", err)
		telemetry.RecordRAGSyncError(ctx, len(records))
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		log.Printf("RAGSyncWorker received error status %d: %s", resp.StatusCode, string(body))
		telemetry.RecordRAGSyncError(ctx, len(records))
		return
	}

	var ids []string
	for _, r := range records {
		ids = append(ids, r.ID)
	}

	err = w.svc.MarkSynced(ctx, ids)
	if err != nil {
		log.Printf("RAGSyncWorker failed to mark records as synced: %v", err)
	}
}

type RAGSyncHandler struct {
	svc RAGSyncService
}

func NewRAGSyncHandler(svc RAGSyncService) *RAGSyncHandler {
	return &RAGSyncHandler{svc: svc}
}

func (h *RAGSyncHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	if r.Method != "POST" {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var records []RAGSyncRecord
	if err := json.NewDecoder(r.Body).Decode(&records); err != nil {
		http.Error(w, fmt.Sprintf("failed to parse payload: %v", err), http.StatusBadRequest)
		return
	}

	if err := h.svc.ProcessIncomingSync(r.Context(), records); err != nil {
		http.Error(w, fmt.Sprintf("failed to process incoming sync: %v", err), http.StatusInternalServerError)
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(`{"status": "ok"}`))
}
