package db

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestDatabaseInitialization_PostgresAndSQLite(t *testing.T) {
	ctx := context.Background()

	t.Run("Initialize SQLite fallback when DATABASE_URL is empty and OHC_STANDALONE=true", func(t *testing.T) {
		// Save envs to restore later
		origURL := os.Getenv("DATABASE_URL")
		origStandalone := os.Getenv("OHC_STANDALONE")
		defer func() {
			os.Setenv("DATABASE_URL", origURL)
			os.Setenv("OHC_STANDALONE", origStandalone)
		}()

		// Set test env
		os.Unsetenv("DATABASE_URL")
		os.Setenv("OHC_STANDALONE", "true")

		// We only want to delete the swarm.db file, not the entire .agent-task directory
		expectedPath := filepath.Join(".agent-task", "swarm.db")
		_ = os.Remove(expectedPath)

		pool, err := New(ctx)
		if err != nil {
			t.Fatalf("Failed to initialize SQLite database fallback: %v", err)
		}
		defer pool.Close()

		if !pool.Provider.IsSQLite() {
			t.Errorf("Expected Provider.IsSQLite() to be true for standalone SQLite fallback, got false")
		}

		// Ensure the directory/file were created
		if _, err := os.Stat(expectedPath); os.IsNotExist(err) {
			t.Errorf("Expected database file at %s but it was not created", expectedPath)
		}

		// Clean up
		_ = os.Remove(expectedPath)
	})

	t.Run("Initialize SQLite when DATABASE_URL is sqlite://:memory:", func(t *testing.T) {
		origURL := os.Getenv("DATABASE_URL")
		defer os.Setenv("DATABASE_URL", origURL)

		os.Setenv("DATABASE_URL", "sqlite://:memory:")

		pool, err := New(ctx)
		if err != nil {
			t.Fatalf("Failed to initialize memory SQLite database: %v", err)
		}
		defer pool.Close()

		if !pool.Provider.IsSQLite() {
			t.Errorf("Expected Provider.IsSQLite() to be true for sqlite://:memory:, got false")
		}
	})

	t.Run("JSONB Array Support via RunMigrations SQLite", func(t *testing.T) {
		origURL := os.Getenv("DATABASE_URL")
		defer os.Setenv("DATABASE_URL", origURL)

		os.Setenv("DATABASE_URL", "sqlite://:memory:")

		pool, err := New(ctx)
		if err != nil {
			t.Fatalf("Failed to initialize memory SQLite database: %v", err)
		}
		defer pool.Close()

		err = pool.RunMigrations(ctx)
		if err != nil {
			t.Fatalf("RunMigrations failed for SQLite: %v", err)
		}
	})
}
