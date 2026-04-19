package tasks

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestRouter(t *testing.T) (*Router, *sql.DB) {
	database, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open db: %v", err)
	}

	_, err = database.Exec(`
		CREATE TABLE shared_tasks (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assignee TEXT,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(database)
	q := NewQueue(provider)
	return NewRouter(q), database
}

func TestRouter_AddHandler(t *testing.T) {
	router, database := setupTestRouter(t)
	defer database.Close()

	task := &Task{
		OrganizationID: "org-1",
		Title:          "Test Task",
		Description:    "Test Description",
	}
	body, _ := json.Marshal(task)

	req := httptest.NewRequest(http.MethodPost, "/api/tasks/add", bytes.NewReader(body))
	w := httptest.NewRecorder()

	router.AddHandler(w, req)

	if w.Code != http.StatusCreated {
		t.Fatalf("Expected status %d, got %d", http.StatusCreated, w.Code)
	}

	var res Task
	if err := json.NewDecoder(w.Body).Decode(&res); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}
	if res.ID == "" {
		t.Fatalf("Expected non-empty ID")
	}
	if res.Status != "PENDING" {
		t.Fatalf("Expected status PENDING, got %s", res.Status)
	}
}

func TestRouter_ClaimHandler(t *testing.T) {
	router, database := setupTestRouter(t)
	defer database.Close()

	// Add a task directly to queue to test claiming
	task := &Task{
		OrganizationID: "org-2",
		Title:          "Claim Task",
		Description:    "Claim Description",
	}
	provider := db.NewSqliteProvider(database)
	q := NewQueue(provider)
	q.AddTask(context.Background(), task)

	reqBody := map[string]string{
		"organization_id": "org-2",
		"agent_id":        "agent-1",
	}
	body, _ := json.Marshal(reqBody)

	req := httptest.NewRequest(http.MethodPost, "/api/tasks/claim", bytes.NewReader(body))
	w := httptest.NewRecorder()

	router.ClaimHandler(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("Expected status %d, got %d", http.StatusOK, w.Code)
	}

	var res Task
	if err := json.NewDecoder(w.Body).Decode(&res); err != nil {
		t.Fatalf("Failed to decode response: %v", err)
	}
	if res.Status != "IN_PROGRESS" {
		t.Fatalf("Expected status IN_PROGRESS, got %s", res.Status)
	}
	if res.Assignee != "agent-1" {
		t.Fatalf("Expected assignee agent-1, got %s", res.Assignee)
	}

	// Test claiming when no tasks available
	req2 := httptest.NewRequest(http.MethodPost, "/api/tasks/claim", bytes.NewReader(body))
	w2 := httptest.NewRecorder()
	router.ClaimHandler(w2, req2)

	if w2.Code != http.StatusNoContent {
		t.Fatalf("Expected status %d, got %d", http.StatusNoContent, w2.Code)
	}
}
