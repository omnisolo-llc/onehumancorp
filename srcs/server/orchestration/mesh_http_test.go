package orchestration

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandleMeshDirect(t *testing.T) {
	// Need a repo and a hub, so we'll just test HTTP method fallback to be safe
	// without needing a full DB mock.
	hub := &Hub{}

	// Test GET method not allowed
	reqGet, _ := http.NewRequest("GET", "/api/mesh/direct", nil)
	rrGet := httptest.NewRecorder()

	handler := http.HandlerFunc(hub.HandleMeshDirect)
	handler.ServeHTTP(rrGet, reqGet)

	if status := rrGet.Code; status != http.StatusMethodNotAllowed {
		t.Errorf("handler returned wrong status code for GET: got %v want %v",
			status, http.StatusMethodNotAllowed)
	}

	// Test Bad JSON
	reqBad, _ := http.NewRequest("POST", "/api/mesh/direct", bytes.NewBuffer([]byte(`{bad json`)))
	rrBad := httptest.NewRecorder()
	handler.ServeHTTP(rrBad, reqBad)
	if status := rrBad.Code; status != http.StatusBadRequest {
		t.Errorf("handler returned wrong status code for bad json: got %v want %v", status, http.StatusBadRequest)
	}
}

func TestHandleMeshMailbox(t *testing.T) {
	hub := &Hub{}

	reqGet, _ := http.NewRequest("POST", "/api/mesh/mailbox", nil)
	rrGet := httptest.NewRecorder()

	handler := http.HandlerFunc(hub.HandleMeshMailbox)
	handler.ServeHTTP(rrGet, reqGet)

	if status := rrGet.Code; status != http.StatusMethodNotAllowed {
		t.Errorf("handler returned wrong status code for POST: got %v want %v",
			status, http.StatusMethodNotAllowed)
	}
}
