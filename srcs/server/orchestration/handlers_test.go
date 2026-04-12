package orchestration

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type MockProvider struct {
	db.Provider
	IsSQLiteMock bool
}

func (m *MockProvider) IsSQLite() bool { return m.IsSQLiteMock }
func (m *MockProvider) QueryRow(ctx context.Context, query string, args ...interface{}) db.Row {
	return &MockRow{}
}
func (m *MockProvider) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
	return 1, nil
}
func (m *MockProvider) Query(ctx context.Context, query string, args ...interface{}) (db.Rows, error) {
	return &MockRows{}, nil
}

type MockRow struct{}
func (m *MockRow) Scan(dest ...interface{}) error { return nil }

type MockRows struct{ count int }
func (m *MockRows) Next() bool {
	m.count++
	return m.count == 1
}
func (m *MockRows) Scan(dest ...interface{}) error {
	*dest[0].(*string) = "id1"
	*dest[1].(*string) = "title1"
	*dest[2].(*string) = "status1"
	return nil
}
func (m *MockRows) Close() {}
func (m *MockRows) Err() error { return nil }
func (m *MockRows) Columns() ([]string, error) { return []string{"id", "title", "status"}, nil }

func TestSharedTaskHandler(t *testing.T) {
	t.Setenv("TEST_ENV", "true")
	provider := &MockProvider{IsSQLiteMock: true}
	handler := NewSharedTaskHandler(provider)

	// Create request with claims
	claims := &auth.Claims{OrganizationID: "org1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	t.Run("CreateTask", func(t *testing.T) {
		body := []byte(`{"title":"t", "description":"d"}`)
		req := httptest.NewRequest("POST", "/tasks", bytes.NewReader(body))
		req = req.WithContext(ctx)
		w := httptest.NewRecorder()
		handler.CreateTask(w, req)
		if w.Code != http.StatusCreated {
			t.Errorf("expected %d, got %d", http.StatusCreated, w.Code)
		}
	})

	t.Run("UpdateTask", func(t *testing.T) {
		body := []byte(`{"status":"COMPLETED"}`)
		req := httptest.NewRequest("PUT", "/tasks/123", bytes.NewReader(body))
		req = req.WithContext(ctx)
		w := httptest.NewRecorder()
		handler.UpdateTask(w, req)
		if w.Code != http.StatusOK {
			t.Errorf("expected %d, got %d", http.StatusOK, w.Code)
		}
	})

	t.Run("ListTasks", func(t *testing.T) {
		req := httptest.NewRequest("GET", "/tasks", nil)
		req = req.WithContext(ctx)
		w := httptest.NewRecorder()
		handler.ListTasks(w, req)
		if w.Code != http.StatusOK {
			t.Errorf("expected %d, got %d", http.StatusOK, w.Code)
		}
	})

	t.Run("LockTask", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/tasks/123/lock", nil)
		req = req.WithContext(ctx)
		w := httptest.NewRecorder()
		handler.LockTask(w, req)
		if w.Code != http.StatusOK {
			t.Errorf("expected %d, got %d", http.StatusOK, w.Code)
		}
	})

	// Unauthorized test
	t.Run("Unauthorized", func(t *testing.T) {
		req := httptest.NewRequest("POST", "/tasks", nil)
		w := httptest.NewRecorder()
		handler.CreateTask(w, req)
		if w.Code != http.StatusUnauthorized {
			t.Errorf("expected %d, got %d", http.StatusUnauthorized, w.Code)
		}
	})
}
