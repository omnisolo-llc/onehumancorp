package statesyncmcp

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockSQLiteProvider struct {
	db.Provider
}

func (m *mockSQLiteProvider) IsSQLite() bool {
	return true
}

func (m *mockSQLiteProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return &mockRow{val: 5}
}

func (m *mockSQLiteProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &mockRows{count: 2}, nil
}

func (m *mockSQLiteProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 1, nil
}

type mockRow struct {
	val int
}

func (r *mockRow) Scan(dest ...any) error {
	if len(dest) > 0 {
		if ptr, ok := dest[0].(*int); ok {
			*ptr = r.val
		}
	}
	return nil
}

type mockRows struct {
	count int
	idx   int
}

func (r *mockRows) Next() bool {
	if r.idx < r.count {
		r.idx++
		return true
	}
	return false
}

func (r *mockRows) Scan(dest ...any) error {
	if len(dest) > 0 {
		if idPtr, ok := dest[0].(*string); ok {
			*idPtr = "test-id"
		}
		if statusPtr, ok := dest[1].(*string); ok {
			*statusPtr = "done"
		}
		if payloadPtr, ok := dest[2].(*string); ok {
			*payloadPtr = `{"key":"value"}`
		}
	}
	return nil
}

func (r *mockRows) Close() {}

func (r *mockRows) Columns() ([]string, error) { return nil, nil }
func (r *mockRows) Err() error { return nil }

func TestDBStateSyncProvider_GetStatus(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost:8080")

	claims := &auth.Claims{OrganizationID: "test-org"}
	res, err := provider.GetStatus(context.Background(), claims)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if res["pending_sync_up"] != 5 {
		t.Errorf("expected pending_sync_up 5, got %v", res["pending_sync_up"])
	}
}

func TestDBStateSyncProvider_SyncDown(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("X-Tenant-ID") != "test-org" {
			t.Errorf("expected X-Tenant-ID header to be test-org")
		}
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"items_synced": 3}`))
	}))
	defer server.Close()

	dbWrapper := &db.DB{Provider: &mockSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, server.URL)

	claims := &auth.Claims{OrganizationID: "test-org"}
	res, err := provider.SyncDown(context.Background(), claims)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if res["status"] != "success" {
		t.Errorf("expected status 'success', got %v", res["status"])
	}
}

func TestDBStateSyncProvider_SyncUp(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status": "ok"}`))
	}))
	defer server.Close()

	dbWrapper := &db.DB{Provider: &mockSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, server.URL)

	claims := &auth.Claims{OrganizationID: "test-org"}
	res, err := provider.SyncUp(context.Background(), claims)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if res["status"] != "success" {
		t.Errorf("expected status 'success', got %v", res["status"])
	}

	if res["synced_count"] != 2 {
		t.Errorf("expected synced_count 2, got %v", res["synced_count"])
	}
}

func TestDBStateSyncProvider_SyncUp_Empty(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockEmptySQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost:8080")

	claims := &auth.Claims{OrganizationID: "test-org"}
	res, err := provider.SyncUp(context.Background(), claims)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if res["synced_count"] != 0 {
		t.Errorf("expected synced_count 0, got %v", res["synced_count"])
	}
}

type mockEmptySQLiteProvider struct {
	db.Provider
}

func (m *mockEmptySQLiteProvider) IsSQLite() bool {
	return true
}

func (m *mockEmptySQLiteProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &mockRows{count: 0}, nil
}
