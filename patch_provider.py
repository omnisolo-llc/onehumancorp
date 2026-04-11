with open("srcs/server/db/provider.go", "r") as f:
    content = f.read()

content = content.replace(
"""	IsSQLite() bool
	AcquireTask(ctx context.Context, agentID string) (*TaskRecord, error)
}""",
"""	IsSQLite() bool
	AcquireTask(ctx context.Context, agentID string) (*TaskRecord, error)
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type RAGSyncStatus string

const (
	SyncStatusPending RAGSyncStatus = "pending"
	SyncStatusSynced  RAGSyncStatus = "synced"
	SyncStatusError   RAGSyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	Context    string
	Vector     []byte
	SyncStatus RAGSyncStatus
	LastSyncAt time.Time
}""")

with open("srcs/server/db/provider.go", "w") as f:
    f.write(content)
