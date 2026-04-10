package hub

import (
	"context"
	"database/sql"
	"encoding/binary"
	"fmt"
	"math"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type ragSyncServiceImpl struct {
	provider db.Provider
}

func NewRAGSyncService(provider db.Provider) RAGSyncService {
	return &ragSyncServiceImpl{provider: provider}
}

func (s *ragSyncServiceImpl) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	query := `
		SELECT id, content, embedding, sync_status, last_sync_at
		FROM agent_memories
		WHERE sync_status = 'pending'
		LIMIT $1
	`
	if s.provider.IsSQLite() {
		query = `
			SELECT id, content, embedding, sync_status, last_sync_at
			FROM agent_memories
			WHERE sync_status = 'pending'
			LIMIT ?
		`
	}

	rows, err := s.provider.Query(ctx, query, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to fetch pending syncs: %w", err)
	}
	defer rows.Close()

	var records []RAGSyncRecord
	for rows.Next() {
		var r RAGSyncRecord
		var embeddingBytes []byte
		var lastSyncAt sql.NullString

		if err := rows.Scan(&r.ID, &r.Context, &embeddingBytes, &r.SyncStatus, &lastSyncAt); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}

		if lastSyncAt.Valid {
			t, err := time.Parse(time.RFC3339Nano, lastSyncAt.String)
			if err == nil {
				r.LastSyncAt = t
			} else {
                t, err := time.Parse("2006-01-02 15:04:05.999999999-07:00", lastSyncAt.String)
                if err == nil {
                    r.LastSyncAt = t
                } else {
                    t, err := time.Parse("2006-01-02 15:04:05.999999999Z07:00", lastSyncAt.String)
                    if err == nil {
                        r.LastSyncAt = t
                    }
                }
            }
		}

		if len(embeddingBytes) > 0 {
			r.Vector = make([]float32, len(embeddingBytes)/4)
			for i := 0; i < len(embeddingBytes)/4; i++ {
				bits := binary.LittleEndian.Uint32(embeddingBytes[i*4 : (i+1)*4])
				r.Vector[i] = math.Float32frombits(bits)
			}
		}

		records = append(records, r)
	}

	return records, rows.Err()
}

func (s *ragSyncServiceImpl) MarkSynced(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	query := `
		UPDATE agent_memories
		SET sync_status = 'synced', last_sync_at = CURRENT_TIMESTAMP
		WHERE id IN (
	`
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		if i > 0 {
			query += ", "
		}
		if s.provider.IsSQLite() {
			query += "?"
		} else {
			query += fmt.Sprintf("$%d", i+1)
		}
		args[i] = id
	}
	query += ")"

	_, err := s.provider.Exec(ctx, query, args...)
	if err != nil {
		return fmt.Errorf("failed to mark synced: %w", err)
	}

	return nil
}

func (s *ragSyncServiceImpl) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, r := range records {
		var embeddingBytes []byte
		if len(r.Vector) > 0 {
			embeddingBytes = make([]byte, len(r.Vector)*4)
			for i, v := range r.Vector {
				bits := math.Float32bits(v)
				binary.LittleEndian.PutUint32(embeddingBytes[i*4:(i+1)*4], bits)
			}
		}

		query := `
			INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status, last_sync_at)
			VALUES ($1, 'system', $2, $3, 'synced', CURRENT_TIMESTAMP)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				sync_status = 'synced',
				last_sync_at = CURRENT_TIMESTAMP
		`
		if s.provider.IsSQLite() {
			query = `
				INSERT INTO agent_memories (id, organization_id, content, embedding, sync_status, last_sync_at)
				VALUES (?, 'system', ?, ?, 'synced', CURRENT_TIMESTAMP)
				ON CONFLICT (id) DO UPDATE SET
					content = excluded.content,
					embedding = excluded.embedding,
					sync_status = 'synced',
					last_sync_at = CURRENT_TIMESTAMP
			`
		}

		_, err := s.provider.Exec(ctx, query, r.ID, r.Context, embeddingBytes)
		if err != nil {
			return fmt.Errorf("failed to process incoming sync for id %s: %w", r.ID, err)
		}
	}
	return nil
}
