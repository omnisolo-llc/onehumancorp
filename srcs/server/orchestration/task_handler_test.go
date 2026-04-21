package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	pb "github.com/onehumancorp/mono/srcs/proto"
)

type mockMesh struct {}
func (m *mockMesh) Start(ctx context.Context) error { return nil }
func (m *mockMesh) Stop() error { return nil }
func (m *mockMesh) Publish(ctx context.Context, channel string, message []byte) error { return nil }
func (m *mockMesh) BroadcastTask(ctx context.Context, task Task) error { return nil }
func (m *mockMesh) SubscribeTasks(ctx context.Context) (<-chan Task, error) { return nil, nil }
func (m *mockMesh) BroadcastCoordination(ctx context.Context, msg MeshMessage) error { return nil }
func (m *mockMesh) SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error) { return nil, nil }
func (m *mockMesh) AdvertiseCapabilities(ctx context.Context, caps pb.AgentCapabilities) error { return nil }
func (m *mockMesh) DiscoverAgents(ctx context.Context) ([]pb.AgentCapabilities, error) { return nil, nil }
func (m *mockMesh) SubscribeCapabilities(ctx context.Context) (<-chan pb.AgentCapabilities, error) { return nil, nil }
func (m *mockMesh) SubscribeMeshEvents(ctx context.Context, channel string) (<-chan []byte, error) { return nil, nil }
func (m *mockMesh) BroadcastMeshEvent(ctx context.Context, channel string, message []byte) error { return nil }


func TestTaskHandler_handleTasks(t *testing.T) {
	provider := newTaskRepositoryTestProvider(t)
	mesh := &mockMesh{}
	handler := NewTaskHandler(provider, mesh)

	mux := http.NewServeMux()
	handler.RegisterRoutes(mux)

	// Test creating a task
	body := map[string]interface{}{
		"title":       "Test API Task",
		"description": "Created via API",
		"priority":    2,
	}
	bodyBytes, _ := json.Marshal(body)

	req := httptest.NewRequest(http.MethodPost, "/api/tasks/shared", bytes.NewBuffer(bodyBytes))
	w := httptest.NewRecorder()

	mux.ServeHTTP(w, req)

	if w.Code != http.StatusCreated {
		t.Fatalf("expected status 201, got %d", w.Code)
	}

	var task TaskEntity
	if err := json.NewDecoder(w.Body).Decode(&task); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if task.Title != "Test API Task" {
		t.Fatalf("expected title 'Test API Task', got '%s'", task.Title)
	}

	// Test listing tasks
	reqList := httptest.NewRequest(http.MethodGet, "/api/tasks/shared", nil)
	wList := httptest.NewRecorder()

	mux.ServeHTTP(wList, reqList)

	if wList.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wList.Code)
	}

	var tasks []TaskEntity
	if err := json.NewDecoder(wList.Body).Decode(&tasks); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if len(tasks) != 1 {
		t.Fatalf("expected 1 task, got %d", len(tasks))
	}
	if tasks[0].ID != task.ID {
		t.Fatalf("expected task ID %s, got %s", task.ID, tasks[0].ID)
	}

	// Test claiming task
	claimBody := map[string]interface{}{
		"task_id":  task.ID,
		"agent_id": "agent-xyz",
	}
	claimBytes, _ := json.Marshal(claimBody)

	reqClaim := httptest.NewRequest(http.MethodPost, "/api/tasks/shared/claim", bytes.NewBuffer(claimBytes))
	wClaim := httptest.NewRecorder()

	mux.ServeHTTP(wClaim, reqClaim)

	if wClaim.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wClaim.Code)
	}

	var claimResp map[string]interface{}
	if err := json.NewDecoder(wClaim.Body).Decode(&claimResp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if claimed, ok := claimResp["claimed"].(bool); !ok || !claimed {
		t.Fatalf("expected claimed to be true, got %v", claimResp["claimed"])
	}
}
