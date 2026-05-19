package sync

import (
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
)

func TestProcessForecastTick(t *testing.T) {
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		assert.Equal(t, "/api/v1/sync/autodream", r.URL.Path)
		w.WriteHeader(http.StatusOK)
	}))
	defer server.Close()

	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	t.Setenv("OHC_CORE_URL", server.URL)
	db, err := sql.Open("sqlite3", ":memory:")
	assert.NoError(t, err)
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE embedding_cache (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding BLOB,
			synced_to_cloud BOOLEAN DEFAULT false
		);
	`)
	assert.NoError(t, err)

	_, err = db.Exec("INSERT INTO embedding_cache (id, content, synced_to_cloud) VALUES ('uuid-1', 'test1', false)")
	assert.NoError(t, err)

	engine := NewAutoDreamSyncEngine(db)
	err = engine.ProcessForecastTick(context.Background())
	assert.NoError(t, err)

	var synced bool
	err = db.QueryRow("SELECT synced_to_cloud FROM embedding_cache WHERE id = 'uuid-1'").Scan(&synced)
	assert.NoError(t, err)
	assert.True(t, synced)
}
