package db

import (
    "context"
    "testing"
    "time"

    "github.com/DATA-DOG/go-sqlmock"
    "github.com/google/uuid"
    "github.com/pgvector/pgvector-go"
)

func TestUpsertFinding(t *testing.T) {
    db, mock, err := sqlmock.New()
    if err != nil {
        t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
    }
    defer db.Close()

    store := NewAutoDreamStore(db)

    id := uuid.New()
    now := time.Now()
    content := "test content"
    embedding := pgvector.NewVector([]float32{1.0, 2.0, 3.0})

    finding := AutoDreamFinding{
        ID:        id,
        Timestamp: now,
        Content:   content,
        Embedding: embedding,
    }

    mock.ExpectExec(`INSERT INTO autodream_findings \(id, timestamp, content, embedding\) VALUES \(\$1, \$2, \$3, \$4\) ON CONFLICT \(id\) DO UPDATE SET timestamp = EXCLUDED\.timestamp, content = EXCLUDED\.content, embedding = EXCLUDED\.embedding`).
        WithArgs(id, now, content, embedding).
        WillReturnResult(sqlmock.NewResult(1, 1))

    err = store.UpsertFinding(context.Background(), finding)
    if err != nil {
        t.Errorf("error was not expected while upserting finding: %s", err)
    }

    if err := mock.ExpectationsWereMet(); err != nil {
        t.Errorf("there were unfulfilled expectations: %s", err)
    }
}

func TestQuerySimilarFindings(t *testing.T) {
    db, mock, err := sqlmock.New()
    if err != nil {
        t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
    }
    defer db.Close()

    store := NewAutoDreamStore(db)

    embedding := pgvector.NewVector([]float32{1.0, 2.0, 3.0})
    limit := 5

    rows := sqlmock.NewRows([]string{"id", "timestamp", "content", "embedding"}).
        AddRow(uuid.New(), time.Now(), "content 1", "[1,2,3]").
        AddRow(uuid.New(), time.Now(), "content 2", "[1,2,3]")

    mock.ExpectQuery(`SELECT id, timestamp, content, embedding FROM autodream_findings ORDER BY embedding <=> \$1 LIMIT \$2`).
        WithArgs(embedding, limit).
        WillReturnRows(rows)

    findings, err := store.QuerySimilarFindings(context.Background(), embedding, limit)
    if err != nil {
        t.Errorf("error was not expected while querying findings: %s", err)
    }

    if len(findings) != 2 {
        t.Errorf("expected 2 findings, got %d", len(findings))
    }

    if err := mock.ExpectationsWereMet(); err != nil {
        t.Errorf("there were unfulfilled expectations: %s", err)
    }
}
