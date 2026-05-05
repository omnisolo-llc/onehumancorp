package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"github.com/go-redis/redis/v8"
)

type MockRedisClient struct{}
func (m *MockRedisClient) Publish(ctx context.Context, channel string, message interface{}) *redis.IntCmd {
	return redis.NewIntResult(1, nil)
}

type MockAuthStore struct{}
func (m *MockAuthStore) ValidateToken(token string) bool {
	return token == "valid"
}

func TestBroadcastHandler(t *testing.T) {
	mesh, _ := NewTeammateMesh("redis://localhost")
	mesh.redisClient = &MockRedisClient{}

	payload := BroadcastPayload{
		AgentID: "agent-123",
		Action:  "update",
		Status:  "ok",
	}
	body, _ := json.Marshal(payload)

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer(body))
	w := httptest.NewRecorder()
	mesh.BroadcastHandler(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("Expected 200, got %d", w.Code)
	}

	reqBad := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer([]byte(`{}`)))
	wBad := httptest.NewRecorder()
	mesh.BroadcastHandler(wBad, reqBad)

	if wBad.Code != http.StatusBadRequest {
		t.Errorf("Expected 400, got %d", wBad.Code)
	}
}

func TestAuthMiddleware(t *testing.T) {
	authStore := &MockAuthStore{}
	authMw := NewAuthMiddleware(authStore)
	handler := authMw.Middleware(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	})

	req := httptest.NewRequest(http.MethodPost, "/", nil)
	w := httptest.NewRecorder()
	handler(w, req)

	if w.Code != http.StatusUnauthorized {
		t.Errorf("Expected 401, got %d", w.Code)
	}

	req.Header.Set("Authorization", "Bearer valid")
	w = httptest.NewRecorder()
	handler(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("Expected 200, got %d", w.Code)
	}

	req.Header.Set("Authorization", "Bearer invalid")
	w = httptest.NewRecorder()
	handler(w, req)

	if w.Code != http.StatusUnauthorized {
		t.Errorf("Expected 401, got %d", w.Code)
	}
}
