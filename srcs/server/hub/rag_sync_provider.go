package hub

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type RAGSyncProvider struct {
	db db.Provider
}

func NewRAGSyncProvider(database db.Provider) *RAGSyncProvider {
	return &RAGSyncProvider{
		db: database,
	}
}

func (p *RAGSyncProvider) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT memory_id, context, vector_embedding, sync_status, last_sync_at
		FROM swarm_memory_embeddings
		WHERE sync_status = $1
		LIMIT $2
	`
	rows, err := p.db.Query(ctx, query, SyncStatusPending, limit)
	if err != nil {
		ragSyncErrors.Add(ctx, 1)
		return nil, fmt.Errorf("querying pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var vectorBytes []byte
		var lastSync sql.NullString

		if err := rows.Scan(&r.ID, &r.Context, &vectorBytes, &r.SyncStatus, &lastSync); err != nil {
			ragSyncErrors.Add(ctx, 1)
			return nil, fmt.Errorf("scanning record: %w", err)
		}

		if lastSync.Valid {
			t, err := time.Parse(time.RFC3339Nano, lastSync.String)
			if err == nil {
				r.LastSyncAt = t
			} else {
				t, err = time.Parse("2006-01-02 15:04:05.999999-07:00", lastSync.String)
				if err == nil {
					r.LastSyncAt = t
				}
			}
		}

		if len(vectorBytes) > 0 {
			if err := json.Unmarshal(vectorBytes, &r.Vector); err != nil {
				ragSyncErrors.Add(ctx, 1)
				return nil, fmt.Errorf("unmarshaling vector: %w", err)
			}
		}
		records = append(records, r)
	}

	if err := rows.Err(); err != nil {
		ragSyncErrors.Add(ctx, 1)
		return nil, fmt.Errorf("rows error: %w", err)
	}

	return records, nil
}

func (p *RAGSyncProvider) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	for _, id := range ids {
		query := `
			UPDATE swarm_memory_embeddings
			SET sync_status = $1, last_sync_at = $2
			WHERE memory_id = $3
		`
		_, err := p.db.Exec(ctx, query, SyncStatusSynced, time.Now(), id)
		if err != nil {
			ragSyncErrors.Add(ctx, 1)
			return fmt.Errorf("updating sync status for id %s: %w", id, err)
		}
		ragRecordsSynced.Add(ctx, 1)
	}

	return nil
}

func (p *RAGSyncProvider) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	if len(records) == 0 {
		return nil
	}

	for _, r := range records {
		vectorBytes, err := json.Marshal(r.Vector)
		if err != nil {
			ragSyncErrors.Add(ctx, 1)
			return fmt.Errorf("marshaling vector for id %s: %w", r.ID, err)
		}

		// Use ON CONFLICT DO UPDATE to handle existing records
		query := `
			INSERT INTO swarm_memory_embeddings (memory_id, context, vector_embedding, sync_status, last_sync_at)
			VALUES ($1, $2, $3, $4, $5)
			ON CONFLICT (memory_id) DO UPDATE SET
				context = EXCLUDED.context,
				vector_embedding = EXCLUDED.vector_embedding,
				sync_status = EXCLUDED.sync_status,
				last_sync_at = EXCLUDED.last_sync_at
		`
		_, err = p.db.Exec(ctx, query, r.ID, r.Context, vectorBytes, r.SyncStatus, r.LastSyncAt)
		if err != nil {
			ragSyncErrors.Add(ctx, 1)
			return fmt.Errorf("upserting record for id %s: %w", r.ID, err)
		}
		ragRecordsSynced.Add(ctx, 1)
	}
	return nil
}
