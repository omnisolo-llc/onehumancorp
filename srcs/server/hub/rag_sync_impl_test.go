package hub

import (
	"context"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
)

type mockProvider struct {
	db.Provider
	execFunc     func(ctx context.Context, sql string, arguments ...any) (int64, error)
	queryFunc    func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error)
	queryRowFunc func(ctx context.Context, sql string, optionsAndArgs ...any) db.Row
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.execFunc != nil {
		return m.execFunc(ctx, sql, arguments...)
	}
	return 0, nil
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if m.queryFunc != nil {
		return m.queryFunc(ctx, sql, optionsAndArgs...)
	}
	return nil, nil
}

func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	if m.queryRowFunc != nil {
		return m.queryRowFunc(ctx, sql, optionsAndArgs...)
	}
	return nil
}

type mockRows struct {
	db.Rows
	nextFunc func() bool
	scanFunc func(dest ...any) error
	errFunc  func() error
}

func (m *mockRows) Next() bool {
	if m.nextFunc != nil {
		return m.nextFunc()
	}
	return false
}

func (m *mockRows) Scan(dest ...any) error {
	if m.scanFunc != nil {
		return m.scanFunc(dest...)
	}
	return nil
}

func (m *mockRows) Err() error {
	if m.errFunc != nil {
		return m.errFunc()
	}
	return nil
}

func (m *mockRows) Close() {}
func (m *mockRows) Columns() ([]string, error) { return nil, nil }

type mockRow struct {
	db.Row
	scanFunc func(dest ...any) error
}

func (m *mockRow) Scan(dest ...any) error {
	if m.scanFunc != nil {
		return m.scanFunc(dest...)
	}
	return nil
}

func TestFetchPendingSyncsImpl(t *testing.T) {
	ctx := context.Background()

	// 1. Success
	rows := &mockRows{
		nextFunc: func() bool {
			// A simple toggle
			return false
		},
	}
	mockProv := &mockProvider{
		queryFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
			return rows, nil
		},
	}
	svc := NewRAGSyncService(mockProv)
	records, err := svc.FetchPendingSyncs(ctx, 10)
	assert.NoError(t, err)
	assert.Empty(t, records)

	// 2. Query Error
	mockProv = &mockProvider{
		queryFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
			return nil, errors.New("db error")
		},
	}
	svc = NewRAGSyncService(mockProv)
	records, err = svc.FetchPendingSyncs(ctx, 10)
	assert.Error(t, err)
	assert.Nil(t, records)
}

func TestMarkSyncedImpl(t *testing.T) {
	ctx := context.Background()

	// 1. Success
	mockProv := &mockProvider{
		execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
			return 1, nil
		},
	}
	svc := NewRAGSyncService(mockProv)
	err := svc.MarkSynced(ctx, []string{"id1"})
	assert.NoError(t, err)

	// 2. Exec Error
	mockProv = &mockProvider{
		execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
			return 0, errors.New("exec error")
		},
	}
	svc = NewRAGSyncService(mockProv)
	err = svc.MarkSynced(ctx, []string{"id1"})
	assert.Error(t, err)
}

func TestProcessIncomingSyncImpl(t *testing.T) {
	ctx := context.Background()

	// 1. Success Update
	mockProv := &mockProvider{
		queryRowFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
			return &mockRow{
				scanFunc: func(dest ...any) error {
					*dest[0].(*int) = 1 // exists
					return nil
				},
			}
		},
		execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
			return 1, nil
		},
	}
	svc := NewRAGSyncService(mockProv)
	err := svc.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "id1", Context: "test"}})
	assert.NoError(t, err)

	// 2. Success Insert
	mockProv = &mockProvider{
		queryRowFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
			return &mockRow{
				scanFunc: func(dest ...any) error {
					*dest[0].(*int) = 0 // does not exist
					return nil
				},
			}
		},
		execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
			return 1, nil
		},
	}
	svc = NewRAGSyncService(mockProv)
	err = svc.ProcessIncomingSync(ctx, []RAGSyncRecord{{ID: "id2", Context: "test"}})
	assert.NoError(t, err)
}
