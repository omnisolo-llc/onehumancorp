package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/integrations/plane"
)

func setupMockPlaneClient(t *testing.T, handler http.HandlerFunc) *plane.Client {
	t.Helper()

	server := httptest.NewServer(handler)
	t.Cleanup(func() {
		server.Close()
	})

	t.Setenv("PLANE_URL", server.URL)
	t.Setenv("PLANE_API_KEY", "test-key")
	t.Setenv("PLANE_WORKSPACE", "test-workspace")
	t.Setenv("PLANE_PROJECT", "test-project")

	return plane.NewClientFromEnv()
}

func TestTaskWorker_pollAndAssign(t *testing.T) {
	tests := []struct {
		name          string
		handler       http.HandlerFunc
		planeDisabled bool
	}{
		{
			name: "happy path",
			handler: func(w http.ResponseWriter, r *http.Request) {
				if r.Method == http.MethodGet && r.URL.Path == "/api/v1/workspaces/test-workspace/projects/test-project/issues/" {
					w.WriteHeader(http.StatusOK)
					w.Write([]byte(`{"results": [{"id": "123", "name": "Task"}]}`))
					return
				}
				if r.Method == http.MethodPatch && r.URL.Path == "/api/v1/workspaces/test-workspace/projects/test-project/issues/123/" {
					w.WriteHeader(http.StatusOK)
					return
				}
				w.WriteHeader(http.StatusNotFound)
			},
			planeDisabled: false,
		},
		{
			name: "plane disabled",
			handler: func(w http.ResponseWriter, r *http.Request) {
				t.Error("Handler should not be called when Plane is disabled")
			},
			planeDisabled: true,
		},
		{
			name: "no issues",
			handler: func(w http.ResponseWriter, r *http.Request) {
				if r.Method == http.MethodGet {
					w.WriteHeader(http.StatusOK)
					w.Write([]byte(`{"results": []}`))
					return
				}
				t.Error("Should not make any other requests")
			},
			planeDisabled: false,
		},
		{
			name: "list error",
			handler: func(w http.ResponseWriter, r *http.Request) {
				if r.Method == http.MethodGet {
					w.WriteHeader(http.StatusInternalServerError)
					return
				}
				t.Error("Should not make any other requests")
			},
			planeDisabled: false,
		},
		{
			name: "update error",
			handler: func(w http.ResponseWriter, r *http.Request) {
				if r.Method == http.MethodGet {
					w.WriteHeader(http.StatusOK)
					w.Write([]byte(`{"results": [{"id": "123", "name": "Task"}]}`))
					return
				}
				if r.Method == http.MethodPatch {
					w.WriteHeader(http.StatusInternalServerError)
					return
				}
				w.WriteHeader(http.StatusNotFound)
			},
			planeDisabled: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.planeDisabled {
				// We don't use setupMockPlaneClient because it sets env vars.
				// For the disabled test, we need them to be completely unset.
				// t.Setenv allows us to temporarily set or unset. Setting to empty string is equivalent to unsetting for os.Getenv.
				t.Setenv("PLANE_URL", "")
				t.Setenv("PLANE_API_KEY", "")
				pc := &plane.Client{} // Dummy client
				tw := NewTaskWorker(pc)
				tw.pollAndAssign() // Should return immediately
			} else {
				client := setupMockPlaneClient(t, tt.handler)
				tw := NewTaskWorker(client)
				tw.pollAndAssign()
			}
		})
	}
}

func TestTaskWorker_Start(t *testing.T) {
	pc := &plane.Client{}
	tw := NewTaskWorker(pc)

	ctx, cancel := context.WithCancel(context.Background())
	tw.Start(ctx, 10*time.Millisecond)

	// Cancel immediately to trigger the ctx.Done() path
	cancel()

	// Wait a small amount of time to allow the goroutine to select ctx.Done() and return
	time.Sleep(10 * time.Millisecond)
}

func TestTaskWorker_Start_Ticker(t *testing.T) {
	// Setup mock to cover line 70 and 34-35
	handler := func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodGet && r.URL.Path == "/api/v1/workspaces/test-workspace/projects/test-project/issues/" {
			w.WriteHeader(http.StatusOK)
			w.Write([]byte(`{"results": [{"id": "123", "name": "Task"}]}`))
			return
		}
		if r.Method == http.MethodPatch && r.URL.Path == "/api/v1/workspaces/test-workspace/projects/test-project/issues/123/" {
			w.WriteHeader(http.StatusOK)
			return
		}
		w.WriteHeader(http.StatusNotFound)
	}
	client := setupMockPlaneClient(t, handler)

	tw := NewTaskWorker(client)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	tw.Start(ctx, 10*time.Millisecond)

	// Wait a bit more than the 10ms ticker to ensure the loop runs at least once
	time.Sleep(20 * time.Millisecond)
}

func TestTaskWorker_Start_DefaultTicker(t *testing.T) {
	pc := &plane.Client{}
	tw := NewTaskWorker(pc)

	ctx, cancel := context.WithCancel(context.Background())
	tw.Start(ctx, 0) // Should default to 30s

	// Cancel immediately
	cancel()

	time.Sleep(10 * time.Millisecond)
}
