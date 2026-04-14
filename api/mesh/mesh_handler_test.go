package mesh

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHandleBroadcast(t *testing.T) {
	svc := NewMemoryMeshService()
	handler := HandleBroadcast(svc)

	tests := []struct {
		name       string
		claims     *auth.Claims
		payload    interface{}
		wantStatus int
	}{
		{
			name:       "unauthorized",
			claims:     nil,
			payload:    BroadcastRequest{AgentID: "1", Action: "test", Status: "ok"},
			wantStatus: http.StatusUnauthorized,
		},
		{
			name:       "invalid body",
			claims:     &auth.Claims{OrganizationID: "org1"},
			payload:    "not a json",
			wantStatus: http.StatusBadRequest,
		},
		{
			name:       "missing fields",
			claims:     &auth.Claims{OrganizationID: "org1"},
			payload:    BroadcastRequest{AgentID: "1"}, // missing Action and Status
			wantStatus: http.StatusBadRequest,
		},
		{
			name:       "success",
			claims:     &auth.Claims{OrganizationID: "org1"},
			payload:    BroadcastRequest{AgentID: "1", Action: "test", Status: "ok"},
			wantStatus: http.StatusOK,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			body, _ := json.Marshal(tt.payload)
			req := httptest.NewRequest("POST", "/api/mesh/broadcast", bytes.NewReader(body))

			if tt.claims != nil {
				ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, tt.claims)
				req = req.WithContext(ctx)
			}

			w := httptest.NewRecorder()
			handler.ServeHTTP(w, req)

			if w.Code != tt.wantStatus {
				t.Errorf("got status %d, want %d", w.Code, tt.wantStatus)
			}
		})
	}
}
