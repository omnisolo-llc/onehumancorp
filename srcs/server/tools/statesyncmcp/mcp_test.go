package statesyncmcp

import (
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockDBProvider struct {
	db.Provider
	queryRows *mockRows
	queryErr  error
	execErr   error
	execCount int
}

func (m *mockDBProvider) Query(ctx context.Context, query string, args ...interface{}) (db.Rows, error) {
	if m.queryErr != nil {
		return nil, m.queryErr
	}
	return m.queryRows, nil
}

func (m *mockDBProvider) QueryRow(ctx context.Context, query string, args ...interface{}) db.Row {
    if m.queryErr != nil {
        return &errorRow{err: m.queryErr}
    }
	return m.queryRows
}

type errorRow struct {
    err error
}
func (e *errorRow) Scan(dest ...interface{}) error {
    return e.err
}


func (m *mockDBProvider) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
	if m.execErr != nil {
		return 0, m.execErr
	}
	m.execCount++
	return 1, nil
}

func (m *mockDBProvider) IsSQLite() bool {
	return true
}

type mockResult struct{}

func (r mockResult) LastInsertId() (int64, error) { return 0, nil }
func (r mockResult) RowsAffected() (int64, error) { return 1, nil }

type mockRows struct {
	data [][]interface{}
	idx  int
}

func (m *mockRows) Close() {}
func (m *mockRows) Err() error   { return nil }
func (m *mockRows) Next() bool {
	m.idx++
	return m.idx <= len(m.data)
}
func (m *mockRows) Scan(dest ...interface{}) error {
	if m == nil || len(m.data) == 0 {
		return sql.ErrNoRows
	}
    idx := m.idx
    if idx == 0 {
        idx = 1
    }
	if idx > len(m.data) {
		return sql.ErrNoRows
	}
	row := m.data[idx-1]
	for i, v := range row {
		if i >= len(dest) {
			break
		}
		switch d := dest[i].(type) {
		case *string:
            if s, ok := v.(string); ok {
			    *d = s
            }
		case *time.Time:
            if t, ok := v.(time.Time); ok {
			    *d = t
            }
		}
	}
	return nil
}
func (m *mockRows) Columns() ([]string, error) {
	return nil, nil
}

func TestServer_CallTool(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mockRowsData := &mockRows{
		data: [][]interface{}{
			{"task1", "SHARED_TASK", "PENDING", "IN_PROGRESS", "agent1", "claimed", time.Now()},
		},
	}
	mockDB := &mockDBProvider{queryRows: mockRowsData}

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Header.Get("Authorization") != "Bearer sync-token-org1" {
			t.Errorf("missing auth header")
		}
		w.WriteHeader(http.StatusOK)
		if r.URL.Path == "/api/sync/down" {
			w.Write([]byte(`[{"entity_id":"task2", "entity_type":"SHARED_TASK", "to_state":"COMPLETED", "agent_id":"agent2", "occurred_at":"2023-01-01T00:00:00Z"}]`))
		}
	}))
	defer ts.Close()

	os.Setenv("OHC_CORE_URL", ts.URL)
	defer os.Unsetenv("OHC_CORE_URL")

	provider := NewDefaultStateSyncProvider(mockDB)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})

	// Test sync_local_to_cloud
	res, err := server.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	status := res.(map[string]string)["status"]
	if status != "success" {
		t.Errorf("expected success, got %s", status)
	}
	if mockDB.execCount == 0 {
		t.Errorf("expected cursor update exec")
	}

	// Test sync_cloud_to_local
	mockDB.execCount = 0
	res, err = server.CallTool(ctx, "sync_cloud_to_local", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	status = res.(map[string]string)["status"]
	if status != "success" {
		t.Errorf("expected success, got %s", status)
	}
	if mockDB.execCount == 0 {
		t.Errorf("expected db exec for sync_down")
	}

	// Test get_sync_status
	resStatus, err := server.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if resStatus.(map[string]interface{})["status"] != "synced" {
		t.Errorf("expected synced status, got %v", resStatus)
	}
}

func TestServer_CallTool_CloudMode(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	provider := NewDefaultStateSyncProvider(nil)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})

	res, err := server.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if res.(map[string]string)["status"] != "success" {
		t.Errorf("expected success in cloud mode")
	}

	// test get status cloud mode
	resStatus, err := server.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if resStatus.(map[string]interface{})["mode"] != "cloud" {
		t.Errorf("expected cloud status, got %v", resStatus)
	}
}

func TestServer_CallTool_MissingClaims(t *testing.T) {
	server := NewServer(nil)
	_, err := server.CallTool(context.Background(), "sync_local_to_cloud", nil)
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Errorf("expected unauthorized error, got %v", err)
	}
}

func TestServer_ListTools(t *testing.T) {
	server := NewServer(nil)
	tools, err := server.ListTools(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}
}

func TestServer_CallTool_InvalidTool(t *testing.T) {
	server := NewServer(nil)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := server.CallTool(ctx, "invalid_tool", nil)
	if err == nil || err.Error() != "unknown tool: invalid_tool" {
		t.Errorf("expected unknown tool error, got %v", err)
	}
}

func TestServer_CallTool_SyncUpError(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mockDB := &mockDBProvider{queryErr: os.ErrPermission}
	provider := NewDefaultStateSyncProvider(mockDB)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := server.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestServer_CallTool_SyncDownError(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	os.Setenv("OHC_CORE_URL", "http://invalid-url:0")
	defer os.Unsetenv("OHC_CORE_URL")

	provider := NewDefaultStateSyncProvider(nil)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := server.CallTool(ctx, "sync_cloud_to_local", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestServer_CallTool_SyncUpHTTPError(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mockRowsData := &mockRows{
		data: [][]interface{}{
			{"task1", "SHARED_TASK", "PENDING", "IN_PROGRESS", "agent1", "claimed", time.Now()},
		},
	}
	mockDB := &mockDBProvider{queryRows: mockRowsData}

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer ts.Close()

	os.Setenv("OHC_CORE_URL", ts.URL)
	defer os.Unsetenv("OHC_CORE_URL")

	provider := NewDefaultStateSyncProvider(mockDB)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := server.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestServer_CallTool_SyncDownHTTPError(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
	}))
	defer ts.Close()

	os.Setenv("OHC_CORE_URL", ts.URL)
	defer os.Unsetenv("OHC_CORE_URL")

	provider := NewDefaultStateSyncProvider(nil)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := server.CallTool(ctx, "sync_cloud_to_local", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestServer_CallTool_SyncDownDecodeError(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`invalid json`))
	}))
	defer ts.Close()

	os.Setenv("OHC_CORE_URL", ts.URL)
	defer os.Unsetenv("OHC_CORE_URL")

	provider := NewDefaultStateSyncProvider(nil)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := server.CallTool(ctx, "sync_cloud_to_local", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestServer_CallTool_SyncUpNoTransitions(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mockRowsData := &mockRows{
		data: [][]interface{}{},
	}
	mockDB := &mockDBProvider{queryRows: mockRowsData}

	provider := NewDefaultStateSyncProvider(mockDB)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	res, err := server.CallTool(ctx, "sync_local_to_cloud", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if res.(map[string]string)["status"] != "success" {
		t.Errorf("expected success")
	}
}

func TestServer_CallTool_GetStatusError(t *testing.T) {
	server := NewServer(&errorStateSyncProvider{})
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := server.CallTool(ctx, "get_sync_status", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

type errorStateSyncProvider struct{}

func (p *errorStateSyncProvider) SyncUp(ctx context.Context, claims *auth.Claims) error {
	return nil
}

func (p *errorStateSyncProvider) SyncDown(ctx context.Context, claims *auth.Claims) error {
	return nil
}

func (p *errorStateSyncProvider) GetStatus(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	return nil, os.ErrPermission
}

func TestServer_CallTool_SyncDownHTTPReqError(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	os.Setenv("OHC_CORE_URL", string([]byte{0x7f})) // invalid URL to trigger NewRequestWithContext error
	defer os.Unsetenv("OHC_CORE_URL")

	provider := NewDefaultStateSyncProvider(nil)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := server.CallTool(ctx, "sync_cloud_to_local", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestServer_CallTool_SyncUpHTTPReqError(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mockRowsData := &mockRows{
		data: [][]interface{}{
			{"task1", "SHARED_TASK", "PENDING", "IN_PROGRESS", "agent1", "claimed", time.Now()},
		},
	}
	mockDB := &mockDBProvider{queryRows: mockRowsData}

	os.Setenv("OHC_CORE_URL", string([]byte{0x7f})) // invalid URL to trigger NewRequestWithContext error
	defer os.Unsetenv("OHC_CORE_URL")

	provider := NewDefaultStateSyncProvider(mockDB)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := server.CallTool(ctx, "sync_local_to_cloud", nil)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestServer_CallTool_GetStatusCloudMode(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	provider := NewDefaultStateSyncProvider(nil)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	res, err := server.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	status := res.(map[string]interface{})["status"]
	if status != "active" {
		t.Errorf("expected active, got %s", status)
	}
}

func TestServer_CallTool_GetStatusNeverSynced(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mockRowsData := &mockRows{
		data: [][]interface{}{},
	}
	mockDB := &mockDBProvider{queryRows: mockRowsData}

	provider := NewDefaultStateSyncProvider(mockDB)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	res, err := server.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	syncTime := res.(map[string]interface{})["last_synced_at"]
	if syncTime != "never" {
		t.Errorf("expected never, got %s", syncTime)
	}
}

func TestServer_CallTool_GetStatusUnknown(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mockDB := &mockDBProvider{queryErr: os.ErrPermission}
	provider := NewDefaultStateSyncProvider(mockDB)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	res, err := server.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	syncTime := res.(map[string]interface{})["last_synced_at"]
	if syncTime != "unknown" {
		t.Errorf("expected unknown, got %s", syncTime)
	}
}

func TestServer_CallTool_GetStatusSynced(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mockRowsData := &mockRows{
		data: [][]interface{}{
			{time.Now()},
		},
	}
	mockDB := &mockDBProvider{queryRows: mockRowsData}

	provider := NewDefaultStateSyncProvider(mockDB)
	server := NewServer(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	res, err := server.CallTool(ctx, "get_sync_status", nil)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	syncTime := res.(map[string]interface{})["last_synced_at"]
	if syncTime == "unknown" || syncTime == "never" {
		t.Errorf("expected time, got %s", syncTime)
	}
}
