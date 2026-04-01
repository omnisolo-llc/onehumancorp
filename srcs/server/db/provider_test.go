package db

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestConvertBindVarsJSONPath(t *testing.T) {
	query := "SELECT id FROM test WHERE payload::json->>'role' = $1 AND meta :: json ->> 'status' = $2"
	expected := "SELECT id FROM test WHERE json_extract(payload, '$.role') = ?1 AND json_extract(meta, '$.status') = ?2"

	result := convertBindVars(query)
	if result != expected {
		t.Errorf("convertBindVars() = %v, want %v", result, expected)
	}
}

func TestSqliteProviderIsSQLite(t *testing.T) {
	// Let's create an empty SqliteProvider and test its IsSQLite method.
	p := &SqliteProvider{}
	if !p.IsSQLite() {
		t.Errorf("SqliteProvider.IsSQLite() = %v, want true", p.IsSQLite())
	}
}

func TestPgProviderIsSQLite(t *testing.T) {
	// Let's create an empty PgProvider and test its IsSQLite method.
	p := &PgProvider{}
	if p.IsSQLite() {
		t.Errorf("PgProvider.IsSQLite() = %v, want false", p.IsSQLite())
	}
}

func TestStandaloneFallback(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("DATABASE_URL", "")
	defer os.Unsetenv("OHC_STANDALONE")

	db, err := New(context.Background())
	if err != nil {
		t.Fatalf("Failed to initialize standalone db: %v", err)
	}
	defer db.Close()

	if !db.Provider.IsSQLite() {
		t.Errorf("Expected fallback to SQLite provider in standalone mode")
	}

	// Hardening Validation: Verify strictly local file permissions (0600 for files)
	info, err := os.Stat(filepath.Join(".agent-task", "swarm.db"))
	if err == nil {
		if info.Mode().Perm() != 0600 {
			t.Errorf("Expected strict 0600 file permission for standalone SQLite DB, got %v", info.Mode().Perm())
		}
	} else {
		t.Errorf("Could not verify file permissions: %v", err)
	}

	// Clean up swarm.db (Do not delete entire directory, as per constraints)
	os.Remove(filepath.Join(".agent-task", "swarm.db"))
}
