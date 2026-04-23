package mesh

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestValidationMiddleware(t *testing.T) {
	handler := ValidationMiddleware(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))

	tests := []struct {
		name           string
		method         string
		body           []byte
		expectedStatus int
	}{

		{
			name:           "Valid KAIROS payload for mesh:tasks",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "123", "channel": "mesh:tasks", "event_type": "TASK_TRANSITION", "action": "CREATE", "status": "PENDING", "data": {}}`),
			expectedStatus: http.StatusOK,
		},
		{
			name:           "Valid payload for other channels",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "123", "channel": "other_channel", "event_type": "TASK_TRANSITION", "data": {}}`),
			expectedStatus: http.StatusOK,
		},
		{
			name:           "Missing action for mesh:tasks",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "123", "channel": "mesh:tasks", "event_type": "TASK_TRANSITION", "status": "PENDING", "data": {}}`),
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Missing status for mesh:tasks",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "123", "channel": "mesh:tasks", "event_type": "TASK_TRANSITION", "action": "CREATE", "data": {}}`),
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Missing agent_id",
			method:         http.MethodPost,
			body:           []byte(`{"channel": "test", "event_type": "start", "data": {}}`),
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Missing channel",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "123", "event_type": "test", "data": {}}`),
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Missing data",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "123", "channel": "test", "event_type": "start"}`),
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Empty fields (still present at root)",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "", "channel": "", "event_type": "", "data": null}`),
			expectedStatus: http.StatusBadRequest, // data cannot be null it must be present json
		},
		{
			name:           "Empty string fields (still present at root)",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "", "channel": "", "event_type": "", "data": {}}`),
			expectedStatus: http.StatusOK,
		},
		{
			name:           "Invalid JSON",
			method:         http.MethodPost,
			body:           []byte(`{invalid`),
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "GET request ignores validation",
			method:         http.MethodGet,
			body:           nil,
			expectedStatus: http.StatusOK,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := httptest.NewRequest(tt.method, "/api/mesh/broadcast", bytes.NewBuffer(tt.body))
			w := httptest.NewRecorder()

			handler.ServeHTTP(w, req)

			if w.Code != tt.expectedStatus {
				t.Errorf("expected status %d, got %d for test %s", tt.expectedStatus, w.Code, tt.name)
			}
		})
	}
}
