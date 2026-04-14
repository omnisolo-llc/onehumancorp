package webhooks

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestReceiver_OHCSIPCompliance(t *testing.T) {
	tests := []struct {
		name       string
		payload    []byte
		statusCode int
	}{
		{
			name:       "Valid Payload",
			payload:    []byte(`{"agent_id": "agent-1", "action": "task_update", "status": "IN_PROGRESS"}`),
			statusCode: http.StatusOK,
		},
		{
			name:       "Missing agent_id",
			payload:    []byte(`{"action": "task_update", "status": "IN_PROGRESS"}`),
			statusCode: http.StatusBadRequest,
		},
		{
			name:       "Missing action",
			payload:    []byte(`{"agent_id": "agent-1", "status": "IN_PROGRESS"}`),
			statusCode: http.StatusBadRequest,
		},
		{
			name:       "Missing status",
			payload:    []byte(`{"agent_id": "agent-1", "action": "task_update"}`),
			statusCode: http.StatusBadRequest,
		},
		{
			name:       "Empty payload",
			payload:    []byte(`{}`),
			statusCode: http.StatusBadRequest,
		},
		{
			name:       "Empty string fields still valid format",
			payload:    []byte(`{"agent_id": "", "action": "", "status": ""}`),
			statusCode: http.StatusOK,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req, err := http.NewRequest("POST", "/webhook", bytes.NewBuffer(tt.payload))
			if err != nil {
				t.Fatal(err)
			}
			req.Header.Set("Content-Type", "application/json")

			rr := httptest.NewRecorder()
			handler := http.HandlerFunc(Receiver)

			handler.ServeHTTP(rr, req)

			if status := rr.Code; status != tt.statusCode {
				t.Errorf("handler returned wrong status code: got %v want %v", status, tt.statusCode)
			}
		})
	}
}
