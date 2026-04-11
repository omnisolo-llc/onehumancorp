package rag_sync

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"
)

type RAGSyncServiceImpl struct {
	db *sql.DB
}

func NewRAGSyncService(db *sql.DB) *RAGSyncServiceImpl {
	return &RAGSyncServiceImpl{
		db: db,
	}
}

func (s *RAGSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `SELECT id, content, embedding, sync_status, last_sync_at FROM autodream_memories WHERE sync_status = 'pending' LIMIT ?`
	rows, err := s.db.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var lastSync sql.NullTime
		var vectorStr sql.NullString
		err := rows.Scan(&r.ID, &r.Context, &vectorStr, &r.SyncStatus, &lastSync)
		if err != nil {
			return nil, fmt.Errorf("failed to scan record: %w", err)
		}
		if lastSync.Valid {
			r.LastSyncAt = lastSync.Time
		}
		if vectorStr.Valid && vectorStr.String != "" {
			err = json.Unmarshal([]byte(vectorStr.String), &r.Vector)
			if err != nil {
                // Log and ignore to keep processing
				fmt.Printf("failed to unmarshal vector for id %s: %v\n", r.ID, err)
			}
		}
		records = append(records, r)
	}
	if err = rows.Err(); err != nil {
		return nil, fmt.Errorf("row iteration error: %w", err)
	}
	return records, nil
}

func (s *RAGSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	query := `UPDATE autodream_memories SET sync_status = 'synced', last_sync_at = ? WHERE id = ?`
	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		return err
	}
	defer stmt.Close()

	now := time.Now().UTC()
	for _, id := range ids {
		_, err := stmt.ExecContext(ctx, now, id)
		if err != nil {
			ragSyncErrorsCounter.Add(ctx, 1)
			return err
		}
		ragRecordsSyncedCounter.Add(ctx, 1)
	}

	return tx.Commit()
}

func (s *RAGSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    if len(records) == 0 {
        return nil
    }

	tx, err := s.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

    // Postgres and SQLite conflict resolution logic differs, but this simple upsert logic works generally.
    // LWW (Last-Write-Wins) for Cloud Postgres DB
	query := `
        INSERT INTO autodream_memories (id, content, embedding, sync_status, last_sync_at)
        VALUES (?, ?, ?, 'synced', ?)
        ON CONFLICT(id) DO UPDATE SET
            content = excluded.content,
            embedding = excluded.embedding,
            sync_status = 'synced',
            last_sync_at = excluded.last_sync_at
    `
	stmt, err := tx.PrepareContext(ctx, query)
	if err != nil {
		return err
	}
	defer stmt.Close()

	for _, r := range records {
		var vectorStr string
		if len(r.Vector) > 0 {
			b, err := json.Marshal(r.Vector)
			if err == nil {
				vectorStr = string(b)
			}
		}
		_, err := stmt.ExecContext(ctx, r.ID, r.Context, vectorStr, r.LastSyncAt)
		if err != nil {
			return err
		}
	}

	return tx.Commit()
}
