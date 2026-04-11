package hub

import (
    "context"
    "testing"
    "time"

    "github.com/stretchr/testify/assert"
    "github.com/stretchr/testify/require"
)

type mockRAGSyncService struct {
    records []RAGSyncRecord
}

func (m *mockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
    var pending []RAGSyncRecord
    for _, r := range m.records {
        if r.SyncStatus == SyncStatusPending {
            pending = append(pending, r)
        }
    }
    if len(pending) > limit {
        pending = pending[:limit]
    }
    return pending, nil
}

func (m *mockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
    idMap := make(map[string]bool)
    for _, id := range ids {
        idMap[id] = true
    }
    for i, r := range m.records {
        if idMap[r.ID] {
            m.records[i].SyncStatus = SyncStatusSynced
            m.records[i].LastSyncAt = time.Now()
        }
    }
    return nil
}

func (m *mockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
    m.records = append(m.records, records...)
    return nil
}

func TestRAGSyncServiceFlow(t *testing.T) {
    svc := &mockRAGSyncService{}

    record1 := RAGSyncRecord{ID: "1", Context: "test1", Vector: []float32{0.1, 0.2, 0.3}, SyncStatus: SyncStatusPending}
    svc.ProcessIncomingSync(context.Background(), []RAGSyncRecord{record1})

    pending, err := svc.FetchPendingSyncs(context.Background(), 10)
    require.NoError(t, err)
    require.Len(t, pending, 1)

    err = svc.MarkSynced(context.Background(), []string{"1"})
    require.NoError(t, err)

    pending, err = svc.FetchPendingSyncs(context.Background(), 10)
    require.NoError(t, err)
    require.Empty(t, pending)
}

func TestParseFormatPgVector(t *testing.T) {
    vec := []float32{1.5, 2.5, -3.0}
    str := formatPgVector(vec)
    assert.Equal(t, "[1.5,2.5,-3]", str)

    parsed, err := parsePgVector(str)
    require.NoError(t, err)
    assert.Equal(t, vec, parsed)

    emptyStr := formatPgVector(nil)
    assert.Equal(t, "[]", emptyStr)

    emptyParsed, err := parsePgVector(emptyStr)
    require.NoError(t, err)
    assert.Empty(t, emptyParsed)
}
