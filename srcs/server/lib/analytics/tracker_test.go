package analytics

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
		t.Fatalf("failed to open test db: %v", err)
	}
	return db
}

func TestHandleTrackEvent(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	tracker := NewTracker(db)

	// Valid POST request
	reqBody := `{"id": "evt-123", "user_id": "usr-456", "event_name": "landing_page_view", "metadata": "{}"}`
	req, err := http.NewRequest("POST", "/api/analytics/track", bytes.NewBufferString(reqBody))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(tracker.HandleTrackEvent)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusCreated {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusCreated)
	}

	var resp Event
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatal(err)
	}

	if resp.ID != "evt-123" {
		t.Errorf("expected ID 'evt-123', got '%s'", resp.ID)
	}

	// Method Not Allowed
	req, err = http.NewRequest("GET", "/api/analytics/track", nil)
	if err != nil {
		t.Fatal(err)
	}
	rr = httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if status := rr.Code; status != http.StatusMethodNotAllowed {
		t.Errorf("expected Method Not Allowed, got %v", status)
	}

	// Bad Request (Invalid JSON)
	req, err = http.NewRequest("POST", "/api/analytics/track", bytes.NewBufferString(`{invalid json`))
	if err != nil {
		t.Fatal(err)
	}
	rr = httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if status := rr.Code; status != http.StatusBadRequest {
		t.Errorf("expected Bad Request, got %v", status)
	}

	// Bad Request (Database Error)
    // Create a conflict error
	req, err = http.NewRequest("POST", "/api/analytics/track", bytes.NewBufferString(reqBody))
	if err != nil {
		t.Fatal(err)
	}
	rr = httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if status := rr.Code; status != http.StatusInternalServerError {
		t.Errorf("expected Internal Server Error, got %v", status)
	}
}
