package hub

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type autoDreamSyncService struct {
	provider db.Provider
}

func NewAutoDreamSyncService(provider db.Provider) AutoDreamSyncService {
	return &autoDreamSyncService{provider: provider}
}

func (s *autoDreamSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]AutoDreamSyncRecord, error) {
	query := `SELECT id, content, sync_status FROM autodream_memories WHERE sync_status = 'pending' LIMIT $1`
	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []AutoDreamSyncRecord
	for rows.Next() {
		var r AutoDreamSyncRecord
		if err := rows.Scan(&r.ID, &r.Context, &r.SyncStatus); err != nil {
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
	for _, id := range ids {
		_, err := s.provider.Exec(ctx, "UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = $1 WHERE id = $2", time.Now(), id)
		if err != nil {
			return err
		}
	}
	return nil
}

func (s *autoDreamSyncService) ProcessIncomingSync(ctx context.Context, records []AutoDreamSyncRecord) error {
	for _, r := range records {
		var count int
		row := s.provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE id = $1", r.ID)
		if err := row.Scan(&count); err != nil {
			return err
		}

		if count > 0 {
			_, err := s.provider.Exec(ctx, "UPDATE autodream_memories SET content = $1, sync_status = $2, last_sync_at = $3 WHERE id = $4", r.Context, r.SyncStatus, r.LastSyncAt, r.ID)
			if err != nil {
				return err
			}
		} else {
			_, err := s.provider.Exec(ctx, "INSERT INTO autodream_memories (id, content, sync_status, last_sync_at) VALUES ($1, $2, $3, $4)", r.ID, r.Context, r.SyncStatus, r.LastSyncAt)
			if err != nil {
				return err
			}
		}
	}
	return nil
}
