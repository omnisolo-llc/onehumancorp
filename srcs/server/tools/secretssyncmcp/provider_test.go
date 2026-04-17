package secretssyncmcp

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockDBProvider struct {
	isSQLite bool
}

func (m *mockDBProvider) Exec(ctx context.Context, sql string, args ...interface{}) (interface{}, error) {
	return nil, nil
}

func (m *mockDBProvider) Query(ctx context.Context, sql string, args ...interface{}) (db.Rows, error) {
	return nil, nil
}

func (m *mockDBProvider) QueryRow(ctx context.Context, sql string, args ...interface{}) db.Row {
	return nil
}

func (m *mockDBProvider) Begin(ctx context.Context) (db.Tx, error) {
	return nil, nil
}

func (m *mockDBProvider) IsSQLite() bool {
	return m.isSQLite
}

func (m *mockDBProvider) Close() {
}

func (m *mockDBProvider) AcquireTask(ctx context.Context, taskType, nodeID string) (*db.TaskRecord, error) {
	return nil, nil
}
func (m *mockDBProvider) ClaimTask(ctx context.Context, taskID int) (bool, error) {
    return true, nil
}
func (m *mockDBProvider) ExecContext(ctx context.Context, sql string, args ...interface{}) (interface{}, error) {
    return nil, nil
}


func TestDBSecretsSyncProvider_SyncSecretsDown_NotSQLite(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockDBProvider{isSQLite: false}}
	provider := NewDBSecretsSyncProvider(dbWrapper, "http://localhost:8080")

	_, err := provider.SyncSecretsDown(context.Background(), &auth.Claims{})
	if err == nil {
		t.Error("expected error for non-sqlite db")
	}
}

func TestDBSecretsSyncProvider_SyncSecretsUp_NotSQLite(t *testing.T) {
	dbWrapper := &db.DB{Provider: &mockDBProvider{isSQLite: false}}
	provider := NewDBSecretsSyncProvider(dbWrapper, "http://localhost:8080")

	_, err := provider.SyncSecretsUp(context.Background(), &auth.Claims{})
	if err == nil {
		t.Error("expected error for non-sqlite db")
	}
}

func TestDBSecretsSyncProvider_SyncSecretsDown_Success(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(map[string]interface{}{
			"secrets": []map[string]interface{}{
				{"id": "1", "key": "k1", "value": "v1"},
			},
		})
	}))
	defer ts.Close()

	// Need a real sqlite driver for testing execs
	dbInstance, err := db.New(context.Background())
if err != nil { t.Fatal(err) }
defer dbInstance.Close()
dbWrapper := dbInstance


	provider := NewDBSecretsSyncProvider(dbWrapper, ts.URL)
	_, err := provider.SyncSecretsDown(context.Background(), &auth.Claims{})
	if err != nil {
		t.Errorf("expected no error, got: %v", err)
	}
}
