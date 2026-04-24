package auth

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandleRegister_Valid(t *testing.T) {
	store := NewStore()
	h := NewHandlers(store)

	reqBody := `{"username": "testuser", "email": "test@example.com", "password": "password123"}`
	req := httptest.NewRequest(http.MethodPost, "/api/auth/register", bytes.NewBufferString(reqBody))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	h.HandleRegister(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status OK, got %d. body: %s", w.Code, w.Body.String())
	}

	var resp loginResponse
	if err := json.NewDecoder(w.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp.Token == "" {
		t.Errorf("expected token in response")
	}
	if resp.User.Username != "testuser" {
		t.Errorf("expected username testuser, got %s", resp.User.Username)
	}
}

func TestHandleRegister_InvalidMethod(t *testing.T) {
	h := NewHandlers(NewStore())
	req := httptest.NewRequest(http.MethodGet, "/api/auth/register", nil)
	w := httptest.NewRecorder()
	h.HandleRegister(w, req)

	if w.Code != http.StatusMethodNotAllowed {
		t.Errorf("expected status MethodNotAllowed, got %d", w.Code)
	}
}

func TestHandleRegister_InvalidJSON(t *testing.T) {
	h := NewHandlers(NewStore())
	req := httptest.NewRequest(http.MethodPost, "/api/auth/register", bytes.NewBufferString(`{bad json`))
	w := httptest.NewRecorder()
	h.HandleRegister(w, req)

	if w.Code != http.StatusBadRequest {
		t.Errorf("expected status BadRequest, got %d", w.Code)
	}
}

func TestHandleRegister_MissingFields(t *testing.T) {
	h := NewHandlers(NewStore())
	reqBody := `{"username": "testuser", "password": ""}` // missing password basically
	req := httptest.NewRequest(http.MethodPost, "/api/auth/register", bytes.NewBufferString(reqBody))
	w := httptest.NewRecorder()
	h.HandleRegister(w, req)

	if w.Code != http.StatusBadRequest {
		t.Errorf("expected status BadRequest, got %d", w.Code)
	}
}

func TestHandleRegister_DuplicateUser(t *testing.T) {
	store := NewStore()
	h := NewHandlers(store)

	_, _ = store.CreateUser("testuser", "test@example.com", "password123", []string{RoleAdmin}, "")

	reqBody := `{"username": "testuser", "email": "test2@example.com", "password": "password123"}`
	req := httptest.NewRequest(http.MethodPost, "/api/auth/register", bytes.NewBufferString(reqBody))
	w := httptest.NewRecorder()
	h.HandleRegister(w, req)

	if w.Code != http.StatusBadRequest {
		t.Errorf("expected status BadRequest on duplicate user, got %d", w.Code)
	}
}
