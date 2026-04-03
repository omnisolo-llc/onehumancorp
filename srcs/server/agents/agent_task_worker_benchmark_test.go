package agents

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/integrations/plane"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func BenchmarkTaskWorker_pollAndDispatch(b *testing.B) {
	plane.ResetGlobalPlaneCircuitBreakerForTest()
	mockSrv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodGet {
			w.WriteHeader(http.StatusOK)
			// Return 10 issues to simulate load
			issues := make([]map[string]string, 10)
			for i := 0; i < 10; i++ {
				issues[i] = map[string]string{"id": "issue", "name": "Test Issue"}
			}
			json.NewEncoder(w).Encode(map[string]interface{}{
				"results": issues,
			})
			return
		}
		if r.Method == http.MethodPatch {
			w.WriteHeader(http.StatusOK)
			w.Write([]byte("{}"))
			return
		}
	}))
	defer mockSrv.Close()

	os.Setenv("PLANE_URL", mockSrv.URL)
	os.Setenv("PLANE_WORKSPACE", "test-ws")
	os.Setenv("PLANE_PROJECT", "test-proj")
	defer func() {
		os.Unsetenv("PLANE_URL")
		os.Unsetenv("PLANE_WORKSPACE")
		os.Unsetenv("PLANE_PROJECT")
	}()

	client := plane.NewClientFromEnv()
	hub := orchestration.NewHub()
	defer hub.Close()
	worker := NewTaskWorker(client, hub)

	// Create a buffered channel for tasks
	taskChan := make(chan plane.Issue, 100)

	go func() {
		for {
			<-taskChan
		}
	}()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		worker.pollAndDispatch(taskChan)
	}
}

func BenchmarkTaskWorker_processIssue(b *testing.B) {
	plane.ResetGlobalPlaneCircuitBreakerForTest()
	mockSrv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPatch {
			w.WriteHeader(http.StatusOK)
			w.Write([]byte("{}"))
			return
		}
	}))
	defer mockSrv.Close()

	os.Setenv("PLANE_URL", mockSrv.URL)
	os.Setenv("PLANE_WORKSPACE", "test-ws")
	os.Setenv("PLANE_PROJECT", "test-proj")
	defer func() {
		os.Unsetenv("PLANE_URL")
		os.Unsetenv("PLANE_WORKSPACE")
		os.Unsetenv("PLANE_PROJECT")
	}()

	client := plane.NewClientFromEnv()
	hub := orchestration.NewHub()
	defer hub.Close()

	// Register some agents
	for i := 0; i < 10; i++ {
		hub.RegisterAgent(orchestration.Agent{ID: "agent", Status: orchestration.StatusActive})
	}

	worker := NewTaskWorker(client, hub)

	issue := plane.Issue{
		ID:   "test-issue",
		Name: "Test Issue",
	}

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		worker.processIssue(issue)
	}
}
