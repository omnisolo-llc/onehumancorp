package db

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestNewSQLiteStandalone(t *testing.T) {
	// Setup env
	os.Setenv("DATABASE_URL", "")
	os.Setenv("OHC_STANDALONE", "true")
	defer func() {
		os.Unsetenv("DATABASE_URL")
		os.Unsetenv("OHC_STANDALONE")
	}()

	// Ensure clean start
	os.RemoveAll(".agent-task")

	ctx := context.Background()
	dbInstance, err := New(ctx)
	if err != nil {
		t.Fatalf("expected no error initializing standalone SQLite DB, got: %v", err)
	}
	defer dbInstance.Close()

	if !dbInstance.IsSQLite() {
		t.Errorf("expected IsSQLite() to return true")
	}

	// Verify swarm.db was created
	stat, err := os.Stat(filepath.Join(".agent-task", "swarm.db"))
	if err != nil {
		t.Fatalf("expected swarm.db to be created, got error: %v", err)
	}
	if stat.IsDir() {
		t.Errorf("swarm.db is a directory, should be a file")
	}

	// Test migration
	if err := dbInstance.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	// Insert and retrieve data to ensure SQLite parameter mapping and JSON extracts work
	_, err = dbInstance.Exec(ctx, "CREATE TABLE test_json (id TEXT PRIMARY KEY, payload TEXT)")
	if err != nil {
		t.Fatalf("failed to create test table: %v", err)
	}

	_, err = dbInstance.Exec(ctx, "INSERT INTO test_json (id, payload) VALUES ($1, $2)", "1", `{"role": "admin"}`)
	if err != nil {
		t.Fatalf("failed to insert data: %v", err)
	}

	// Test Postgres style JSON extraction that should be mapped to SQLite by provider
	row := dbInstance.QueryRow(ctx, "SELECT id FROM test_json WHERE payload::json->>'role' = $1", "admin")
	var id string
	if err := row.Scan(&id); err != nil {
		t.Fatalf("failed to query using JSON path translation: %v", err)
	}
	if id != "1" {
		t.Errorf("expected id '1', got %s", id)
	}
}

func TestConvertBindVars(t *testing.T) {
	query := "SELECT id FROM agent_missions WHERE payload::json->>'role' = $1 AND status = 'PENDING'"
	converted := convertBindVars(query)
	expected := "SELECT id FROM agent_missions WHERE json_extract(payload, '$.role') = ?1 AND status = 'PENDING'"
	if converted != expected {
		t.Errorf("expected %q, got %q", expected, converted)
	}

	query2 := "UPDATE users SET metadata = jsonb_set(metadata, '{role}', '\"new_role\"') WHERE id = $1 RETURNING *"
	converted2 := convertBindVars(query2)
	expected2 := "UPDATE users SET metadata = jsonb_set(metadata, '{role}', '\"new_role\"') WHERE id = ?1 RETURNING *"
	if converted2 != expected2 {
		t.Errorf("expected %q, got %q", expected2, converted2)
	}

	query3 := "INSERT INTO things (data) VALUES ('some string with $1 inside') WHERE id = $2"
	converted3 := convertBindVars(query3)
	expected3 := "INSERT INTO things (data) VALUES ('some string with $1 inside') WHERE id = ?2"
	if converted3 != expected3 {
		t.Errorf("expected %q, got %q", expected3, converted3)
	}
}
