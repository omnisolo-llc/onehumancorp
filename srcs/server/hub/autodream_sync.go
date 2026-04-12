package hub

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type AutoDreamSyncRecord struct {
	ID         string
	Content    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type AutoDreamSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]AutoDreamSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []AutoDreamSyncRecord) error
}

type autoDreamSyncService struct {
	provider db.Provider
}

func NewAutoDreamSyncService(provider db.Provider) AutoDreamSyncService {
	return &autoDreamSyncService{provider: provider}
}

func (s *autoDreamSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]AutoDreamSyncRecord, error) {
	query := ` + "`" + `SELECT id, content, sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1` + "`" + `
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		telemetry.RecordAutoDreamSyncError(ctx)
		return nil, err
	}
	defer rows.Close()

	var records []AutoDreamSyncRecord
	for rows.Next() {
		var r AutoDreamSyncRecord
		if err := rows.Scan(&r.ID, &r.Content, &r.SyncStatus); err != nil {
			telemetry.RecordAutoDreamSyncError(ctx)
			return nil, err
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (s *autoDreamSyncService) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	// For testing compatibility, updating one by one or string building. Here using simple loop since it's an interface impl.
	for _, id := range ids {
		query := ` + "`" + `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1` + "`" + `
		_, err := s.provider.Exec(ctx, query, id)
		if err != nil {
			telemetry.RecordAutoDreamSyncError(ctx)
			return err
		}
		telemetry.RecordAutoDreamRecordSynced(ctx)
	}
	return nil
}

func (s *autoDreamSyncService) ProcessIncomingSync(ctx context.Context, records []AutoDreamSyncRecord) error {
	for _, r := range records {
		query := ` + "`" + `INSERT INTO autodream_memories (id, content, sync_status) VALUES ($1, $2, 'synced')
		ON CONFLICT(id) DO UPDATE SET content = $2, sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP` + "`" + `
		_, err := s.provider.Exec(ctx, query, r.ID, r.Content)
		if err != nil {
			telemetry.RecordAutoDreamSyncError(ctx)
			return err
		}
		telemetry.RecordAutoDreamRecordSynced(ctx)
	}
	return nil
}
