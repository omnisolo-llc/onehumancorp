package db

import (
	"context"
	"os"
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

	// Clean up swarm.db
	os.RemoveAll(".agent-task")
}

func TestStandaloneEncryptionKeyGeneration(t *testing.T) {
	// Verify that when OHC_STANDALONE=true and OHC_SQLITE_KEY is empty,
	// New() uses a dynamically generated 32-byte secure encryption key,
	// rejecting the static 'standalone_ephemeral_key', and persists it.

	// Ensure clean slate
	os.RemoveAll(".agent-task")

	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("DATABASE_URL", "sqlite://file:test_crypto.db?mode=memory")
	os.Setenv("OHC_SQLITE_KEY", "")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("DATABASE_URL")
	defer os.Unsetenv("OHC_SQLITE_KEY")

	db1, err := New(context.Background())
	if err != nil {
		t.Fatalf("Failed to initialize db1: %v", err)
	}
	db1.Close()

	if !db1.Provider.IsSQLite() {
		t.Errorf("Expected fallback to SQLite provider in standalone mode")
	}

	// Verify key file was created
	keyFile := ".agent-task/.ohc_sqlite_key"
	if _, err := os.Stat(keyFile); os.IsNotExist(err) {
		t.Fatalf("Expected key file %s to be created", keyFile)
	}

	keyBytes, _ := os.ReadFile(keyFile)
	if len(keyBytes) != 64 { // 32 bytes hex encoded = 64 chars
		t.Errorf("Expected key to be 64 characters long, got %d", len(keyBytes))
	}

	// Clean up
	os.RemoveAll(".agent-task")
}
