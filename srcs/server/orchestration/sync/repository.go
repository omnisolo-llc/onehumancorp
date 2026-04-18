package sync

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type SqliteLocalRepository struct {
	dbWrapper *db.DB
}

func NewSqliteLocalRepository(dbWrapper *db.DB) *SqliteLocalRepository {
	return &SqliteLocalRepository{dbWrapper: dbWrapper}
}

func (r *SqliteLocalRepository) GetPendingSync(ctx context.Context, limit int) ([]LocalMission, error) {
	query := fmt.Sprintf("SELECT id, status, payload, created_at, synced_to_cloud, cloud_mission_id, sync_error, last_synced_at FROM agent_missions WHERE synced_to_cloud = 0 AND (status = 'PENDING' OR status = 'BURSTING') LIMIT %d", limit)
	rows, err := r.dbWrapper.Provider.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var missions []LocalMission
	for rows.Next() {
		var m LocalMission
		var payloadStr string
		var cloudMissionID sql.NullString
		var syncError sql.NullString
		var lastSyncedAt sql.NullTime

		if err := rows.Scan(&m.ID, &m.Status, &payloadStr, &m.CreatedAt, &m.SyncedToCloud, &cloudMissionID, &syncError, &lastSyncedAt); err != nil {
			return nil, err
		}

		if cloudMissionID.Valid {
			m.CloudMissionID = cloudMissionID.String
		}
		if syncError.Valid {
			m.SyncError = syncError.String
		}
		if lastSyncedAt.Valid {
			m.LastSyncedAt = lastSyncedAt.Time
		}

		m.PayloadRaw = payloadStr

		missions = append(missions, m)
	}
	return missions, nil
}

func (r *SqliteLocalRepository) MarkSynced(ctx context.Context, localID string, cloudID string) error {
	query := "UPDATE agent_missions SET synced_to_cloud = 1, cloud_mission_id = $1, last_synced_at = CURRENT_TIMESTAMP WHERE id = $2"
	_, err := r.dbWrapper.Provider.Exec(ctx, query, cloudID, localID)
	return err
}

func (r *SqliteLocalRepository) MarkSyncError(ctx context.Context, localID string, syncError string) error {
	query := "UPDATE agent_missions SET sync_error = $1 WHERE id = $2"
	_, err := r.dbWrapper.Provider.Exec(ctx, query, syncError, localID)
	return err
}

func (r *SqliteLocalRepository) GetActiveEscalations(ctx context.Context) ([]LocalMission, error) {
	query := "SELECT id, status, payload, created_at, synced_to_cloud, cloud_mission_id, sync_error, last_synced_at FROM agent_missions WHERE synced_to_cloud = 1 AND cloud_mission_id IS NOT NULL AND status IN ('BURSTING', 'PENDING')"
	rows, err := r.dbWrapper.Provider.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var missions []LocalMission
	for rows.Next() {
		var m LocalMission
		var payloadStr string
		var cloudMissionID sql.NullString
		var syncError sql.NullString
		var lastSyncedAt sql.NullTime

		if err := rows.Scan(&m.ID, &m.Status, &payloadStr, &m.CreatedAt, &m.SyncedToCloud, &cloudMissionID, &syncError, &lastSyncedAt); err != nil {
			return nil, err
		}

		if cloudMissionID.Valid {
			m.CloudMissionID = cloudMissionID.String
		}
		if syncError.Valid {
			m.SyncError = syncError.String
		}
		if lastSyncedAt.Valid {
			m.LastSyncedAt = lastSyncedAt.Time
		}

		m.PayloadRaw = payloadStr

		missions = append(missions, m)
	}
	return missions, nil
}

func (r *SqliteLocalRepository) UpdateLocalStatus(ctx context.Context, localID string, newStatus string) error {
	query := "UPDATE agent_missions SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2"
	_, err := r.dbWrapper.Provider.Exec(ctx, query, newStatus, localID)
	return err
}
