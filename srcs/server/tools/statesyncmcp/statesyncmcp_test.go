package statesyncmcp

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	_ "modernc.org/sqlite"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func setupTestDB(t *testing.T) *db.DB {
	t.Helper()
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open test database: %v", err)
	}

	provider := db.NewSqliteProvider(sqlDB)
	d := &db.DB{Provider: provider}

	_, err = d.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			updated_at TEXT DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	return d
}

func TestSyncUp(t *testing.T) {
	d := setupTestDB(t)

	// Insert test data
	_, err := d.Exec(context.Background(), "INSERT INTO shared_tasks (id, organization_id, title, status, updated_at) VALUES (?, ?, ?, ?, ?)",
		"task-1", "org-1", "Test Task 1", "PENDING", time.Now().Format(time.RFC3339))
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			t.Errorf("expected POST method, got %s", r.Method)
		}
		if r.URL.Path != "/api/v1/sync/up" {
			t.Errorf("expected /api/v1/sync/up path, got %s", r.URL.Path)
		}

		var tasks []Task
		if err := json.NewDecoder(r.Body).Decode(&tasks); err != nil {
			t.Errorf("failed to decode request body: %v", err)
		}
		if len(tasks) != 1 {
			t.Errorf("expected 1 task, got %d", len(tasks))
		}
		if tasks[0].ID != "task-1" {
			t.Errorf("expected task ID 'task-1', got '%s'", tasks[0].ID)
		}

		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	provider := NewStateSyncProvider(d, ts.URL)
	server := NewServer(provider)

	// Test without claims
	ctx := context.Background()
	req := CallToolRequest{Name: "sync_local_to_cloud"}
	resp := server.CallTool(ctx, req)
	if !resp.IsError {
		t.Errorf("expected error due to missing claims")
	}

	// Test with claims
	claims := &auth.Claims{OrganizationID: "org-1"}
	ctx = context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	resp = server.CallTool(ctx, req)
	if resp.IsError {
		t.Errorf("unexpected error: %s", resp.Content[0].Text)
	}
	if resp.Content[0].Text != "Successfully synced 1 tasks to cloud" {
		t.Errorf("unexpected result text: %s", resp.Content[0].Text)
	}
}

func TestSyncDown(t *testing.T) {
	d := setupTestDB(t)

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			t.Errorf("expected GET method, got %s", r.Method)
		}
		if r.URL.Path != "/api/v1/sync/down" {
			t.Errorf("expected /api/v1/sync/down path, got %s", r.URL.Path)
		}
		orgID := r.URL.Query().Get("org_id")
		if orgID != "org-1" {
			t.Errorf("expected org_id 'org-1', got '%s'", orgID)
		}

		tasks := []Task{
			{ID: "task-2", OrganizationID: "org-1", Title: "Cloud Task 1", Status: "DONE", UpdatedAt: time.Now()},
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(tasks)
	}))
	defer ts.Close()

	provider := NewStateSyncProvider(d, ts.URL)
	server := NewServer(provider)

	claims := &auth.Claims{OrganizationID: "org-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	req := CallToolRequest{Name: "sync_cloud_to_local"}
	resp := server.CallTool(ctx, req)

	if resp.IsError {
		t.Errorf("unexpected error: %s", resp.Content[0].Text)
	}
	if resp.Content[0].Text != "Successfully synced 1 tasks from cloud" {
		t.Errorf("unexpected result text: %s", resp.Content[0].Text)
	}

	// Verify task was inserted
	var count int
	err := d.QueryRow(context.Background(), "SELECT COUNT(*) FROM shared_tasks WHERE id = 'task-2'").Scan(&count)
	if err != nil {
		t.Fatalf("failed to query db: %v", err)
	}
	if count != 1 {
		t.Errorf("expected 1 task in db, got %d", count)
	}
}

func TestGetStatus(t *testing.T) {
	d := setupTestDB(t)
	provider := NewStateSyncProvider(d, "http://localhost")
	server := NewServer(provider)

	claims := &auth.Claims{OrganizationID: "org-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	req := CallToolRequest{Name: "get_sync_status"}
	resp := server.CallTool(ctx, req)

	if resp.IsError {
		t.Errorf("unexpected error: %s", resp.Content[0].Text)
	}
	if resp.Content[0].Text != "Standalone Mode: Ready to sync" {
		t.Errorf("unexpected result text: %s", resp.Content[0].Text)
	}
}

func TestListTools(t *testing.T) {
	d := setupTestDB(t)
	provider := NewStateSyncProvider(d, "http://localhost")
	server := NewServer(provider)

	tools := server.ListTools(context.Background())
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}
}
