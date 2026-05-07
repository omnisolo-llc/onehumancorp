package growth

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

func TestHandleAddExperiment(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewExperimentsService(db)

	reqBody := `{"id": "exp-1", "name": "Local vs Cloud", "variant": "local_sovereignty", "traffic_allocation": 0.5}`
	req, err := http.NewRequest("POST", "/api/experiments/add", bytes.NewBufferString(reqBody))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(svc.HandleAddExperiment)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusCreated {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusCreated)
	}

	var resp Experiment
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatal(err)
	}

	if resp.ID != "exp-1" {
		t.Errorf("expected ID 'exp-1', got '%s'", resp.ID)
	}

	// Method Not Allowed
	req, _ = http.NewRequest("GET", "/api/experiments/add", nil)
	rr = httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if status := rr.Code; status != http.StatusMethodNotAllowed {
		t.Errorf("expected Method Not Allowed, got %v", status)
	}

	// Bad Request (Invalid JSON)
	req, _ = http.NewRequest("POST", "/api/experiments/add", bytes.NewBufferString(`{invalid json`))
	rr = httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if status := rr.Code; status != http.StatusBadRequest {
		t.Errorf("expected Bad Request, got %v", status)
	}

	// Bad Request (Database Error)
	req, _ = http.NewRequest("POST", "/api/experiments/add", bytes.NewBufferString(reqBody))
	rr = httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if status := rr.Code; status != http.StatusInternalServerError {
		t.Errorf("expected Internal Server Error, got %v", status)
	}
}

func TestHandleGetAssignment(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewExperimentsService(db)

	// First, add an experiment
	_, err := db.Exec(`
		INSERT INTO experiments (id, name, variant, traffic_allocation)
		VALUES ('exp-2', 'Test Exp', 'variant_B', 1.0)
	`)
	if err != nil {
		t.Fatal(err)
	}

	// Test GET Assignment
	req, err := http.NewRequest("GET", "/api/experiments/assignment?experiment_id=exp-2&user_id=usr-1", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(svc.HandleGetAssignment)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp AssignmentResponse
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatal(err)
	}

	if resp.ExperimentID != "exp-2" {
		t.Errorf("expected ExperimentID 'exp-2', got '%s'", resp.ExperimentID)
	}
	if resp.Variant != "variant_B" {
		t.Errorf("expected Variant 'variant_B', got '%s'", resp.Variant)
	}

	// Test GET Assignment again (should return same assignment)
	rr2 := httptest.NewRecorder()
	handler.ServeHTTP(rr2, req)
	if status := rr2.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	// Method Not Allowed
	reqPost, _ := http.NewRequest("POST", "/api/experiments/assignment", nil)
	rrPost := httptest.NewRecorder()
	handler.ServeHTTP(rrPost, reqPost)
	if status := rrPost.Code; status != http.StatusMethodNotAllowed {
		t.Errorf("expected Method Not Allowed, got %v", status)
	}

	// Missing Params
	reqMiss, _ := http.NewRequest("GET", "/api/experiments/assignment?experiment_id=exp-2", nil)
	rrMiss := httptest.NewRecorder()
	handler.ServeHTTP(rrMiss, reqMiss)
	if status := rrMiss.Code; status != http.StatusBadRequest {
		t.Errorf("expected Bad Request, got %v", status)
	}

	// Not Found Experiment
	reqNF, _ := http.NewRequest("GET", "/api/experiments/assignment?experiment_id=exp-3&user_id=usr-1", nil)
	rrNF := httptest.NewRecorder()
	handler.ServeHTTP(rrNF, reqNF)
	if status := rrNF.Code; status != http.StatusNotFound {
		t.Errorf("expected Not Found, got %v", status)
	}

    // Force DB error for coverage using dropped table
    _, _ = db.Exec(`DROP TABLE experiments`)
    rrErr := httptest.NewRecorder()
	handler.ServeHTTP(rrErr, reqNF)
	if status := rrErr.Code; status != http.StatusInternalServerError {
		t.Errorf("expected Internal Server Error, got %v", status)
	}
}
