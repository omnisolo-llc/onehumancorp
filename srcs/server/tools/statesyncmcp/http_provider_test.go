package statesyncmcp

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockLocalDB struct {
	db.Provider
	isSQLite bool
}

func (m *mockLocalDB) IsSQLite() bool {
	return m.isSQLite
}

func TestHTTPProvider_SyncUp_SQLite(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			t.Errorf("expected POST, got %s", r.Method)
		}
		if r.URL.Path != "/api/v1/sync/up" {
			t.Errorf("expected /api/v1/sync/up, got %s", r.URL.Path)
		}
		if r.Header.Get("X-Tenant-ID") != "test-org" {
			t.Errorf("expected X-Tenant-ID 'test-org', got '%s'", r.Header.Get("X-Tenant-ID"))
		}
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"success","items_synced":5}`))
	}))
	defer server.Close()

	localDB := &mockLocalDB{isSQLite: true}
	provider := NewHTTPProvider(server.URL, localDB)

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "test-org"}

	result, err := provider.SyncUp(ctx, claims)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if result["status"] != "success" {
		t.Errorf("expected status 'success', got %v", result["status"])
	}
	if items := result["items_synced"].(float64); items != 5 {
		t.Errorf("expected items_synced 5, got %v", items)
	}
}

func TestHTTPProvider_SyncUp_NotSQLite(t *testing.T) {
	localDB := &mockLocalDB{isSQLite: false}
	provider := NewHTTPProvider("http://localhost", localDB)

	ctx := context.Background()
	result, err := provider.SyncUp(ctx, nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if result["status"] != "skipped" {
		t.Errorf("expected status 'skipped', got %v", result["status"])
	}
}

func TestHTTPProvider_SyncDown_SQLite(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			t.Errorf("expected GET, got %s", r.Method)
		}
		if r.URL.Path != "/api/v1/sync/down" {
			t.Errorf("expected /api/v1/sync/down, got %s", r.URL.Path)
		}
		if r.Header.Get("X-Tenant-ID") != "test-org" {
			t.Errorf("expected X-Tenant-ID 'test-org', got '%s'", r.Header.Get("X-Tenant-ID"))
		}
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"success","items_fetched":3}`))
	}))
	defer server.Close()

	localDB := &mockLocalDB{isSQLite: true}
	provider := NewHTTPProvider(server.URL, localDB)

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "test-org"}

	result, err := provider.SyncDown(ctx, claims)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if result["status"] != "success" {
		t.Errorf("expected status 'success', got %v", result["status"])
	}
	if items := result["items_fetched"].(float64); items != 3 {
		t.Errorf("expected items_fetched 3, got %v", items)
	}
}

func TestHTTPProvider_SyncDown_NotSQLite(t *testing.T) {
	localDB := &mockLocalDB{isSQLite: false}
	provider := NewHTTPProvider("http://localhost", localDB)

	ctx := context.Background()
	result, err := provider.SyncDown(ctx, nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if result["status"] != "skipped" {
		t.Errorf("expected status 'skipped', got %v", result["status"])
	}
}

func TestHTTPProvider_GetStatus_SQLite(t *testing.T) {
	localDB := &mockLocalDB{isSQLite: true}
	provider := NewHTTPProvider("http://localhost", localDB)

	ctx := context.Background()
	status, err := provider.GetStatus(ctx, nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if status.Status != "synchronized" {
		t.Errorf("expected status 'synchronized', got %v", status.Status)
	}
	if status.PendingItems != 0 {
		t.Errorf("expected pending_items 0, got %v", status.PendingItems)
	}
	if status.LastSyncTime == "" {
		t.Error("expected LastSyncTime to be set")
	}
}

func TestHTTPProvider_GetStatus_NotSQLite(t *testing.T) {
	localDB := &mockLocalDB{isSQLite: false}
	provider := NewHTTPProvider("http://localhost", localDB)

	ctx := context.Background()
	status, err := provider.GetStatus(ctx, nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if status.Status != "skipped (cloud mode)" {
		t.Errorf("expected status 'skipped (cloud mode)', got %v", status.Status)
	}
}

func TestNoopProvider(t *testing.T) {
	provider := NewNoopProvider()
	ctx := context.Background()

	resUp, err := provider.SyncUp(ctx, nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resUp["status"] != "skipped" {
		t.Errorf("expected status 'skipped', got %v", resUp["status"])
	}

	resDown, err := provider.SyncDown(ctx, nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if resDown["status"] != "skipped" {
		t.Errorf("expected status 'skipped', got %v", resDown["status"])
	}

	status, err := provider.GetStatus(ctx, nil)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if status.Status != "skipped (cloud mode)" {
		t.Errorf("expected status 'skipped (cloud mode)', got %v", status.Status)
	}
}

func TestHTTPProvider_Errors(t *testing.T) {
	// Setup failing server
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte("internal server error"))
	}))
	defer server.Close()

	localDB := &mockLocalDB{isSQLite: true}
	provider := NewHTTPProvider(server.URL, localDB)
	ctx := context.Background()

	_, err := provider.SyncUp(ctx, nil)
	if err == nil {
		t.Error("expected error for SyncUp with failing server")
	}

	_, err = provider.SyncDown(ctx, nil)
	if err == nil {
		t.Error("expected error for SyncDown with failing server")
	}
}
