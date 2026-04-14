package orchestration

import (
	"database/sql"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestDecompositionTaskOrchestrator_ClaimTask(t *testing.T) {
	sqlDb, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	dbProv := db.NewSqliteProvider(sqlDb)
	store := NewDecompositionTaskOrchestrator(dbProv)
	if store == nil {
		t.Fatal("store is nil")
	}
}
