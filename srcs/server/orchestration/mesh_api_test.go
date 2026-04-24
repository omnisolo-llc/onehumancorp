package orchestration

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandleMeshBroadcast(t *testing.T) {
	tests := []struct {
		name           string
		method         string
		body           string
		expectedStatus int
	}{
		{
			name:           "Valid Payload",
			method:         http.MethodPost,
			body:           `{"agent_id": "agent-1", "channel": "channel-1", "event_type": "event-1", "data": {"key": "value"}}`,
			expectedStatus: http.StatusOK,
		},
		{
			name:           "Invalid Method",
			method:         http.MethodGet,
			body:           `{"agent_id": "agent-1", "channel": "channel-1", "event_type": "event-1", "data": {"key": "value"}}`,
			expectedStatus: http.StatusMethodNotAllowed,
		},
		{
			name:           "Missing AgentID",
			method:         http.MethodPost,
			body:           `{"channel": "channel-1", "event_type": "event-1", "data": {"key": "value"}}`,
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Missing Channel",
			method:         http.MethodPost,
			body:           `{"agent_id": "agent-1", "event_type": "event-1", "data": {"key": "value"}}`,
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Missing EventType",
			method:         http.MethodPost,
			body:           `{"agent_id": "agent-1", "channel": "channel-1", "data": {"key": "value"}}`,
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Missing Data",
			method:         http.MethodPost,
			body:           `{"agent_id": "agent-1", "channel": "channel-1", "event_type": "event-1"}`,
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Invalid JSON",
			method:         http.MethodPost,
			body:           `{invalid-json`,
			expectedStatus: http.StatusBadRequest,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req, err := http.NewRequest(tt.method, "/api/mesh/broadcast", bytes.NewBuffer([]byte(tt.body)))
			if err != nil {
				t.Fatal(err)
			}

			rr := httptest.NewRecorder()
			handler := http.HandlerFunc(HandleMeshBroadcast)

			handler.ServeHTTP(rr, req)

			if status := rr.Code; status != tt.expectedStatus {
				t.Errorf("handler returned wrong status code: got %v want %v", status, tt.expectedStatus)
			}
		})
	}
}
