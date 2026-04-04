package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type mockTeammateMesh struct {
	tasks        []Task
	coordinations []MeshMessage
}

func (m *mockTeammateMesh) BroadcastTask(ctx context.Context, task Task) error {
	m.tasks = append(m.tasks, task)
	return nil
}

func (m *mockTeammateMesh) SubscribeTasks(ctx context.Context) (<-chan Task, error) {
	return nil, nil
}

func (m *mockTeammateMesh) BroadcastCoordination(ctx context.Context, msg MeshMessage) error {
	m.coordinations = append(m.coordinations, msg)
	return nil
}

func (m *mockTeammateMesh) SubscribeCoordination(ctx context.Context) (<-chan MeshMessage, error) {
	return nil, nil
}

func TestMeshAPI_BroadcastCoordination(t *testing.T) {
	mesh := &mockTeammateMesh{}
	mux := http.NewServeMux()
	RegisterMeshHTTPHandlers(mux, mesh, nil)

	msg := MeshMessage{
		AgentID: "agent-123",
		Action:  "update",
		Status:  "ok",
		Role:    "tester",
	}
	body, _ := json.Marshal(msg)

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewReader(body))
	// Add "system" role to the context for auth middleware
	claims := &auth.Claims{}
	claims.Roles = []string{"system"}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("Expected status 200 OK, got %v: %s", rr.Code, rr.Body.String())
	}

	if len(mesh.coordinations) != 1 {
		t.Errorf("Expected 1 coordination message, got %d", len(mesh.coordinations))
	} else {
		if mesh.coordinations[0].AgentID != "agent-123" {
			t.Errorf("Expected agent-123, got %s", mesh.coordinations[0].AgentID)
		}
	}
}

func TestMeshAPI_BroadcastTask(t *testing.T) {
	mesh := &mockTeammateMesh{}
	mux := http.NewServeMux()
	RegisterMeshHTTPHandlers(mux, mesh, nil)

	task := Task{
		AgentID: "agent-tasker",
		Action:  "start",
		Status:  "running",
		TaskID:  "task-789",
	}
	body, _ := json.Marshal(task)

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/tasks", bytes.NewReader(body))
	claims := &auth.Claims{}
	claims.Roles = []string{"system"}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("Expected status 200 OK, got %v: %s", rr.Code, rr.Body.String())
	}

	if len(mesh.tasks) != 1 {
		t.Errorf("Expected 1 task, got %d", len(mesh.tasks))
	} else {
		if mesh.tasks[0].TaskID != "task-789" {
			t.Errorf("Expected task-789, got %s", mesh.tasks[0].TaskID)
		}
	}
}

func TestMeshAPI_MissingFields(t *testing.T) {
	mesh := &mockTeammateMesh{}
	mux := http.NewServeMux()
	RegisterMeshHTTPHandlers(mux, mesh, nil)

	// Missing status
	msg := MeshMessage{
		AgentID: "agent-123",
		Action:  "update",
	}
	body, _ := json.Marshal(msg)

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewReader(body))
	claims := &auth.Claims{}
	claims.Roles = []string{"system"}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if rr.Code != http.StatusBadRequest {
		t.Errorf("Expected status 400 Bad Request, got %v: %s", rr.Code, rr.Body.String())
	}
}

func TestMeshAPI_Unauthorized(t *testing.T) {
	mesh := &mockTeammateMesh{}
	mux := http.NewServeMux()
	RegisterMeshHTTPHandlers(mux, mesh, nil)

	msg := MeshMessage{
		AgentID: "agent-123",
		Action:  "update",
		Status:  "ok",
	}
	body, _ := json.Marshal(msg)

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewReader(body))
	// NOT setting system role
	claims := &auth.Claims{}
	claims.Roles = []string{"user"}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if rr.Code != http.StatusForbidden {
		t.Errorf("Expected status 403 Forbidden, got %v: %s", rr.Code, rr.Body.String())
	}
}
