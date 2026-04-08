package hub

import (
	"context"
	"errors"
	"testing"
	"time"
)

// Mock Database Provider for testing DefaultRAGSyncService
type MockDB struct {
	queries []string
	rows    *MockRows
	result  *MockResult
	err     error
}

func (m *MockDB) QueryContext(ctx context.Context, query string, args ...any) (Rows, error) {
	m.queries = append(m.queries, query)
	if m.err != nil {
		return nil, m.err
	}
	return m.rows, nil
}

func (m *MockDB) ExecContext(ctx context.Context, query string, args ...any) (Result, error) {
	m.queries = append(m.queries, query)
	if m.err != nil {
		return nil, m.err
	}
	return m.result, nil
}

type MockRows struct {
	data [][]any
	idx  int
	err  error
}

func (m *MockRows) Next() bool {
	if m.idx < len(m.data) {
		return true
	}
	return false
}

func (m *MockRows) Scan(dest ...any) error {
	if m.idx >= len(m.data) {
		return errors.New("no more rows")
	}
	row := m.data[m.idx]
	for i, d := range dest {
		switch ptr := d.(type) {
		case *string:
			if val, ok := row[i].(string); ok {
				*ptr = val
			}
		case **string:
			if val, ok := row[i].(*string); ok {
				*ptr = val
			}
		case *SyncStatus:
			if val, ok := row[i].(SyncStatus); ok {
				*ptr = val
			}
		case **time.Time:
			if val, ok := row[i].(*time.Time); ok {
				*ptr = val
			}
		}
	}
	m.idx++
	return nil
}

func (m *MockRows) Close() error { return nil }
func (m *MockRows) Err() error   { return m.err }

type MockResult struct {
	affected int64
}

func (m *MockResult) LastInsertId() (int64, error) { return 0, nil }
func (m *MockResult) RowsAffected() (int64, error) { return m.affected, nil }


type MockRAGSyncService struct {
	records []RAGSyncRecord
}

func (m *MockRAGSyncService) FetchPendingSyncs(ctx context.Context, limit int) ([]RAGSyncRecord, error) {
	var pending []RAGSyncRecord
	for _, record := range m.records {
		if record.SyncStatus == SyncStatusPending {
			pending = append(pending, record)
		}
		if len(pending) >= limit {
			break
		}
	}
	return pending, nil
}

func (m *MockRAGSyncService) MarkSynced(ctx context.Context, ids []string) error {
	for i, record := range m.records {
		for _, id := range ids {
			if record.ID == id {
				m.records[i].SyncStatus = SyncStatusSynced
				m.records[i].LastSyncAt = time.Now()
			}
		}
	}
	return nil
}

func (m *MockRAGSyncService) ProcessIncomingSync(ctx context.Context, records []RAGSyncRecord) error {
	for _, record := range records {
		if record.Context == "error" {
			return errors.New("simulated error processing record")
		}
		// simulating saving to DB
		record.SyncStatus = SyncStatusSynced
		m.records = append(m.records, record)
	}
	return nil
}

func TestRAGSyncService_Mock(t *testing.T) {
	mockService := &MockRAGSyncService{
		records: []RAGSyncRecord{
			{ID: "1", Context: "test context 1", SyncStatus: SyncStatusPending},
			{ID: "2", Context: "test context 2", SyncStatus: SyncStatusPending},
		},
	}

	ctx := context.Background()

	// Test FetchPendingSyncs
	pending, err := mockService.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error fetching pending syncs: %v", err)
	}

	if len(pending) != 2 {
		t.Errorf("expected 2 pending records, got %d", len(pending))
	}
}

func TestDefaultRAGSyncService_FetchPendingSyncs(t *testing.T) {
	ctx := context.Background()
	now := time.Now()
	vectorStr := "[0.1, 0.2]"

	mockRows := &MockRows{
		data: [][]any{
			{"1", "context 1", &vectorStr, SyncStatusPending, &now},
			{"2", "context 2", &vectorStr, SyncStatusPending, &now},
		},
	}
	mockDB := &MockDB{
		rows: mockRows,
	}

	service := NewDefaultRAGSyncService(mockDB)

	records, err := service.FetchPendingSyncs(ctx, 10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(records) != 2 {
		t.Errorf("expected 2 records, got %d", len(records))
	}
	if records[0].ID != "1" || records[1].ID != "2" {
		t.Errorf("unexpected record IDs: %v", records)
	}
}

func TestDefaultRAGSyncService_ProcessIncomingSync(t *testing.T) {
	ctx := context.Background()

	mockDB := &MockDB{
		result: &MockResult{affected: 0}, // simulate insert required
	}

	service := NewDefaultRAGSyncService(mockDB)

	records := []RAGSyncRecord{
		{ID: "1", Context: "new context"},
	}

	err := service.ProcessIncomingSync(ctx, records)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(mockDB.queries) != 2 {
		t.Errorf("expected 2 queries (update then insert), got %d", len(mockDB.queries))
	}
}
