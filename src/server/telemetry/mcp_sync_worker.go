package telemetry

import (
	"context"
	"log"
	"time"
)

type Provider interface {
    GetPendingMetrics() []string
    MarkSynced(id string) error
}

type McpSyncWorker struct {
	DB Provider
}

func (m *McpSyncWorker) Start(ctx context.Context) {
	log.Println("Starting McpSyncWorker...")
	go func() {
		ticker := time.NewTicker(5 * time.Second)
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				log.Println("McpSyncWorker stopping...")
				return
			case <-ticker.C:
				m.syncMetrics()
			}
		}
	}()
}

func (m *McpSyncWorker) syncMetrics() {
    if m.DB == nil {
        return
    }

    pending := m.DB.GetPendingMetrics()
    if len(pending) > 0 {
	    log.Printf("Simulating MCP upload for %d pending metrics...\n", len(pending))
        for _, id := range pending {
            m.DB.MarkSynced(id)
        }
    }
}
