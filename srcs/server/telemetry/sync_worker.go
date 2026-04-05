package telemetry


// The SyncBufferedMetrics functionality is already implemented in srcs/server/orchestration/sip.go (SyncBufferedMetrics on SIPDB).
// We should set up a sync worker to periodically call it when OHC_STANDALONE is true.
