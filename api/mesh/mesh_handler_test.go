package mesh

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
)

type mockMeshService struct {
	broadcastCalled bool
}

func (m *mockMeshService) BroadcastIntent(ctx context.Context, intent string) error {
	m.broadcastCalled = true
	return nil
}

func (m *mockMeshService) Subscribe(ctx context.Context) (<-chan string, error) {
	return nil, nil
}

func TestBroadcastHandler(t *testing.T) {
	mockSvc := &mockMeshService{}
	handler := BroadcastHandler(mockSvc)

	// Test invalid method
	req := httptest.NewRequest(http.MethodGet, "/broadcast", nil)
	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if rr.Code != http.StatusMethodNotAllowed {
		t.Errorf("Expected 405, got %d", rr.Code)
	}

	// Test missing fields
	reqBody := bytes.NewBuffer([]byte(`{"agent_id": "123"}`))
	req = httptest.NewRequest(http.MethodPost, "/broadcast", reqBody)
	rr = httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if rr.Code != http.StatusBadRequest {
		t.Errorf("Expected 400 for missing fields, got %d", rr.Code)
	}

	// Test valid request
	validBody := bytes.NewBuffer([]byte(`{"agent_id": "123", "action": "CREATE", "status": "PENDING"}`))
	req = httptest.NewRequest(http.MethodPost, "/broadcast", validBody)
	rr = httptest.NewRecorder()
	handler.ServeHTTP(rr, req)
	if rr.Code != http.StatusOK {
		t.Errorf("Expected 200 OK, got %d", rr.Code)
	}
	if !mockSvc.broadcastCalled {
		t.Error("Expected BroadcastIntent to be called")
	}
}
