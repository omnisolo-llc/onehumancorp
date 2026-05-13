package orchestration

import (
	"context"
	"database/sql"
	"os"
	"path/filepath"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func TestAutoDreamWorker(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open memory db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`
		CREATE TABLE shared_tasks (
			id TEXT PRIMARY KEY,
			status TEXT,
			auto_dreamed BOOLEAN,
			payload TEXT
		);
		CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT,
			embedding TEXT,
			source_mission_id TEXT
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create tables: %v", err)
	}

	_, err = db.Exec(`INSERT INTO shared_tasks (id, status, auto_dreamed, payload) VALUES ('task-1', 'COMPLETED', FALSE, 'test payload')`)
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	memDir := ".agent-task/memory"
	os.MkdirAll(memDir, 0755)
	defer os.RemoveAll(".agent-task")
	err = os.WriteFile(filepath.Join(memDir, "test.yml"), []byte("test content"), 0644)
	if err != nil {
		t.Fatalf("Failed to write test file: %v", err)
	}
	err = os.WriteFile(filepath.Join(memDir, "test.txt"), []byte("test content"), 0644)
	if err != nil {
		t.Fatalf("Failed to write test file: %v", err)
	}
	os.MkdirAll(filepath.Join(memDir, "test_dir"), 0755)

	worker := NewAutoDreamWorker(db, true)
	err = worker.ConsolidateEpoch(context.Background())
	if err != nil {
		t.Fatalf("ConsolidateEpoch failed: %v", err)
	}

	var count int
	err = db.QueryRow("SELECT COUNT(*) FROM autodream_memories").Scan(&count)
	if err != nil {
		t.Fatalf("Failed to count memories: %v", err)
	}
	if count != 2 {
		t.Errorf("Expected 2 memories, got %d", count)
	}

	var autoDreamed bool
	err = db.QueryRow("SELECT auto_dreamed FROM shared_tasks WHERE id = 'task-1'").Scan(&autoDreamed)
	if err != nil {
		t.Fatalf("Failed to get auto_dreamed status: %v", err)
	}
	if !autoDreamed {
		t.Errorf("Expected auto_dreamed to be true")
	}

	// Test DoDBOperation
	id, err := worker.DoDBOperation(context.Background(), "INSERT INTO shared_tasks (id, status, auto_dreamed, payload) VALUES ('task-2', 'COMPLETED', FALSE, 'test payload 2')")
	if err != nil {
		t.Fatalf("DoDBOperation failed: %v", err)
	}
	_ = id

	_, err = worker.DoDBOperation(context.Background(), "INSERT INTO invalid_table (id) VALUES ('1')")
	if err == nil {
		t.Fatalf("Expected DoDBOperation to fail")
	}

	db2, _ := sql.Open("sqlite3", ":memory:")
	db2.Close()
	workerClosed := NewAutoDreamWorker(db2, true)
	err = workerClosed.ConsolidateEpoch(context.Background())
	if err == nil {
	    t.Fatalf("Expected error when DB is closed")
	}
	os.WriteFile(filepath.Join(memDir, "test2.yml"), []byte("test content"), 0644)
	_ = workerClosed.processFSOperations(context.Background())

	workerPg := NewAutoDreamWorker(db, false)
	_ = workerPg.processCompletedTasks(context.Background())

	db.Exec(`DROP TABLE autodream_memories`)
	db.Exec(`CREATE TABLE autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT,
			embedding TEXT,
			source_mission_id TEXT
		);`)
	_ = workerPg.processFSOperations(context.Background())

    err = os.WriteFile(filepath.Join(memDir, "test_fail.yml"), []byte("test content"), 0644)
    if err == nil {
	os.Chmod(filepath.Join(memDir, "test_fail.yml"), 0000)
	worker.processFSOperations(context.Background())
    }
}

func TestAutoDreamWorker_NoMemDir(t *testing.T) {
	db, _ := sql.Open("sqlite3", ":memory:")
	defer db.Close()
	worker := NewAutoDreamWorker(db, true)
	os.RemoveAll(".agent-task")
	err := worker.processFSOperations(context.Background())
	if err != nil {
		t.Fatalf("Expected no error when memDir is missing, got: %v", err)
	}
}

func TestAutoDreamWorker_ProcessCompletedTasks_RowErr(t *testing.T) {
	db, _ := sql.Open("sqlite3", ":memory:")
	defer db.Close()

	_, _ = db.Exec(`CREATE TABLE shared_tasks (id TEXT PRIMARY KEY, status TEXT, auto_dreamed BOOLEAN, payload TEXT);`)
	worker := NewAutoDreamWorker(db, true)
	_, _ = db.Exec(`INSERT INTO shared_tasks (id, status, auto_dreamed, payload) VALUES ('t1', 'COMPLETED', FALSE, NULL)`)
	_ = worker.processCompletedTasks(context.Background())

	db2, _ := sql.Open("sqlite3", ":memory:")
	worker2 := NewAutoDreamWorker(db2, true)
	db2.Exec(`CREATE TABLE shared_tasks (id TEXT PRIMARY KEY, status TEXT, auto_dreamed BOOLEAN, payload TEXT);`)
	db2.Exec(`INSERT INTO shared_tasks (id, status, auto_dreamed, payload) VALUES ('t1', 'COMPLETED', FALSE, 'test')`)
	db2.Close()
	_ = worker2.processCompletedTasks(context.Background())
}

func TestAutoDreamWorker_ReadDirErr(t *testing.T) {
    db, _ := sql.Open("sqlite3", ":memory:")
	defer db.Close()
	worker := NewAutoDreamWorker(db, true)

	memDir := ".agent-task/memory"
	os.MkdirAll(memDir, 0755)
	defer os.RemoveAll(".agent-task")

	os.Chmod(memDir, 0000)
	defer os.Chmod(memDir, 0755)

	err := worker.processFSOperations(context.Background())
	if err == nil {
	    t.Fatalf("Expected ReadDir to fail")
	}
}

func TestAutoDreamWorker_ConsolidateEpoch_FSErr(t *testing.T) {
	db, _ := sql.Open("sqlite3", ":memory:")
	defer db.Close()
	worker := NewAutoDreamWorker(db, true)

	memDir := ".agent-task/memory"
	os.MkdirAll(memDir, 0755)
	defer os.RemoveAll(".agent-task")
	os.Chmod(memDir, 0000)
	defer os.Chmod(memDir, 0755)

	_, _ = db.Exec(`CREATE TABLE shared_tasks (id TEXT PRIMARY KEY, status TEXT, auto_dreamed BOOLEAN, payload TEXT);`)

	err := worker.ConsolidateEpoch(context.Background())
	if err == nil {
	    t.Fatalf("Expected ConsolidateEpoch to fail due to fs error")
	}
}
