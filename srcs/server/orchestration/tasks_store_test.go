package orchestration

import (
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskStoreInterface(t *testing.T) {
	dbProv, _ := db.NewSqliteProvider("file::memory:?cache=shared")
	store := NewTaskStore(dbProv)
	if store == nil {
		t.Fatal("store is nil")
	}
}
