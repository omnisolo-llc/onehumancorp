package db

import (
	"context"
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
	// Use an in-memory SQLite database to avoid creating files on disk.
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory&cache=shared")

	db, err := New(context.Background())
	if err != nil {
		t.Fatalf("Failed to initialize standalone db: %v", err)
	}
	defer db.Close()

	if !db.Provider.IsSQLite() {
		t.Errorf("Expected SQLite provider")
	}
}
