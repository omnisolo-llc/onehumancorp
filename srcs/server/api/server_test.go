package api_test

import (
	"bytes"
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/api"
	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestSyncMissionsEndpoint(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer sqlDB.Close()

	prov := db.NewSqliteProvider(sqlDB)
	ctx := context.Background()

	// Create table
	_, err = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload JSON,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	mux := http.NewServeMux()
	api.RegisterRoutes(mux, prov, nil)

	t.Run("ValidPayload", func(t *testing.T) {
		payload := `{"missions": [{"id": "m1", "status": "PENDING", "payload": {"foo": "bar"}}]}`
		req := httptest.NewRequest("POST", "/api/sync/missions", bytes.NewBufferString(payload))
		req.Header.Set("Authorization", "Bearer my-secret-token")
		rec := httptest.NewRecorder()

		mux.ServeHTTP(rec, req)

		if rec.Code != http.StatusOK {
			t.Fatalf("expected 200 OK, got %d", rec.Code)
		}

		var count int
		err := prov.QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions WHERE id = 'm1'").Scan(&count)
		if err != nil {
			t.Fatalf("failed to count: %v", err)
		}
		if count != 1 {
			t.Fatalf("expected 1 record, got %d", count)
		}
	})
}
