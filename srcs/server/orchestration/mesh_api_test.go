package orchestration

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
)

type MockTransport struct {
	BroadcastCalled bool
	LastChannel     string
	LastPayload     []byte
}

func (m *MockTransport) Broadcast(channel string, payload []byte) error {
	m.BroadcastCalled = true
	m.LastChannel = channel
	m.LastPayload = payload
	return nil
}

func TestMeshAPI(t *testing.T) {
	mockTransport := &MockTransport{}
	api := &MeshAPI{Transport: mockTransport}

	tests := []struct {
		name            string
		payload         string
		statusCode      int
		shouldBroadcast bool
	}{
		{
			name:            "valid",
			payload:         `{"agent_id": "spiffe://example.org/agent", "channel": "mesh:tasks", "event_type": "test", "data": {}}`,
			statusCode:      http.StatusOK,
			shouldBroadcast: true,
		},
		{
			name:            "missing agent_id",
			payload:         `{"channel": "mesh:tasks", "event_type": "test", "data": {}}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
		{
			name:            "invalid agent_id",
			payload:         `{"agent_id": "invalid", "channel": "mesh:tasks", "event_type": "test", "data": {}}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
		{
			name:            "missing channel",
			payload:         `{"agent_id": "spiffe://example.org/agent", "event_type": "test", "data": {}}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
		{
			name:            "invalid channel",
			payload:         `{"agent_id": "spiffe://example.org/agent", "channel": "invalid", "event_type": "test", "data": {}}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
		{
			name:            "missing event_type",
			payload:         `{"agent_id": "spiffe://example.org/agent", "channel": "mesh:tasks", "data": {}}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
		{
			name:            "missing data",
			payload:         `{"agent_id": "spiffe://example.org/agent", "channel": "mesh:tasks", "event_type": "test"}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mockTransport.BroadcastCalled = false // reset for each run
			req, err := http.NewRequest("POST", "/api/mesh/broadcast", bytes.NewBuffer([]byte(tt.payload)))
			if err != nil {
				t.Fatal(err)
			}

			rr := httptest.NewRecorder()
			handler := http.HandlerFunc(api.HandleBroadcast)

			handler.ServeHTTP(rr, req)

			if status := rr.Code; status != tt.statusCode {
				t.Errorf("handler returned wrong status code: got %v want %v",
					status, tt.statusCode)
			}

			if tt.shouldBroadcast && !mockTransport.BroadcastCalled {
				t.Errorf("expected Broadcast to be called, but it was not")
			}

			if !tt.shouldBroadcast && mockTransport.BroadcastCalled {
				t.Errorf("expected Broadcast NOT to be called, but it was")
			}
		})
	}
}
