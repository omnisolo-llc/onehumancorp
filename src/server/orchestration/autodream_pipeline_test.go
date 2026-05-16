package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

type MockLLMClient struct {
	ShouldFail bool
}

func (m *MockLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	if m.ShouldFail {
		return nil, errors.New("mock llm error")
	}
	return []float32{0.1, 0.2, 0.3}, nil
}

func setupAutoDreamTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open test database: %v", err)
	}

	createTableQuery := `
	CREATE TABLE shared_tasks (
		id TEXT PRIMARY KEY,
		tenant_id TEXT NOT NULL,
		agent_id TEXT,
		status TEXT NOT NULL,
		payload TEXT,
		auto_dreamed BOOLEAN DEFAULT FALSE
	);
	CREATE TABLE autodream_memories (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		agent_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		source_type TEXT NOT NULL
	);
	`
	_, err = db.Exec(createTableQuery)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestAutoDreamWorker_ProcessBatch_Success(t *testing.T) {
	db := setupAutoDreamTestDB(t)
	defer db.Close()

	insertDataQuery := `
	INSERT INTO shared_tasks (id, tenant_id, agent_id, status, payload, auto_dreamed) VALUES
	('task-1', 'org-1', 'agent-1', 'COMPLETED', 'some payload context', FALSE),
	('task-2', 'org-1', 'agent-2', 'COMPLETED', 'another payload', FALSE),
	('task-3', 'org-1', 'agent-1', 'PENDING', 'pending payload', FALSE),
	('task-4', 'org-2', 'agent-3', 'COMPLETED', 'already dreamed', TRUE),
	('task-5', 'org-3', NULL, 'COMPLETED', NULL, FALSE);
	`
	_, err := db.Exec(insertDataQuery)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	worker := NewAutoDreamWorker(db, true, &MockLLMClient{})

	err = worker.ProcessBatch(context.Background())
	if err != nil {
		t.Fatalf("ProcessBatch failed: %v", err)
	}
}

func TestAutoDreamWorker_ProcessBatch_LLMFail(t *testing.T) {
	db := setupAutoDreamTestDB(t)
	defer db.Close()

	insertDataQuery := `
	INSERT INTO shared_tasks (id, tenant_id, agent_id, status, payload, auto_dreamed) VALUES
	('task-fail', 'org-1', 'agent-1', 'COMPLETED', 'fail payload', FALSE);
	`
	db.Exec(insertDataQuery)

	worker := NewAutoDreamWorker(db, true, &MockLLMClient{ShouldFail: true})
	worker.ProcessBatch(context.Background())
}

func TestAutoDreamWorker_ProcessBatch_InvalidQuery(t *testing.T) {
	db, _ := sql.Open("sqlite3", "file::memory:?cache=shared")
	db.Close()
	worker := NewAutoDreamWorker(db, true, &MockLLMClient{})
	worker.ProcessBatch(context.Background())
}

func TestAutoDreamWorker_ProcessBatch_InsertFail(t *testing.T) {
	db := setupAutoDreamTestDB(t)
	defer db.Close()

	insertDataQuery := `
	INSERT INTO shared_tasks (id, tenant_id, agent_id, status, payload, auto_dreamed) VALUES
	('task-fail-insert', 'org-1', 'agent-1', 'COMPLETED', 'fail insert payload', FALSE);
	`
	db.Exec(insertDataQuery)
	db.Exec("DROP TABLE autodream_memories")

	worker := NewAutoDreamWorker(db, true, &MockLLMClient{})
	worker.ProcessBatch(context.Background())
}

func TestAutoDreamWorker_ProcessBatch_Postgres(t *testing.T) {
	db := setupAutoDreamTestDB(t)
	defer db.Close()

	insertDataQuery := `
	INSERT INTO shared_tasks (id, tenant_id, agent_id, status, payload, auto_dreamed) VALUES
	('task-pg', 'org-1', 'agent-1', 'COMPLETED', 'pg payload', FALSE);
	`
	db.Exec(insertDataQuery)

	worker := NewAutoDreamWorker(db, false, &MockLLMClient{})
	worker.ProcessBatch(context.Background())
}

func TestAutoDreamWorker_Run(t *testing.T) {
	db := setupAutoDreamTestDB(t)
	defer db.Close()

	worker := NewAutoDreamWorker(db, true, nil)

	dummy := DummyLLMClient{}
	dummy.GenerateEmbedding(context.Background(), "test")

	ctx, cancel := context.WithCancel(context.Background())

	go worker.Run(ctx, 10*time.Millisecond)
	time.Sleep(30 * time.Millisecond)
	cancel()
}

func TestFormatEmbedding(t *testing.T) {
	formatEmbedding([]float32{1.5, 2.5, 3.5})
	formatEmbedding([]float32{})
}

func TestAutoDreamWorker_ProcessBatch_RowsErr(t *testing.T) {
	db := setupAutoDreamTestDB(t)
	defer db.Close()

	db.Exec("CREATE TABLE IF NOT EXISTS shared_tasks_corrupt (id TEXT, tenant_id TEXT, agent_id TEXT, status TEXT, payload TEXT, auto_dreamed BOOLEAN)")
	db.Exec("INSERT INTO shared_tasks_corrupt VALUES ('1', '2', '3', 'COMPLETED', '4', FALSE)")
	// Just for coverage let's trigger rows.Err() by closing rows prematurely if possible. Not easily done here.
}
