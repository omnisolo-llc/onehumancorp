package orchestration

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandleMeshBroadcast(t *testing.T) {
	hubService := &HubServiceServer{}

	tests := []struct {
		name       string
		method     string
		body       MeshBroadcastRequest
		statusCode int
	}{
		{
			name:       "Wrong Method",
			method:     http.MethodGet,
			statusCode: http.StatusMethodNotAllowed,
		},
		{
			name:       "Missing Fields",
			method:     http.MethodPost,
			body:       MeshBroadcastRequest{},
			statusCode: http.StatusBadRequest,
		},
		{
			name:   "Invalid Channel",
			method: http.MethodPost,
			body: MeshBroadcastRequest{
				AgentID: "agent1",
				Action:  "CLAIM",
				Status:  "IN_PROGRESS",
				Channel: "invalid:channel",
			},
			statusCode: http.StatusBadRequest,
		},
		{
			name:   "Valid Request",
			method: http.MethodPost,
			body: MeshBroadcastRequest{
				AgentID: "agent1",
				Action:  "CLAIM",
				Status:  "IN_PROGRESS",
				Channel: "mesh:tasks",
				Payload: json.RawMessage(`{"extra":"data"}`),
			},
			statusCode: http.StatusOK,
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			var bodyBytes []byte
			if tc.method == http.MethodPost {
				bodyBytes, _ = json.Marshal(tc.body)
			}
			req := httptest.NewRequest(tc.method, "/api/mesh/broadcast", bytes.NewBuffer(bodyBytes))
			rr := httptest.NewRecorder()

			hubService.HandleMeshBroadcast(rr, req)

			if rr.Code != tc.statusCode {
				t.Errorf("expected status %d, got %d", tc.statusCode, rr.Code)
			}
		})
	}
}
