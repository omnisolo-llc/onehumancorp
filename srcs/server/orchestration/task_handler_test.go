package orchestration

import (
	"bytes"
	"context"
	"database/sql"

	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gorilla/mux"
	_ "github.com/mattn/go-sqlite3"
)

func TestCreateTaskEndpoint(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open database: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE ohc_tasks (
			id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			priority INTEGER DEFAULT 0,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	repo := NewTaskRepository(db)
	mesh := &LocalMesh{}
	handler := NewTaskHandler(repo, mesh)

	router := mux.NewRouter()
	handler.RegisterRoutes(router)

	payload := []byte(`{"title": "Test Task", "description": "This is a test task"}`)
	req, err := http.NewRequest("POST", "/tasks", bytes.NewBuffer(payload))
	if err != nil {
		t.Fatalf("Failed to create request: %v", err)
	}

	rr := httptest.NewRecorder()
	router.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusCreated {
		t.Errorf("Handler returned wrong status code: got %v want %v", status, http.StatusCreated)
	}
}

func TestClaimTaskEndpoint(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open database: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE ohc_tasks (
			id TEXT PRIMARY KEY,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			priority INTEGER DEFAULT 0,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	repo := NewTaskRepository(db)
	mesh := &LocalMesh{}
	handler := NewTaskHandler(repo, mesh)

	router := mux.NewRouter()
	handler.RegisterRoutes(router)

	task := &Task{Title: "Test Task"}
	err = repo.CreateTask(context.Background(), task)
	if err != nil {
		t.Fatalf("Failed to create task: %v", err)
	}

	payload := []byte(`{"agent_id": "agent-1"}`)
	req, err := http.NewRequest("POST", "/tasks/"+task.ID+"/claim", bytes.NewBuffer(payload))
	if err != nil {
		t.Fatalf("Failed to create request: %v", err)
	}

	rr := httptest.NewRecorder()
	router.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("Handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}
