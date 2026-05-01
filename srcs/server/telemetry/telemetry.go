package telemetry

// Metrics declarations for AutoDream Sync
var (
	// AutodreamRecordsSyncedTotal tracks the number of AutoDream memory records successfully synchronized to the Cloud
	AutodreamRecordsSyncedTotal = "autodream_records_synced_total"

	// AutodreamSyncErrorsTotal tracks the number of errors encountered during AutoDream synchronization
	AutodreamSyncErrorsTotal = "autodream_sync_errors_total"
)
