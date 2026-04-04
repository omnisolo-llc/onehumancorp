package dashboard

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleMeshBroadcast(t *testing.T) {
	pool, err := db.New(context.Background())
	if err != nil {
		t.Fatalf("failed to open test db: %v", err)
	}
	defer pool.Close()

	if err := pool.RunMigrations(context.Background()); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	authStore := auth.NewStore(pool.Provider, "test-secret")
	hubService, err := orchestration.NewHubService(pool.Provider, "dummy-minimax-key")
	if err != nil {
		t.Fatalf("failed to init hub service: %v", err)
	}

	server := NewServer(hubService, authStore, "dist")
	mux := server.SetupRoutes()

	tests := []struct {
		name       string
		method     string
		body       string
		wantStatus int
	}{
		{
			name:       "valid request",
			method:     http.MethodPost,
			body:       `{"channel": "mesh:tasks", "agent_id": "agent-1", "action": "CLAIM", "status": "IN_PROGRESS"}`,
			wantStatus: http.StatusOK,
		},
		{
			name:       "invalid method",
			method:     http.MethodGet,
			body:       `{"channel": "mesh:tasks"}`,
			wantStatus: http.StatusMethodNotAllowed,
		},
		{
			name:       "invalid channel",
			method:     http.MethodPost,
			body:       `{"channel": "mesh:invalid", "agent_id": "agent-1", "action": "CLAIM", "status": "IN_PROGRESS"}`,
			wantStatus: http.StatusBadRequest,
		},
		{
			name:       "invalid json",
			method:     http.MethodPost,
			body:       `{"channel": "mesh:tasks", `,
			wantStatus: http.StatusBadRequest,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req, err := http.NewRequest(tt.method, "/api/mesh/broadcast", bytes.NewBufferString(tt.body))
			if err != nil {
				t.Fatalf("failed to create request: %v", err)
			}
			req.Header.Set("Content-Type", "application/json")

			rr := httptest.NewRecorder()
			mux.ServeHTTP(rr, req)

			if status := rr.Code; status != tt.wantStatus {
				t.Errorf("handler returned wrong status code: got %v want %v", status, tt.wantStatus)
			}
		})
	}
}
