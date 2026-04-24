package db

import (
	"reflect"
	"testing"
	"context"
)

func TestSplitSQLStatements(t *testing.T) {
	input := `
CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);
INSERT INTO users(name) VALUES ('A;B');
-- keep this together;
INSERT INTO users(name) VALUES ('C');
/* multi; line; comment */
UPDATE users SET name = "semi;colon" WHERE id = 1;
`

	got := splitSQLStatements(input)
	want := []string{
		"CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)",
		"INSERT INTO users(name) VALUES ('A;B')",
		"-- keep this together;\nINSERT INTO users(name) VALUES ('C')",
		"/* multi; line; comment */\nUPDATE users SET name = \"semi;colon\" WHERE id = 1",
	}

	if !reflect.DeepEqual(got, want) {
		t.Fatalf("splitSQLStatements() = %#v, want %#v", got, want)
	}
}

func TestKairosMasterOrchestrationMigration(t *testing.T) {
	// Use an in-memory SQLite database to test degradation logic
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory&cache=shared")

	dbInstance, err := New(context.Background())
	if err != nil {
		t.Fatalf("Failed to initialize standalone db: %v", err)
	}
	defer dbInstance.Close()

	ctx := context.Background()

	// Run migrations
	if err := dbInstance.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	// Verify kairos_shared_tasks
	var count int
	err = dbInstance.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM kairos_shared_tasks").Scan(&count)
	if err != nil {
		t.Fatalf("kairos_shared_tasks table missing or invalid: %v", err)
	}

	// Verify kairos_state_transitions
	err = dbInstance.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM kairos_state_transitions").Scan(&count)
	if err != nil {
		t.Fatalf("kairos_state_transitions table missing or invalid: %v", err)
	}

	// Verify kairos_sub_agent_jobs
	err = dbInstance.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM kairos_sub_agent_jobs").Scan(&count)
	if err != nil {
		t.Fatalf("kairos_sub_agent_jobs table missing or invalid: %v", err)
	}

	// Verify autodream_vector_memories
	err = dbInstance.Provider.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_vector_memories").Scan(&count)
	if err != nil {
		t.Fatalf("autodream_vector_memories table missing or invalid: %v", err)
	}
}
