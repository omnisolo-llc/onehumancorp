package hub

import (
	"bytes"
	"context"
	"crypto/tls"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type SyncWorker struct {
	db             db.Provider
	service        RAGSyncService
	syncInterval   time.Duration
	syncBatchSize  int
	isStandalone   bool // Only push from local to cloud
	remoteEndpoint string
	httpClient     *http.Client
}

func NewSyncWorker(db db.Provider, service RAGSyncService, isStandalone bool, remoteEndpoint string) *SyncWorker {
	// Normally we would use mutual TLS via SPIFFE/SPIRE here as per instructions.
	// We're setting up a basic client that can be customized with mTLS later.
	tr := &http.Transport{
		TLSClientConfig: &tls.Config{
			// InsecureSkipVerify: true, // Only for dev/testing if needed
		},
	}
	client := &http.Client{
		Transport: tr,
		Timeout:   10 * time.Second,
	}

	return &SyncWorker{
		db:             db,
		service:        service,
		syncInterval:   5 * time.Minute,
		syncBatchSize:  50,
		isStandalone:   isStandalone,
		remoteEndpoint: remoteEndpoint,
		httpClient:     client,
	}
}

func (w *SyncWorker) Start(ctx context.Context) {
	if !w.isStandalone {
		// Only run sync daemon on standalone instances
		return
	}

	ticker := time.NewTicker(w.syncInterval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.runSync(ctx)
		}
	}
}

func (w *SyncWorker) runSync(ctx context.Context) {
	// Fetch pending records from local database
	records, err := w.service.FetchPendingSyncs(ctx, w.syncBatchSize)
	if err != nil {
		log.Printf("SyncWorker error fetching pending syncs: %v", err)
		return
	}

	if len(records) == 0 {
		return
	}

	log.Printf("SyncWorker: Pushing %d records to %s", len(records), w.remoteEndpoint)

	// Send to remote endpoint
	payload, err := json.Marshal(records)
	if err != nil {
		log.Printf("SyncWorker error marshaling payload: %v", err)
		return
	}

	req, err := http.NewRequestWithContext(ctx, "POST", w.remoteEndpoint, bytes.NewReader(payload))
	if err != nil {
		log.Printf("SyncWorker error creating request: %v", err)
		return
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := w.httpClient.Do(req)
	if err != nil {
		log.Printf("SyncWorker error sending sync request: %v", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		bodyBytes, _ := io.ReadAll(resp.Body)
		log.Printf("SyncWorker remote endpoint returned non-200 status: %d, body: %s", resp.StatusCode, string(bodyBytes))
		return
	}

	var idsToMark []string
	for _, r := range records {
		idsToMark = append(idsToMark, r.ID)
	}

	err = w.service.MarkSynced(ctx, idsToMark)
	if err != nil {
		log.Printf("SyncWorker error marking records synced: %v", err)
	} else {
		log.Printf("SyncWorker: Successfully synced %d records", len(records))
	}
}

// ProcessRemoteSync is the HTTP handler for the cloud API gateway to receive the sync
func (w *SyncWorker) ProcessRemoteSync(ctx context.Context, payload []byte) error {
	var records []RAGSyncRecord
	if err := json.Unmarshal(payload, &records); err != nil {
		return fmt.Errorf("failed to unmarshal payload: %w", err)
	}

	if err := w.service.ProcessIncomingSync(ctx, records); err != nil {
		return fmt.Errorf("failed to process incoming sync: %w", err)
	}

	return nil
}
