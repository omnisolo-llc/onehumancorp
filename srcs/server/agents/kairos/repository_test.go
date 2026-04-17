package kairos

import (
	"context"
	"database/sql"
    "fmt"
	"testing"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// mockProvider implements db.Provider to test the PostgreSQL paths.
type mockProvider struct {
	db.Provider // Embed so we don't have to implement everything
    ExecCalled bool
    QueryRowCalled bool
    QueryCalled bool
}

func (m *mockProvider) IsSQLite() bool {
	return false
}

func (m *mockProvider) Exec(ctx context.Context, query string, args ...any) (int64, error) {
    m.ExecCalled = true
	return 1, nil
}

type mockRow struct {}
func (m *mockRow) Scan(dest ...any) error { return sql.ErrNoRows }

func (m *mockProvider) QueryRow(ctx context.Context, query string, args ...any) db.Row {
    m.QueryRowCalled = true
	return &mockRow{}
}

func (m *mockProvider) Query(ctx context.Context, query string, args ...any) (db.Rows, error) {
    m.QueryCalled = true
	return nil, fmt.Errorf("mock error")
}


func TestMissionRepository_PostgresPaths(t *testing.T) {
    mp := &mockProvider{}
    repo := NewMissionRepository(mp)
    ctx := context.Background()

    // CreateMission
    _ = repo.CreateMission(ctx, &Mission{Title: "pg test"})
    if !mp.ExecCalled {
        t.Errorf("expected Exec to be called on pg branch")
    }

    // GetMission
    _, _ = repo.GetMission(ctx, uuid.New())
    if !mp.QueryRowCalled {
        t.Errorf("expected QueryRow to be called on pg branch")
    }

    // UpdateMissionStatus
    mp.ExecCalled = false
    _ = repo.UpdateMissionStatus(ctx, uuid.New(), "IN_PROGRESS")
    if !mp.ExecCalled {
        t.Errorf("expected Exec to be called on pg branch for update")
    }

    // AddDependency
    mp.ExecCalled = false
    _ = repo.AddDependency(ctx, uuid.New(), uuid.New())
    if !mp.ExecCalled {
        t.Errorf("expected Exec to be called on pg branch for dependency")
    }

    // GetDependencies
    _, _ = repo.GetDependencies(ctx, uuid.New())
    if !mp.QueryCalled {
        t.Errorf("expected Query to be called on pg branch for get deps")
    }

    // CreateAutodreamVector
    mp.ExecCalled = false
    _ = repo.CreateAutodreamVector(ctx, &AutodreamVector{Embedding: []float32{1.0}})
    if !mp.ExecCalled {
        t.Errorf("expected Exec to be called on pg branch for vector")
    }

    mp.ExecCalled = false
    _ = repo.CreateAutodreamVector(ctx, &AutodreamVector{}) // No embedding
    if !mp.ExecCalled {
        t.Errorf("expected Exec to be called on pg branch for empty vector")
    }
}

func TestMissionRepository_SQLitePaths(t *testing.T) {
	dbProvider := db.NewTestProvider(t)
	defer dbProvider.Close()

	ctx := context.Background()

    // setup schema
	_, err := dbProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS ohc_tasks_missions (
            id TEXT PRIMARY KEY,
            epic_id TEXT,
            title TEXT NOT NULL,
            status TEXT NOT NULL DEFAULT 'PENDING',
            assigned_agent_id TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS ohc_tasks_mission_dependencies (
            task_id TEXT NOT NULL,
            depends_on_task_id TEXT NOT NULL,
            PRIMARY KEY (task_id, depends_on_task_id)
        );

        CREATE TABLE IF NOT EXISTS ohc_memory_autodream_vectors (
            id TEXT PRIMARY KEY,
            task_id TEXT,
            content TEXT NOT NULL,
            embedding TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );
    `)
	if err != nil {
		t.Fatalf("failed to create schema: %v", err)
	}

    repo := NewMissionRepository(dbProvider)

	// test CreateMission
	missionID := uuid.New()
	err = repo.CreateMission(ctx, &Mission{
		ID:    missionID,
		Title: "Test Mission",
	})
	if err != nil {
		t.Errorf("failed to create mission: %v", err)
	}

    // test CreateMission with Epic
    epicID := uuid.New()
    missionIDEpic := uuid.New()
	err = repo.CreateMission(ctx, &Mission{
		ID:    missionIDEpic,
        EpicID: &epicID,
		Title: "Epic Mission",
	})
	if err != nil {
		t.Errorf("failed to create epic mission: %v", err)
	}

	// test GetMission
	mission, err := repo.GetMission(ctx, missionID)
	if err != nil {
		t.Errorf("failed to get mission: %v", err)
	}
	if mission == nil {
		t.Fatalf("mission is nil")
	}
	if mission.Title != "Test Mission" {
		t.Errorf("expected title 'Test Mission', got '%s'", mission.Title)
	}
	if mission.Status != "PENDING" {
		t.Errorf("expected status 'PENDING', got '%s'", mission.Status)
	}

    // test GetMission missing
    missingID := uuid.New()
	missionMissing, err := repo.GetMission(ctx, missingID)
	if err != nil {
		t.Errorf("failed to get mission: %v", err)
	}
    if missionMissing != nil {
        t.Errorf("expected mission to be nil")
    }

	// test UpdateMissionStatus
	err = repo.UpdateMissionStatus(ctx, missionID, "IN_PROGRESS")
	if err != nil {
		t.Errorf("failed to update mission status: %v", err)
	}
	mission, err = repo.GetMission(ctx, missionID)
	if err != nil {
		t.Errorf("failed to get mission: %v", err)
	}
	if mission.Status != "IN_PROGRESS" {
		t.Errorf("expected status 'IN_PROGRESS', got '%s'", mission.Status)
	}

	// test Dependencies
	depID := uuid.New()
	err = repo.CreateMission(ctx, &Mission{
		ID:    depID,
		Title: "Dependency Mission",
	})
	if err != nil {
		t.Errorf("failed to create dependency mission: %v", err)
	}

	err = repo.AddDependency(ctx, missionID, depID)
	if err != nil {
		t.Errorf("failed to add dependency: %v", err)
	}

	deps, err := repo.GetDependencies(ctx, missionID)
	if err != nil {
		t.Errorf("failed to get dependencies: %v", err)
	}
	if len(deps) != 1 {
		t.Fatalf("expected 1 dependency, got %d", len(deps))
	}
	if deps[0] != depID {
		t.Errorf("expected dependency ID '%s', got '%s'", depID, deps[0])
	}

	// test AutodreamVector
	vectorID := uuid.New()
	err = repo.CreateAutodreamVector(ctx, &AutodreamVector{
		ID:        vectorID,
		TaskID:    &missionID,
		Content:   "test content",
		Embedding: []float32{0.1, 0.2, 0.3},
	})
	if err != nil {
		t.Errorf("failed to create autodream vector: %v", err)
	}

    // test AutodreamVector without embedding
    vectorID2 := uuid.New()
	err = repo.CreateAutodreamVector(ctx, &AutodreamVector{
		ID:        vectorID2,
		TaskID:    &missionID,
		Content:   "test content 2",
	})
	if err != nil {
		t.Errorf("failed to create autodream vector: %v", err)
	}
}
