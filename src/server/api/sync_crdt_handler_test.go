package api

import (
	"bytes"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open DB: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE crdt_deltas (
			id TEXT PRIMARY KEY,
			entity_id TEXT,
			data TEXT,
			updated_at TEXT
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestSyncCRDTHandler(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	handler := NewSyncCRDTHandler(db)

	payload := CRDTPayload{
		Deltas: []CRDTDelta{
			{ID: "1", EntityID: "e1", Data: "d1", UpdatedAt: "2023-01-01T00:00:00Z"},
		},
	}
	body, _ := json.Marshal(payload)
	req := httptest.NewRequest("POST", "/api/v1/sync/mcp-deltas", bytes.NewBuffer(body))
	w := httptest.NewRecorder()

	handler.HandlePost(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("Expected status 200, got %d", w.Code)
	}

	var count int
	err := db.QueryRow("SELECT COUNT(*) FROM crdt_deltas").Scan(&count)
	if err != nil {
		t.Errorf("Failed to query DB: %v", err)
	}
	if count != 1 {
		t.Errorf("Expected 1 row, got %d", count)
	}
}
