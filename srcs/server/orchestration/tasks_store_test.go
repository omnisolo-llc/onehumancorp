package orchestration

import (
    "database/sql"

	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskStoreInterface(t *testing.T) {
	d, _ := sql.Open("sqlite", "file::memory:?cache=shared")
    dbProv := db.NewSqliteProvider(d)
	store := NewTaskStore(dbProv)
	if store == nil {
		t.Fatal("store is nil")
	}
}
