package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"go.opentelemetry.io/otel"
	"time"
)

type SyncStatus string

const (
	SyncStatusPending SyncStatus = "pending"
	SyncStatusSynced  SyncStatus = "synced"
	SyncStatusError   SyncStatus = "error"
)

type RAGSyncRecord struct {
	ID         string
	OrgID      string
	Context    string
	Vector     []float32
	SyncStatus SyncStatus
	LastSyncAt time.Time
}

type RAGSyncService interface {
	FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error)
	MarkSynced(ctx context.Context, ids []string) error
	ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error
}

type RAGSyncProvider struct {
	db *sql.DB
}

func NewRAGSyncProvider(db *sql.DB) *RAGSyncProvider {
	return &RAGSyncProvider{db: db}
}

var (
	meter = otel.Meter("github.com/onehumancorp/mono/srcs/server/hub")

	recordsSyncedTotal, _ = meter.Int64Counter("rag_records_synced_total")
	syncErrorsTotal, _    = meter.Int64Counter("rag_sync_errors_total")
)

func (p *RAGSyncProvider) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := "SELECT id, organization_id, content, embedding, sync_status FROM consolidated_memory WHERE sync_status = 'pending' LIMIT $1"
	if false { // in production we will inject provider dialect to use: query += " FOR UPDATE SKIP LOCKED"
	}
	rows, err := p.db.QueryContext(ctx, query, limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vectorString sql.NullString
		if err := rows.Scan(&r.ID, &r.OrgID, &r.Context, &vectorString, &r.SyncStatus); err != nil {
			return nil, err
		}
		if vectorString.Valid && vectorString.String != "" {
			json.Unmarshal([]byte(vectorString.String), &r.Vector)
		}
		records = append(records, r)
	}
	return records, rows.Err()
}

func (p *RAGSyncProvider) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	tx, err := p.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	stmt, err := tx.PrepareContext(ctx, "UPDATE consolidated_memory SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP WHERE id = $1")
	if err != nil {
		return err
	}
	defer stmt.Close()

	for _, id := range ids {
		if _, err := stmt.ExecContext(ctx, id); err != nil {
			syncErrorsTotal.Add(ctx, 1)
			return err
		}
		recordsSyncedTotal.Add(ctx, 1)
	}

	return tx.Commit()
}

func (p *RAGSyncProvider) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	tx, err := p.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	for _, r := range records {
		var vectorParam interface{}
		if len(r.Vector) > 0 {
			vectorJSON, err := json.Marshal(r.Vector)
			if err != nil {
				return err
			}
			vectorParam = string(vectorJSON)
		} else {
			vectorParam = nil
		}

		res, err := tx.ExecContext(ctx, "UPDATE consolidated_memory SET content = $1, sync_status = $2, embedding = $3 WHERE id = $4", r.Context, r.SyncStatus, vectorParam, r.ID)
		if err != nil {
			syncErrorsTotal.Add(ctx, 1)
			return err
		}
		rowsAffected, err := res.RowsAffected()
		if err != nil {
			syncErrorsTotal.Add(ctx, 1)
			return err
		}
		if rowsAffected == 0 {
			_, err := tx.ExecContext(ctx, "INSERT INTO consolidated_memory (id, organization_id, content, source_type, sync_status, embedding, last_sync_at) VALUES ($1, $2, $3, 'sync', $4, $5, CURRENT_TIMESTAMP)", r.ID, r.OrgID, r.Context, r.SyncStatus, vectorParam)
			if err != nil {
				syncErrorsTotal.Add(ctx, 1)
				return err
			}
		}

		recordsSyncedTotal.Add(ctx, 1)
	}

	return tx.Commit()
}
