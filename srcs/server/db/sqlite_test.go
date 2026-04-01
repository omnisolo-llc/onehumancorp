package db

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestSQLiteProvider(t *testing.T) {
	ctx := context.Background()
	tempDir, err := os.MkdirTemp("", "sqlite-test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	dbPath := filepath.Join(tempDir, "test.db")
	dsn := "file:" + dbPath + "?cache=shared&mode=rwc"
	provider, err := NewSQLite(ctx, dsn)
	if err != nil {
		t.Fatalf("failed to create SQLite provider: %v", err)
	}
	defer provider.Close()

	if err := provider.Ping(ctx); err != nil {
		t.Errorf("failed to ping SQLite: %v", err)
	}

	// Test a simple query
	_, err = provider.Exec(ctx, "CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT)")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = provider.Exec(ctx, "INSERT INTO test (name) VALUES (?)", "jules")
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	var name string
	err = provider.QueryRow(ctx, "SELECT name FROM test WHERE id = ?", 1).Scan(&name)
	if err != nil {
		t.Fatalf("failed to query data: %v", err)
	}
	if name != "jules" {
		t.Errorf("expected 'jules', got '%s'", name)
	}

	// Test RunMigrations
	if err := provider.RunMigrations(ctx); err != nil {
		t.Errorf("failed to run migrations: %v", err)
	}
}
