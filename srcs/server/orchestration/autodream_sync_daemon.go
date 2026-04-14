package orchestration

import (
    "context"
    "time"
)

type AutoDreamSyncDaemon struct {
    worker *AutoDreamWorker
    ticker *time.Ticker
    quit   chan struct{}
}

func NewAutoDreamSyncDaemon(worker *AutoDreamWorker, pollInterval time.Duration) *AutoDreamSyncDaemon {
    return &AutoDreamSyncDaemon{
        worker: worker,
        ticker: time.NewTicker(pollInterval),
        quit:   make(chan struct{}),
    }
}

func (d *AutoDreamSyncDaemon) Start(ctx context.Context) {
    go func() {
        for {
            select {
            case <-d.quit:
                d.ticker.Stop()
                return
            case <-ctx.Done():
                d.ticker.Stop()
                return
            case <-d.ticker.C:
                _ = d.worker.ProcessMemories(ctx)
            }
        }
    }()
}

func (d *AutoDreamSyncDaemon) Stop() {
    close(d.quit)
}
