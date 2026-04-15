package orchestration

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type storeMockTx struct {
	db.Tx
}

func (m *storeMockTx) Exec(ctx context.Context, sql string, arguments ...interface{}) (int64, error) {
	return 0, nil
}
func (m *storeMockTx) QueryRow(ctx context.Context, sql string, args ...interface{}) db.Row {
	return &storeMockRow{}
}
func (m *storeMockTx) Commit(ctx context.Context) error {
	return nil
}
func (m *storeMockTx) Rollback(ctx context.Context) error {
	return nil
}

type storeMockRow struct {
	db.Row
}

func (m *storeMockRow) Scan(dest ...interface{}) error {
	// simulate no rows
	return nil
}

type storeMockDB struct {
	db.Provider
	sqlite bool
}

func (m *storeMockDB) IsSQLite() bool {
	return m.sqlite
}

func (m *storeMockDB) Begin(ctx context.Context) (db.Tx, error) {
	return &storeMockTx{}, nil
}

func TestClaimTaskNoClaims(t *testing.T) {
	store := NewTaskStore(&storeMockDB{})
	_, err := store.ClaimTask(context.Background(), "agent1")
	if err == nil {
		t.Fatal("expected error due to no claims")
	}
}

func TestClaimTaskSQLite(t *testing.T) {
	store := NewTaskStore(&storeMockDB{sqlite: true})
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := store.ClaimTask(ctx, "agent1")
	if err != nil {
		t.Fatalf("expected nil error (no rows), got %v", err)
	}
}

func TestClaimTaskPostgres(t *testing.T) {
	store := NewTaskStore(&storeMockDB{sqlite: false})
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := store.ClaimTask(ctx, "agent1")
	if err != nil {
		t.Fatalf("expected nil error (no rows), got %v", err)
	}
}
