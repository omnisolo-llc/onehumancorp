1. sed -i 's/^status: PENDING/status: IN_PROGRESS\nagent: Implementer/' .agent-task/missions/2026-04-12T12-00-00Z.md && cat .agent-task/missions/2026-04-12T12-00-00Z.md | head -n 5
2. cat << 'INNER_EOF' > srcs/server/db/migrations/033_shared_tasks_schema.sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload JSONB,
    parent_plan_id TEXT,
    dependencies JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
INNER_EOF
3. cat srcs/server/db/migrations/033_shared_tasks_schema.sql
4. cat << 'INNER_EOF' > srcs/server/orchestration/handlers.go
package orchestration

import (
	"encoding/json"
	"net/http"
	"strconv"

	"github.com/onehumancorp/ohc/srcs/server/auth"
)

type TaskRequest struct {
	Title        string   `json:"title"`
	Description  string   `json:"description"`
	Priority     string   `json:"priority"`
	Dependencies []string `json:"dependencies"`
}

type ClaimRequest struct {
	AgentID string `json:"agent_id"`
}

func CreateTaskHandler(tm *TaskManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		var req TaskRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Bad Request", http.StatusBadRequest)
			return
		}

		task, err := tm.CreateTaskWithPlan(r.Context(), claims.OrganizationID, req.Dependencies, req.Title, req.Description, req.Priority)
		if err != nil {
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusCreated)
		json.NewEncoder(w).Encode(task)
	}
}

func ListTasksHandler(tm *TaskManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		limitStr := r.URL.Query().Get("limit")
		limit := 10
		if limitStr != "" {
			if parsed, err := strconv.Atoi(limitStr); err == nil && parsed > 0 {
				limit = parsed
			}
		}

		tasks, err := tm.PeekTasks(r.Context(), limit)
		if err != nil {
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(tasks)
	}
}

func ClaimTaskHandler(tm *TaskManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		var req ClaimRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Bad Request", http.StatusBadRequest)
			return
		}

		taskID := r.URL.Query().Get("task_id")
		if taskID == "" {
			http.Error(w, "Missing task_id", http.StatusBadRequest)
			return
		}

		task, err := tm.ClaimTask(r.Context(), taskID, req.AgentID)
		if err != nil {
			http.Error(w, "Conflict", http.StatusConflict)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(task)
	}
}

func CompleteTaskHandler(tm *TaskManager) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		taskID := r.URL.Query().Get("task_id")
		agentID := r.URL.Query().Get("agent_id")
		if taskID == "" || agentID == "" {
			http.Error(w, "Missing task_id or agent_id", http.StatusBadRequest)
			return
		}

		err := tm.CompleteTask(r.Context(), taskID, agentID)
		if err != nil {
			http.Error(w, "Conflict", http.StatusConflict)
			return
		}

		w.WriteHeader(http.StatusOK)
	}
}
INNER_EOF
5. cat srcs/server/orchestration/handlers.go | head -n 15
6. cat << 'INNER_EOF' > srcs/server/orchestration/handlers_test.go
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
}
INNER_EOF
7. cat srcs/server/orchestration/handlers_test.go | head -n 15
8. sed -i 's/^status: IN_PROGRESS/status: DONE/' .agent-task/missions/2026-04-12T12-00-00Z.md && cat .agent-task/missions/2026-04-12T12-00-00Z.md | head -n 5
9. ~/go/bin/bazelisk test //srcs/server/orchestration/...
