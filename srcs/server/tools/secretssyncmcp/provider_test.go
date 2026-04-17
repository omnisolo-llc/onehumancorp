package secretssyncmcp

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"database/sql"
	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockDBProvider struct {
	db.Provider
	isSQLite bool
}

func (m *mockDBProvider) IsSQLite() bool {
	return m.isSQLite
}

func TestDBSecretsSyncProvider_SyncSecretsDown_NotSQLite(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockDBProvider{isSQLite: false}}
	provider := NewDBSecretsSyncProvider(dbWrapper, "http://localhost:8080")
	_, err := provider.SyncSecretsDown(context.Background(), nil)
	if err == nil {
		t.Fatalf("expected error for non-SQLite provider")
	}
}

func TestDBSecretsSyncProvider_SyncSecretsUp_NotSQLite(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockDBProvider{isSQLite: false}}
	provider := NewDBSecretsSyncProvider(dbWrapper, "http://localhost:8080")
	_, err := provider.SyncSecretsUp(context.Background(), nil)
	if err == nil {
		t.Fatalf("expected error for non-SQLite provider")
	}
}

func TestDBSecretsSyncProvider_SyncSecretsDown_Success(t *testing.T) {
    // Basic test
    ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"secrets": [{"id": "1", "key": "k1", "value": "v1"}]}`))
	}))
	defer ts.Close()

    // Using real sqlite db for the test
    sqlDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
    if err != nil {
        t.Fatalf("failed to open sqlite: %v", err)
    }
    dbProvider := db.NewSqliteProvider(sqlDB)
    dbWrapper := &db.DB{Provider: dbProvider}

	provider := NewDBSecretsSyncProvider(dbWrapper, ts.URL)
	res, err := provider.SyncSecretsDown(context.Background(), &auth.Claims{OrganizationID: "test-org"})
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

    if res["status"] != "success" {
		t.Errorf("expected status 'success', got %v", res["status"])
	}
}
