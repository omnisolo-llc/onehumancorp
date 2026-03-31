package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/integrations/plane"
)

func TestTaskWorker_StartAndPoll(t *testing.T) {
	// 1. Setup Mock Server for Plane API
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		if r.URL.Path == "/api/v1/workspaces/dummy-workspace/projects/dummy-project/issues/" {
			response := `{
				"results": [
					{
						"id": "test-issue-1",
						"name": "Test Issue",
						"state_detail": {
							"name": "Backlog",
							"group": "backlog"
						}
					}
				]
			}`
			w.Write([]byte(response))
		} else {
			// update issue status mock
			w.Write([]byte(`{}`))
		}
	}))
	defer ts.Close()

	os.Setenv("PLANE_API_URL", ts.URL)
	os.Setenv("PLANE_API_KEY", "dummy-key")
	os.Setenv("PLANE_WORKSPACE_SLUG", "dummy-workspace")
	os.Setenv("PLANE_PROJECT_ID", "dummy-project")
	defer func() {
		os.Unsetenv("PLANE_API_URL")
		os.Unsetenv("PLANE_API_KEY")
		os.Unsetenv("PLANE_WORKSPACE_SLUG")
		os.Unsetenv("PLANE_PROJECT_ID")
	}()

	pc := plane.NewClientFromEnv()

	tw := NewTaskWorker(pc)
	// Override the pollInterval for testing to avoid 30 seconds wait
	tw.pollInterval = 10 * time.Millisecond

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	tw.Start(ctx)

	// Wait enough time for at least one tick to occur
	time.Sleep(50 * time.Millisecond)
}
