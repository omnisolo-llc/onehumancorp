package statesyncmcp

import (
	"context"
	"errors"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
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

type mockErrorSQLiteProvider struct {
	db.Provider
}

func (m *mockErrorSQLiteProvider) IsSQLite() bool {
	return true
}

func (m *mockErrorSQLiteProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row {
	return &mockErrorRow{}
}

func (m *mockErrorSQLiteProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return nil, errors.New("query error")
}

func (m *mockErrorSQLiteProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 0, errors.New("exec error")
}

type mockErrorRow struct{}

func (r *mockErrorRow) Scan(dest ...any) error {
	return errors.New("scan error")
}

func TestDBStateSyncProvider_SendToCloud_EmptyURL(t *testing.T) {
	provider := NewDBStateSyncProvider(&db.DB{}, "")
	// Use an empty test to export the method via testing package trick or just invoke
	_, err := provider.sendToCloud(context.Background(), "/api", http.MethodGet, nil, nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestDBStateSyncProvider_CRDTPush(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost:8080")

	payload := map[string]interface{}{
		"id":         "delta1",
		"entity_id":  "e1",
		"data":       "testdata",
		"updated_at": "now",
	}

	claims := &auth.Claims{OrganizationID: "test-org"}
	res, err := provider.CRDTPush(context.Background(), payload, claims)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if res["status"] != "success" {
		t.Errorf("expected success status, got %v", res["status"])
	}
}

func TestDBStateSyncProvider_CRDTPush_MissingFields(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost:8080")

	payload := map[string]interface{}{
		"id": "delta1",
		// missing other fields
	}

	claims := &auth.Claims{OrganizationID: "test-org"}
	_, err := provider.CRDTPush(context.Background(), payload, claims)
	if err == nil {
		t.Fatal("expected error for missing fields")
	}
}

func TestDBStateSyncProvider_CRDTPush_NotSQLite(t *testing.T) {
	dbWrapper := &db.DB{Provider: &concreteMockNonSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost:8080")

	_, err := provider.CRDTPush(context.Background(), nil, nil)
	if err == nil {
		t.Fatal("expected error for not running in sqlite")
	}
}

func TestDBStateSyncProvider_CRDTPush_ExecError(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockErrorSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost:8080")

	payload := map[string]interface{}{
		"id":         "delta1",
		"entity_id":  "e1",
		"data":       "testdata",
		"updated_at": "now",
	}

	_, err := provider.CRDTPush(context.Background(), payload, nil)
	if err == nil {
		t.Fatal("expected error on exec")
	}
}

func TestDBStateSyncProvider_CRDTPull(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost:8080")

	claims := &auth.Claims{OrganizationID: "test-org"}
	res, err := provider.CRDTPull(context.Background(), "e1", claims)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if res["status"] != "success" {
		t.Errorf("expected success status, got %v", res["status"])
	}
}

func TestDBStateSyncProvider_CRDTPull_NotSQLite(t *testing.T) {
	dbWrapper := &db.DB{Provider: &concreteMockNonSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost:8080")

	_, err := provider.CRDTPull(context.Background(), "e1", nil)
	if err == nil {
		t.Fatal("expected error for not running in sqlite")
	}
}

func TestDBStateSyncProvider_CRDTPull_Mocked(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockErrorSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost:8080")

	res, err := provider.CRDTPull(context.Background(), "e1", nil)
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if res["crdt_state"] != "latest_mocked_state" {
		t.Errorf("expected latest_mocked_state, got %v", res["crdt_state"])
	}
}

func TestDBStateSyncProvider_SendToCloud_MarshalError(t *testing.T) {
	provider := NewDBStateSyncProvider(&db.DB{}, "http://localhost")
	_, err := provider.sendToCloud(context.Background(), "/api", http.MethodPost, make(chan int), nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestDBStateSyncProvider_SendToCloud_InvalidMethod(t *testing.T) {
	provider := NewDBStateSyncProvider(&db.DB{}, "http://localhost")
	_, err := provider.sendToCloud(context.Background(), "/api", "INV@LID", nil, nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestDBStateSyncProvider_SendToCloud_Spiffe(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("Authorization") != "Bearer test-token" {
			t.Errorf("expected spiffe token")
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	os.Setenv("SPIFFE_IDENTITY_TOKEN", "test-token")
	defer os.Unsetenv("SPIFFE_IDENTITY_TOKEN")

	provider := NewDBStateSyncProvider(&db.DB{}, server.URL)
	_, err := provider.sendToCloud(context.Background(), "/api", http.MethodGet, nil, nil)
	if err != nil {
		t.Fatal(err)
	}
}

func TestDBStateSyncProvider_SendToCloud_ErrorStatus(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer server.Close()

	provider := NewDBStateSyncProvider(&db.DB{}, server.URL)
	_, err := provider.sendToCloud(context.Background(), "/api", http.MethodGet, nil, nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestDBStateSyncProvider_SendToCloud_DoError(t *testing.T) {
	provider := NewDBStateSyncProvider(&db.DB{}, "http://invalid-url-that-does-not-exist.local")
	_, err := provider.sendToCloud(context.Background(), "/api", http.MethodGet, nil, nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestDBStateSyncProvider_SyncUp_QueryError(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockErrorSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost")
	_, err := provider.SyncUp(context.Background(), nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestDBStateSyncProvider_SyncUp_SendError(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://invalid-url-that-does-not-exist.local")
	_, err := provider.SyncUp(context.Background(), nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestDBStateSyncProvider_SyncDown_NotSQLite(t *testing.T) {
	type mockNonSQLiteProvider struct {
		db.Provider
	}
	// Need to implement IsSQLite() to return false to prevent nil pointer dereference
	// because IsSQLite is part of the db.Provider interface which we are embedding implicitly
	// but it defaults to nil interface and triggers a crash.
}

type concreteMockNonSQLiteProvider struct{
	db.Provider
}
func (c *concreteMockNonSQLiteProvider) IsSQLite() bool { return false }
func (c *concreteMockNonSQLiteProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
func (c *concreteMockNonSQLiteProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) { return nil, nil }
func (c *concreteMockNonSQLiteProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) { return 0, nil }

func TestDBStateSyncProvider_SyncDown_NotSQLiteFixed(t *testing.T) {
	dbWrapper := &db.DB{Provider: &concreteMockNonSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost")
	_, err := provider.SyncDown(context.Background(), nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestDBStateSyncProvider_SyncDown_SendError(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://invalid-url-that-does-not-exist.local")
	_, err := provider.SyncDown(context.Background(), nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestDBStateSyncProvider_SyncDown_JSONError(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`invalid-json`))
	}))
	defer server.Close()

	dbWrapper := &db.DB{Provider: &mockSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, server.URL)
	_, err := provider.SyncDown(context.Background(), nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestDBStateSyncProvider_SyncDown_Missions(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"missions": [{"id": "test1", "status": "done"}]}`))
	}))
	defer server.Close()

	dbWrapper := &db.DB{Provider: &mockSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, server.URL)
	_, err := provider.SyncDown(context.Background(), nil)
	if err != nil {
		t.Fatal(err)
	}
}

func TestDBStateSyncProvider_GetStatus_NotSQLite(t *testing.T) {
	dbWrapper := &db.DB{Provider: &concreteMockNonSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost")
	_, err := provider.GetStatus(context.Background(), nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestDBStateSyncProvider_GetStatus_Error(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockErrorSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost")
	_, err := provider.GetStatus(context.Background(), nil)
	if err == nil {
		t.Fatal("expected error")
	}
}

type mockScanErrorSQLiteProvider struct {
	db.Provider
}

func (m *mockScanErrorSQLiteProvider) IsSQLite() bool { return true }
func (m *mockScanErrorSQLiteProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &mockScanErrorRows{count: 1}, nil
}

type mockScanErrorRows struct {
	count int
	idx   int
}
func (r *mockScanErrorRows) Next() bool {
	if r.idx < r.count {
		r.idx++
		return true
	}
	return false
}
func (r *mockScanErrorRows) Scan(dest ...any) error { return errors.New("scan error") }
func (r *mockScanErrorRows) Close() {}
func (r *mockScanErrorRows) Columns() ([]string, error) { return nil, nil }
func (r *mockScanErrorRows) Err() error { return nil }

func TestDBStateSyncProvider_SyncUp_ScanError(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockScanErrorSQLiteProvider{}}
	provider := NewDBStateSyncProvider(dbWrapper, "http://localhost")
	res, err := provider.SyncUp(context.Background(), nil)
	if err != nil {
		t.Fatal(err)
	}
	if res["synced_count"] != 0 {
		t.Fatalf("expected 0, got %v", res["synced_count"])
	}
}

func TestDBStateSyncProvider_SendToCloud_ReadError(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Length", "10") // claim it's 10 bytes long
		w.WriteHeader(http.StatusOK)
		// close immediately without writing body
		if flusher, ok := w.(http.Flusher); ok {
			flusher.Flush()
		}
	}))
	defer server.Close()

	provider := NewDBStateSyncProvider(&db.DB{}, server.URL)
	_, err := provider.sendToCloud(context.Background(), "/api", http.MethodGet, nil, nil)
	if err == nil {
		t.Fatal("expected error")
	}
}
