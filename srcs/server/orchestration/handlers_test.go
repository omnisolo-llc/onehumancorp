package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/ohc/srcs/server/auth"
	"github.com/onehumancorp/ohc/srcs/server/db"
)

func TestHandlers(t *testing.T) {
	provider, _ := db.NewSQLiteProvider(":memory:")
	tm := NewTaskManager(provider, nil)

	t.Run("CreateTaskHandler", func(t *testing.T) {
		reqBody := TaskRequest{Title: "Test Task", Description: "Test Desc", Priority: "P1"}
		bodyBytes, _ := json.Marshal(reqBody)
		req := httptest.NewRequest(http.MethodPost, "/tasks", bytes.NewReader(bodyBytes))

		ctx := auth.ContextWithClaims(req.Context(), &auth.Claims{OrganizationID: "org-1", UserID: "user-1"})
		req = req.WithContext(ctx)

		rr := httptest.NewRecorder()
		handler := CreateTaskHandler(tm)
		handler.ServeHTTP(rr, req)

		if rr.Code != http.StatusCreated {
			t.Errorf("Expected status 201, got %v", rr.Code)
		}
	})

	t.Run("ListTasksHandler", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/tasks?limit=5", nil)
		ctx := auth.ContextWithClaims(req.Context(), &auth.Claims{OrganizationID: "org-1", UserID: "user-1"})
		req = req.WithContext(ctx)

		rr := httptest.NewRecorder()
		handler := ListTasksHandler(tm)
		handler.ServeHTTP(rr, req)

		if rr.Code != http.StatusOK {
			t.Errorf("Expected status 200, got %v", rr.Code)
		}
	})

	t.Run("ClaimTaskHandler_MissingTaskID", func(t *testing.T) {
		reqBody := ClaimRequest{AgentID: "agent-1"}
		bodyBytes, _ := json.Marshal(reqBody)
		req := httptest.NewRequest(http.MethodPost, "/tasks/claim", bytes.NewReader(bodyBytes))

		ctx := auth.ContextWithClaims(req.Context(), &auth.Claims{OrganizationID: "org-1", UserID: "user-1"})
		req = req.WithContext(ctx)

		rr := httptest.NewRecorder()
		handler := ClaimTaskHandler(tm)
		handler.ServeHTTP(rr, req)

		if rr.Code != http.StatusBadRequest {
			t.Errorf("Expected status 400, got %v", rr.Code)
		}
	})

	t.Run("CompleteTaskHandler_MissingParams", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPost, "/tasks/complete", nil)
		ctx := auth.ContextWithClaims(req.Context(), &auth.Claims{OrganizationID: "org-1", UserID: "user-1"})
		req = req.WithContext(ctx)

		rr := httptest.NewRecorder()
		handler := CompleteTaskHandler(tm)
		handler.ServeHTTP(rr, req)

		if rr.Code != http.StatusBadRequest {
			t.Errorf("Expected status 400, got %v", rr.Code)
		}
	})

	t.Run("Unauthorized", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/tasks", nil)
		rr := httptest.NewRecorder()
		handler := ListTasksHandler(tm)
		handler.ServeHTTP(rr, req)

		if rr.Code != http.StatusUnauthorized {
			t.Errorf("Expected status 401, got %v", rr.Code)
		}
	})

	t.Run("UpdateTaskHandler_MissingTaskID", func(t *testing.T) {
		reqBody := TaskRequest{Title: "Test Task"}
		bodyBytes, _ := json.Marshal(reqBody)
		req := httptest.NewRequest(http.MethodPost, "/tasks/update", bytes.NewReader(bodyBytes))

		ctx := auth.ContextWithClaims(req.Context(), &auth.Claims{OrganizationID: "org-1", UserID: "user-1"})
		req = req.WithContext(ctx)

		rr := httptest.NewRecorder()
		handler := UpdateTaskHandler(tm)
		handler.ServeHTTP(rr, req)

		if rr.Code != http.StatusBadRequest {
			t.Errorf("Expected status 400, got %v", rr.Code)
		}
	})
}
