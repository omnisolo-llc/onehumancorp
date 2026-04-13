package agents

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	agentruntime "github.com/onehumancorp/mono/srcs/server/agents/runtime"
	"github.com/onehumancorp/mono/srcs/server/integrations"
	"github.com/onehumancorp/mono/srcs/server/integrations/plane"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type recordingLauncher struct {
	requests chan agentruntime.TaskRequest
	err      error
}

func newRecordingLauncher() *recordingLauncher {
	return &recordingLauncher{requests: make(chan agentruntime.TaskRequest, 4)}
}

func (l *recordingLauncher) LaunchTask(_ context.Context, req agentruntime.TaskRequest) error {
	l.requests <- req
	return l.err
}

func (l *recordingLauncher) DefaultRegion() string {
	return "process"
}

func TestTaskWorker_pollAndAssign(t *testing.T) {
	plane.ResetGlobalPlaneCircuitBreakerForTest()
	oldAllow := integrations.AllowLocalIPsForTesting
	integrations.AllowLocalIPsForTesting = true
	defer func() { integrations.AllowLocalIPsForTesting = oldAllow }()
	tests := []struct {
		name        string
		setupMock   func() *httptest.Server
		envSetup    func(url string)
		envTeardown func()
	}{
		{
			name: "not enabled",
			setupMock: func() *httptest.Server {
				return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					t.Fatal("should not be called")
				}))
			},
			envSetup: func(url string) {
				os.Unsetenv("PLANE_URL")
				os.Unsetenv("PLANE_API_KEY")
			},
			envTeardown: func() {},
		},
		{
			name: "list issues error",
			setupMock: func() *httptest.Server {
				return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					w.WriteHeader(http.StatusInternalServerError)
				}))
			},
			envSetup: func(url string) {
				os.Setenv("PLANE_URL", url)
				os.Setenv("PLANE_WORKSPACE", "test-ws")
				os.Setenv("PLANE_PROJECT", "test-proj")
			},
			envTeardown: func() {
				os.Unsetenv("PLANE_URL")
				os.Unsetenv("PLANE_WORKSPACE")
				os.Unsetenv("PLANE_PROJECT")
			},
		},
		{
			name: "empty issues list",
			setupMock: func() *httptest.Server {
				return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					w.WriteHeader(http.StatusOK)
					json.NewEncoder(w).Encode(map[string]interface{}{"results": []interface{}{}})
				}))
			},
			envSetup: func(url string) {
				os.Setenv("PLANE_URL", url)
				os.Setenv("PLANE_WORKSPACE", "test-ws")
				os.Setenv("PLANE_PROJECT", "test-proj")
			},
			envTeardown: func() {
				os.Unsetenv("PLANE_URL")
				os.Unsetenv("PLANE_WORKSPACE")
				os.Unsetenv("PLANE_PROJECT")
			},
		},
		{
			name: "update issue status error",
			setupMock: func() *httptest.Server {
				return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					if r.Method == http.MethodGet {
						w.WriteHeader(http.StatusOK)
						json.NewEncoder(w).Encode(map[string]interface{}{
							"results": []map[string]string{
								{"id": "issue-1", "name": "Test Issue"},
							},
						})
						return
					}
					w.WriteHeader(http.StatusInternalServerError)
				}))
			},
			envSetup: func(url string) {
				os.Setenv("PLANE_URL", url)
				os.Setenv("PLANE_WORKSPACE", "test-ws")
				os.Setenv("PLANE_PROJECT", "test-proj")
			},
			envTeardown: func() {
				os.Unsetenv("PLANE_URL")
				os.Unsetenv("PLANE_WORKSPACE")
				os.Unsetenv("PLANE_PROJECT")
			},
		},
		{
			name: "success",
			setupMock: func() *httptest.Server {
				return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
					if r.Method == http.MethodGet {
						w.WriteHeader(http.StatusOK)
						json.NewEncoder(w).Encode(map[string]interface{}{
							"results": []map[string]string{
								{"id": "issue-1", "name": "Test Issue"},
							},
						})
						return
					}
					if r.Method == http.MethodPatch {
						w.WriteHeader(http.StatusOK)
						w.Write([]byte("{}"))
						return
					}
				}))
			},
			envSetup: func(url string) {
				os.Setenv("PLANE_URL", url)
				os.Setenv("PLANE_WORKSPACE", "test-ws")
				os.Setenv("PLANE_PROJECT", "test-proj")
			},
			envTeardown: func() {
				os.Unsetenv("PLANE_URL")
				os.Unsetenv("PLANE_WORKSPACE")
				os.Unsetenv("PLANE_PROJECT")
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			mockSrv := tt.setupMock()
			defer mockSrv.Close()

			tt.envSetup(mockSrv.URL)
			defer tt.envTeardown()

			client := plane.NewClientFromEnv()
			hub := orchestration.NewHub()
			defer hub.Close()
			worker := NewTaskWorker(client, hub)

			// Doesn't return anything or error, just covering branches
			worker.pollAndAssign()
		})
	}
}

func TestTaskWorker_Start_CancelDuringSleep(t *testing.T) {
	plane.ResetGlobalPlaneCircuitBreakerForTest()
	// Simple test to exercise the context cancellation branch
	hub := orchestration.NewHub()
	defer hub.Close()
	worker := NewTaskWorker(nil, hub)
	worker.pollInterval = 10 * time.Millisecond
	ctx, cancel := context.WithCancel(context.Background())
	cancel() // cancel immediately
	worker.Start(ctx)

	// wait a bit
	time.Sleep(50 * time.Millisecond)
}

func TestTaskWorker_Start_TickerTrigger_Wait(t *testing.T) {
	plane.ResetGlobalPlaneCircuitBreakerForTest()
	oldAllow := integrations.AllowLocalIPsForTesting
	integrations.AllowLocalIPsForTesting = true
	defer func() { integrations.AllowLocalIPsForTesting = oldAllow }()
	mockSrv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(map[string]interface{}{"results": []interface{}{}})
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
	worker.pollInterval = 10 * time.Millisecond

	ctx, cancel := context.WithCancel(context.Background())
	worker.Start(ctx)

	// Fast-forward or wait manually to trigger the select
	// We'll just wait to let it trigger on ticker at least once.
	time.Sleep(25 * time.Millisecond)
	cancel()
	time.Sleep(10 * time.Millisecond)
}

func TestTaskWorker_pollAndAssign_NotEnabled(t *testing.T) {
	plane.ResetGlobalPlaneCircuitBreakerForTest()
	os.Unsetenv("PLANE_URL")
	os.Unsetenv("PLANE_API_KEY")

	client := plane.NewClientFromEnv()
	hub := orchestration.NewHub()
	defer hub.Close()
	worker := NewTaskWorker(client, hub)
	worker.pollAndAssign()
}

func TestTaskWorker_pollAndAssign_ManualTrigger(t *testing.T) {
	plane.ResetGlobalPlaneCircuitBreakerForTest()
	// Directly call to guarantee execution
	os.Unsetenv("PLANE_URL")
	os.Unsetenv("PLANE_API_KEY")

	client := plane.NewClientFromEnv()
	hub := orchestration.NewHub()
	defer hub.Close()
	worker := NewTaskWorker(client, hub)
	worker.pollAndAssign()
}

func TestTaskWorker_processIssueLaunchesBuiltinTaskForIdleAgent(t *testing.T) {
	plane.ResetGlobalPlaneCircuitBreakerForTest()
	launcher := newRecordingLauncher()
	hub := orchestration.NewHub()
	defer hub.Close()
	hub.RegisterAgent(orchestration.Agent{
		ID:           "agent-1",
		Name:         "Builder",
		Role:         "SOFTWARE_ENGINEER",
		ProviderType: string(ProviderTypeBuiltin),
		Status:       orchestration.StatusIdle,
	})

	worker := NewTaskWorker(nil, hub)
	worker.taskLauncher = launcher
	worker.processIssue(plane.Issue{ID: "issue-1", Name: "Fix failing test"})

	select {
	case req := <-launcher.requests:
		if req.AgentID != "agent-1" {
			t.Fatalf("expected launch for agent-1, got %q", req.AgentID)
		}
		if req.IssueID != "issue-1" {
			t.Fatalf("expected issue-1, got %q", req.IssueID)
		}
		if req.Description != "Fix failing test" {
			t.Fatalf("expected description to match issue name, got %q", req.Description)
		}
		if req.WorkDir == "" {
			t.Fatal("expected workdir to be populated")
		}
	case <-time.After(2 * time.Second):
		t.Fatal("timed out waiting for builtin task launch")
	}

	deadline := time.Now().Add(2 * time.Second)
	for time.Now().Before(deadline) {
		agents := hub.Agents()
		if len(agents) == 1 && agents[0].Status == orchestration.StatusIdle {
			return
		}
		time.Sleep(10 * time.Millisecond)
	}
	if agents := hub.Agents(); len(agents) != 1 || agents[0].Status != orchestration.StatusIdle {
		t.Fatalf("expected agent to return to idle, got %+v", agents)
	}
}

func TestTaskWorker_processIssueSkipsBusyAgents(t *testing.T) {
	plane.ResetGlobalPlaneCircuitBreakerForTest()
	launcher := newRecordingLauncher()
	hub := orchestration.NewHub()
	defer hub.Close()
	hub.RegisterAgent(orchestration.Agent{
		ID:           "agent-1",
		Name:         "Busy Builder",
		Role:         "SOFTWARE_ENGINEER",
		ProviderType: string(ProviderTypeBuiltin),
		Status:       orchestration.StatusActive,
	})

	worker := NewTaskWorker(nil, hub)
	worker.taskLauncher = launcher
	worker.processIssue(plane.Issue{ID: "issue-2", Name: "Do not dispatch"})

	select {
	case req := <-launcher.requests:
		t.Fatalf("unexpected launch request: %+v", req)
	case <-time.After(100 * time.Millisecond):
	}
}
