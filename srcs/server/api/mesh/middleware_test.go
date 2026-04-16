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
			name:           "Valid payload",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "123", "channel": "mesh:tasks", "event_type": "TASK_TRANSITION", "data": {"key": "value"}}`),
			expectedStatus: http.StatusOK,
		},
		{
			name:           "Missing agent_id",
			method:         http.MethodPost,
			body:           []byte(`{"channel": "mesh:tasks", "event_type": "TASK_TRANSITION", "data": {}}`),
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Missing channel",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "123", "event_type": "TASK_TRANSITION", "data": {}}`),
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Missing event_type",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "123", "channel": "mesh:tasks", "data": {}}`),
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Missing data",
			method:         http.MethodPost,
			body:           []byte(`{"agent_id": "123", "channel": "mesh:tasks", "event_type": "TASK_TRANSITION"}`),
			expectedStatus: http.StatusBadRequest,
		},
		{
			name:           "Empty fields (still present at root)",
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
				t.Errorf("expected status %d, got %d", tt.expectedStatus, w.Code)
			}
		})
	}
}
