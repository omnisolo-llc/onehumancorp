package orchestration

import (
	"fmt"

	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestTaskStoreInterface(t *testing.T) {
	dbProv, _ := db.NewSqliteProvider(fmt.Sprintf("file:%s?mode=memory&cache=shared", t.Name()))
	store := NewTaskStore(dbProv)
	if store == nil {
		t.Fatal("store is nil")
	}
}
