package orchestration

import (
	"bytes"
	"io"
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

func TestMeshAPI_Integration(t *testing.T) {
	mockTransport := &MockTransport{}
	api := &MeshAPI{Transport: mockTransport}

	mux := http.NewServeMux()
	mux.HandleFunc("/api/mesh/broadcast", api.HandleBroadcast)
	mux.HandleFunc("/api/mesh/publish", api.HandlePublish)

	server := httptest.NewServer(mux)
	defer server.Close()

	tests := []struct {
		name            string
		endpoint        string
		payload         string
		statusCode      int
		shouldBroadcast bool
	}{
		{
			name:            "valid broadcast",
			endpoint:        "/api/mesh/broadcast",
			payload:         `{"agent_id": "spiffe://example.org/agent", "channel": "mesh:tasks", "event_type": "test", "data": {}}`,
			statusCode:      http.StatusOK,
			shouldBroadcast: true,
		},
		{
			name:            "valid publish",
			endpoint:        "/api/mesh/publish",
			payload:         `{"agent_id": "spiffe://example.org/agent", "channel": "mesh:tasks", "event_type": "test", "data": {}}`,
			statusCode:      http.StatusOK,
			shouldBroadcast: true,
		},
		{
			name:            "missing agent_id",
			endpoint:        "/api/mesh/broadcast",
			payload:         `{"channel": "mesh:tasks", "event_type": "test", "data": {}}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
		{
			name:            "invalid agent_id prefix",
			endpoint:        "/api/mesh/publish",
			payload:         `{"agent_id": "invalid", "channel": "mesh:tasks", "event_type": "test", "data": {}}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
		{
			name:            "missing channel",
			endpoint:        "/api/mesh/broadcast",
			payload:         `{"agent_id": "spiffe://example.org/agent", "event_type": "test", "data": {}}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
		{
			name:            "invalid channel format",
			endpoint:        "/api/mesh/publish",
			payload:         `{"agent_id": "spiffe://example.org/agent", "channel": "invalid", "event_type": "test", "data": {}}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
		{
			name:            "missing event_type",
			endpoint:        "/api/mesh/broadcast",
			payload:         `{"agent_id": "spiffe://example.org/agent", "channel": "mesh:tasks", "data": {}}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
		{
			name:            "missing data",
			endpoint:        "/api/mesh/publish",
			payload:         `{"agent_id": "spiffe://example.org/agent", "channel": "mesh:tasks", "event_type": "test"}`,
			statusCode:      http.StatusBadRequest,
			shouldBroadcast: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mockTransport.BroadcastCalled = false // reset for each run
			res, err := http.Post(server.URL+tt.endpoint, "application/json", bytes.NewBuffer([]byte(tt.payload)))
			if err != nil {
				t.Fatal(err)
			}
			defer res.Body.Close()

			// Read body to clear the connection
			_, _ = io.ReadAll(res.Body)

			if status := res.StatusCode; status != tt.statusCode {
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
