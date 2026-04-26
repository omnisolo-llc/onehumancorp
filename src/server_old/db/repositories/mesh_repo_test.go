package repositories

import (
	"context"
	"errors"
	"testing"


	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/db/models"
)

type mockTx struct {
	db.Tx
}

func (m *mockTx) Commit(ctx context.Context) error { return nil }
func (m *mockTx) Rollback(ctx context.Context) error { return nil }

type mockRows struct {
	db.Rows
	data      [][]interface{}
	currIndex int
	err       error
}

func (m *mockRows) Next() bool {
	if m.currIndex < len(m.data) {
		m.currIndex++
		return true
	}
	return false
}

func (m *mockRows) Scan(dest ...interface{}) error {
	row := m.data[m.currIndex-1]
	for i, d := range dest {
		switch ptr := d.(type) {
		case *string:
			*ptr = row[i].(string)
		}
	}
	return nil
}

func (m *mockRows) Close() {}
func (m *mockRows) Err() error { return m.err }

type mockRow struct {
	db.Row
}
func (m *mockRow) Scan(dest ...any) error { return nil }

type MockDBProvider struct {
	db.Provider
	isSQLite   bool
	execErr    error
	queryErr   error
	rows       *mockRows
}

func (m *MockDBProvider) IsSQLite() bool {
	return m.isSQLite
}

func (m *MockDBProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.execErr != nil {
		return 0, m.execErr
	}
	return 1, nil
}

func (m *MockDBProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if m.queryErr != nil {
		return nil, m.queryErr
	}
	m.rows.currIndex = 0
	return m.rows, nil
}

func TestMeshRepository_CreateMission(t *testing.T) {
	mockDB := &MockDBProvider{}
	repo := NewMeshRepository(mockDB)

	agentID := "agent1"
	mission := &models.Mission{
		ID:              "mission1",
		EpicID:          "epic1",
		Title:           "Test",
		Status:          "PENDING",
		AssignedAgentID: &agentID,
	}

	err := repo.CreateMission(context.Background(), mission)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Test error case
	mockDB.execErr = errors.New("db error")
	err = repo.CreateMission(context.Background(), mission)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestMeshRepository_UpdateMissionStatus(t *testing.T) {
	mockDB := &MockDBProvider{}
	repo := NewMeshRepository(mockDB)

	err := repo.UpdateMissionStatus(context.Background(), "mission1", "IN_PROGRESS")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Test error case
	mockDB.execErr = errors.New("db error")
	err = repo.UpdateMissionStatus(context.Background(), "mission1", "IN_PROGRESS")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestMeshRepository_GetMissionDependencies(t *testing.T) {
	mockDB := &MockDBProvider{
		rows: &mockRows{
			data: [][]interface{}{
				{"dep1"},
				{"dep2"},
			},
		},
	}
	repo := NewMeshRepository(mockDB)

	deps, err := repo.GetMissionDependencies(context.Background(), "mission1")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(deps) != 2 {
		t.Fatalf("expected 2 deps, got %d", len(deps))
	}
	if deps[0] != "dep1" || deps[1] != "dep2" {
		t.Fatalf("unexpected deps: %v", deps)
	}

	// Test error case
	mockDB.queryErr = errors.New("db error")
	_, err = repo.GetMissionDependencies(context.Background(), "mission1")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestMeshRepository_InsertAutodreamVector(t *testing.T) {
	mockDB := &MockDBProvider{}
	repo := NewMeshRepository(mockDB)

	vector := &models.AutodreamVector{
		ID:        "vec1",
		TaskID:    "mission1",
		Content:   "content",
		Embedding: []float32{0.1, 0.2},
		Metadata:  map[string]interface{}{"key": "val"},
	}

	err := repo.InsertAutodreamVector(context.Background(), vector)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Test error case
	mockDB.execErr = errors.New("db error")
	err = repo.InsertAutodreamVector(context.Background(), vector)
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestMeshRepository_SQLiteFallback(t *testing.T) {
	mockDB := &MockDBProvider{isSQLite: true}
	repo := NewMeshRepository(mockDB)

	agentID := "agent1"
	mission := &models.Mission{
		ID:              "mission1",
		EpicID:          "epic1",
		Title:           "Test",
		Status:          "PENDING",
		AssignedAgentID: &agentID,
	}

	err := repo.CreateMission(context.Background(), mission)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	err = repo.UpdateMissionStatus(context.Background(), "mission1", "IN_PROGRESS")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	vector := &models.AutodreamVector{
		ID:        "vec1",
		TaskID:    "mission1",
		Content:   "content",
		Embedding: []float32{0.1, 0.2},
		Metadata:  map[string]interface{}{"key": "val"},
	}

	err = repo.InsertAutodreamVector(context.Background(), vector)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}
