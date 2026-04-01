package db

import (
	"context"

	"path/filepath"
	"testing"
)

func TestProvider_SQLite(t *testing.T) {
	tmpDir := t.TempDir()
	dbPath := filepath.Join(tmpDir, "swarm.db")

	t.Setenv("DATABASE_URL", "sqlite://"+dbPath)
	t.Setenv("OHC_STANDALONE", "true")

	ctx := context.Background()
	provider, err := NewProvider(ctx)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}
	defer provider.Close()

	if provider.Dialect != "sqlite" {
		t.Errorf("Expected dialect sqlite, got %s", provider.Dialect)
	}

	if provider.Sqlite == nil {
		t.Fatal("Expected Sqlite DB to be initialized")
	}

	if err := provider.RunMigrations(ctx); err != nil {
		t.Fatalf("Failed to run migrations: %v", err)
	}

	// Test array insertion/selection fallback via JSON
	_, err = provider.Sqlite.ExecContext(ctx, `INSERT INTO meeting_rooms (id, agenda, participants) VALUES ('test-room', 'test', '["a", "b"]')`)
	if err != nil {
		t.Fatalf("Failed to insert mock data: %v", err)
	}

	var parts string
	err = provider.Sqlite.QueryRowContext(ctx, "SELECT participants FROM meeting_rooms WHERE id = 'test-room'").Scan(&parts)
	if err != nil {
		t.Fatalf("Failed to query mock data: %v", err)
	}

	if parts != `["a", "b"]` {
		t.Errorf("Expected JSON array string, got %s", parts)
	}
}
