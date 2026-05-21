package db

import (
    "context"
    "database/sql"
    "time"

    "github.com/google/uuid"
    "github.com/pgvector/pgvector-go"
)

type AutoDreamFinding struct {
    ID        uuid.UUID
    Timestamp time.Time
    Content   string
    Embedding pgvector.Vector
}

type AutoDreamStore struct {
    db *sql.DB
}

func NewAutoDreamStore(db *sql.DB) *AutoDreamStore {
    return &AutoDreamStore{db: db}
}

func (s *AutoDreamStore) UpsertFinding(ctx context.Context, finding AutoDreamFinding) error {
    query := `INSERT INTO autodream_findings (id, timestamp, content, embedding) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO UPDATE SET timestamp = EXCLUDED.timestamp, content = EXCLUDED.content, embedding = EXCLUDED.embedding`
    _, err := s.db.ExecContext(ctx, query, finding.ID, finding.Timestamp, finding.Content, finding.Embedding)
    return err
}

func (s *AutoDreamStore) QuerySimilarFindings(ctx context.Context, embedding pgvector.Vector, limit int) ([]AutoDreamFinding, error) {
    query := `SELECT id, timestamp, content, embedding FROM autodream_findings ORDER BY embedding <=> $1 LIMIT $2`
    rows, err := s.db.QueryContext(ctx, query, embedding, limit)
    if err != nil {
        return nil, err
    }
    defer rows.Close()

    var findings []AutoDreamFinding
    for rows.Next() {
        var finding AutoDreamFinding
        if err := rows.Scan(&finding.ID, &finding.Timestamp, &finding.Content, &finding.Embedding); err != nil {
            return nil, err
        }
        findings = append(findings, finding)
    }

    if err := rows.Err(); err != nil {
        return nil, err
    }

    return findings, nil
}
