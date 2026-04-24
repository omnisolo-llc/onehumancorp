package mesh

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestAutoDreamHandler(t *testing.T) {
	ctx := context.Background()
	provider := db.NewTestProvider(t)

	// Create table
	_, err := provider.Exec(ctx, `
		CREATE TABLE autodream_findings (
			id TEXT PRIMARY KEY,
			timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			content TEXT NOT NULL,
			embedding TEXT
		)
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := db.NewAutoDreamRepository(provider)
	handler := NewAutoDreamHandler(repo)

	// Test Store
	finding := db.Finding{
		ID:        "f1",
		Timestamp: time.Now(),
		Content:   "Mesh Architecture",
		Embedding: []float32{1.0, 0.0, 0.0},
	}
	body, _ := json.Marshal(finding)
	req := httptest.NewRequest(http.MethodPost, "/api/autodream/store", bytes.NewBuffer(body))
	w := httptest.NewRecorder()

	handler.Store(w, req)

	if w.Code != http.StatusCreated {
		t.Errorf("expected status %d, got %d", http.StatusCreated, w.Code)
	}

	// Test Search
	searchReq := struct {
		Embedding []float32 `json:"embedding"`
		Limit     int       `json:"limit"`
	}{
		Embedding: []float32{1.0, 0.0, 0.0},
		Limit:     1,
	}
	body, _ = json.Marshal(searchReq)
	req = httptest.NewRequest(http.MethodPost, "/api/autodream/search", bytes.NewBuffer(body))
	w = httptest.NewRecorder()

	handler.Search(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status %d, got %d", http.StatusOK, w.Code)
	}

	var results []*db.Finding
	if err := json.NewDecoder(w.Body).Decode(&results); err != nil {
		t.Fatalf("failed to decode results: %v", err)
	}

	if len(results) != 1 {
		t.Fatalf("expected 1 result, got %d", len(results))
	}

	if results[0].ID != "f1" {
		t.Errorf("expected finding id f1, got %s", results[0].ID)
	}
}
