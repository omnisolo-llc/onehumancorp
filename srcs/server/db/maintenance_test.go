package db

import (
	"context"
	"testing"
)

func TestOptimizeStorage_Pg_NoOp(t *testing.T) {
	// A Postgres provider should do nothing and return nil
	p := &PgProvider{}
	err := OptimizeStorage(context.Background(), p)
	if err != nil {
		t.Fatalf("Expected nil, got %v", err)
	}
}

func TestOptimizeStorage_SQLite(t *testing.T) {
	ctx := context.Background()
	p := NewTestProvider(t)
	defer p.Close()

	if !p.IsSQLite() {
		t.Skip("Skipping SQLite-specific test")
	}

	// Create tables if they do not exist
	_, err := p.Exec(ctx, "CREATE TABLE IF NOT EXISTS telemetry_buffer(id INTEGER PRIMARY KEY, created_at TEXT)")
	if err != nil {
		t.Fatalf("Failed to create telemetry_buffer table: %v", err)
	}

	_, err = p.Exec(ctx, "CREATE TABLE IF NOT EXISTS llm_completion_cache(id INTEGER PRIMARY KEY, created_at TEXT)")
	if err != nil {
		t.Fatalf("Failed to create llm_completion_cache table: %v", err)
	}

	// Just run optimization to ensure no syntax errors.
	err = OptimizeStorage(ctx, p)
	if err != nil {
		t.Fatalf("Expected OptimizeStorage to succeed, got %v", err)
	}
}
