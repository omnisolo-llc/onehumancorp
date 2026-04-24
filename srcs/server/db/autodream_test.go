package db

import (
	"context"
	"testing"
	"time"
)

func TestAutoDreamRepository(t *testing.T) {
	ctx := context.Background()
	provider := NewTestProvider(t)

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

	repo := NewAutoDreamRepository(provider)

	finding1 := &Finding{
		ID:        "f1",
		Timestamp: time.Now(),
		Content:   "Architecture of Sandbox",
		Embedding: []float32{1.0, 0.0, 0.0},
	}
	finding2 := &Finding{
		ID:        "f2",
		Timestamp: time.Now(),
		Content:   "Deployment strategy for OHC",
		Embedding: []float32{0.0, 1.0, 0.0},
	}

	if err := repo.Upsert(ctx, finding1); err != nil {
		t.Fatalf("failed to upsert finding1: %v", err)
	}
	if err := repo.Upsert(ctx, finding2); err != nil {
		t.Fatalf("failed to upsert finding2: %v", err)
	}

	// Search for something close to finding1
	results, err := repo.Search(ctx, []float32{1.0, 0.1, 0.0}, 10)
	if err != nil {
		t.Fatalf("failed to search: %v", err)
	}

	if len(results) != 2 {
		t.Fatalf("expected 2 results, got %d", len(results))
	}

	if results[0].ID != "f1" {
		t.Errorf("expected closest finding to be f1, got %s", results[0].ID)
	}

	// Test Update
	finding1.Content = "Architecture of Sandbox v2"
	if err := repo.Upsert(ctx, finding1); err != nil {
		t.Fatalf("failed to update finding1: %v", err)
	}

	results, err = repo.Search(ctx, []float32{1.0, 0.0, 0.0}, 1)
	if err != nil {
		t.Fatalf("failed to search after update: %v", err)
	}
	if results[0].Content != "Architecture of Sandbox v2" {
		t.Errorf("expected updated content, got %s", results[0].Content)
	}
}
