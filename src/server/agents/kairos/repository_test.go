package kairos

import (
	"context"
	"database/sql"
	"fmt"
	"testing"
	"time"

	"github.com/DATA-DOG/go-sqlmock"
)

func setupMockDB(t *testing.T) (*sql.DB, sqlmock.Sqlmock, Repository) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	repo := NewRepository(db)
	return db, mock, repo
}

func TestRepository_CreateMission(t *testing.T) {
	db, mock, repo := setupMockDB(t)
	defer db.Close()

	mission := &Mission{
		ID:              "m1",
		EpicID:          "e1",
		Title:           "Test Mission",
		Status:          MissionStatusPending,
		AssignedAgentID: "a1",
		CreatedAt:       time.Now(),
		UpdatedAt:       time.Now(),
	}

	mock.ExpectExec("INSERT INTO ohc_tasks.missions").
		WithArgs(mission.ID, mission.EpicID, mission.Title, mission.Status, mission.AssignedAgentID, mission.CreatedAt, mission.UpdatedAt).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err := repo.CreateMission(context.Background(), mission)
	if err != nil {
		t.Errorf("error was not expected while inserting mission: %s", err)
	}
	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}

	missionZero := &Mission{
		ID:              "m2",
		EpicID:          "e2",
		Title:           "Test Mission 2",
		Status:          MissionStatusPending,
		AssignedAgentID: "",
	}

	mock.ExpectExec("INSERT INTO ohc_tasks.missions").
		WithArgs(missionZero.ID, missionZero.EpicID, missionZero.Title, missionZero.Status, sql.NullString{}, sqlmock.AnyArg(), sqlmock.AnyArg()).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = repo.CreateMission(context.Background(), missionZero)
	if err != nil {
		t.Errorf("error was not expected while inserting mission: %s", err)
	}

	mock.ExpectExec("INSERT INTO ohc_tasks.missions").WillReturnError(fmt.Errorf("db error"))
	err = repo.CreateMission(context.Background(), mission)
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}

func TestRepository_GetMission(t *testing.T) {
	db, mock, repo := setupMockDB(t)
	defer db.Close()

	now := time.Now()
	rows := sqlmock.NewRows([]string{"id", "epic_id", "title", "status", "assigned_agent_id", "created_at", "updated_at"}).
		AddRow("m1", "e1", "Test", MissionStatusPending, "a1", now, now)

	mock.ExpectQuery(`SELECT id, epic_id, title, status, assigned_agent_id, created_at, updated_at\s+FROM ohc_tasks\.missions`).
		WithArgs("m1").
		WillReturnRows(rows)

	m, err := repo.GetMission(context.Background(), "m1")
	if err != nil {
		t.Errorf("error was not expected while querying mission: %s", err)
	}
	if m.ID != "m1" || m.AssignedAgentID != "a1" {
		t.Errorf("unexpected mission data")
	}

	mock.ExpectQuery("SELECT").WithArgs("m2").WillReturnError(sql.ErrNoRows)
	_, err = repo.GetMission(context.Background(), "m2")
	if err == nil || err.Error() != "mission not found" {
		t.Errorf("expected mission not found error, got: %v", err)
	}

	mock.ExpectQuery("SELECT").WithArgs("m3").WillReturnError(fmt.Errorf("db error"))
	_, err = repo.GetMission(context.Background(), "m3")
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}

func TestRepository_UpdateMissionStatus(t *testing.T) {
	db, mock, repo := setupMockDB(t)
	defer db.Close()

	mock.ExpectExec("UPDATE ohc_tasks.missions").
		WithArgs(MissionStatusInProgress, "a1", sqlmock.AnyArg(), "m1").
		WillReturnResult(sqlmock.NewResult(1, 1))

	err := repo.UpdateMissionStatus(context.Background(), "m1", MissionStatusInProgress, "a1")
	if err != nil {
		t.Errorf("error was not expected: %s", err)
	}

	mock.ExpectExec("UPDATE ohc_tasks.missions").
		WillReturnResult(sqlmock.NewResult(0, 0))
	err = repo.UpdateMissionStatus(context.Background(), "m2", MissionStatusInProgress, "")
	if err == nil || err.Error() != "mission not found" {
		t.Errorf("expected mission not found error, got: %v", err)
	}

	mock.ExpectExec("UPDATE ohc_tasks.missions").WillReturnError(fmt.Errorf("db error"))
	err = repo.UpdateMissionStatus(context.Background(), "m3", MissionStatusInProgress, "")
	if err == nil {
		t.Errorf("expected error, got nil")
	}

	mock.ExpectExec("UPDATE ohc_tasks.missions").WillReturnResult(sqlmock.NewErrorResult(fmt.Errorf("rows affected err")))
	err = repo.UpdateMissionStatus(context.Background(), "m4", MissionStatusInProgress, "")
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}

func TestRepository_AddMissionDependency(t *testing.T) {
	db, mock, repo := setupMockDB(t)
	defer db.Close()

	dep := &MissionDependency{
		ID:                 "d1",
		MissionID:          "m1",
		DependsOnMissionID: "m2",
		CreatedAt:          time.Now(),
	}

	mock.ExpectExec("INSERT INTO ohc_tasks.mission_dependencies").
		WithArgs(dep.ID, dep.MissionID, dep.DependsOnMissionID, dep.CreatedAt).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err := repo.AddMissionDependency(context.Background(), dep)
	if err != nil {
		t.Errorf("error was not expected: %s", err)
	}

	dep.CreatedAt = time.Time{}
	mock.ExpectExec("INSERT INTO ohc_tasks.mission_dependencies").
		WithArgs(dep.ID, dep.MissionID, dep.DependsOnMissionID, sqlmock.AnyArg()).
		WillReturnResult(sqlmock.NewResult(1, 1))
	err = repo.AddMissionDependency(context.Background(), dep)
	if err != nil {
		t.Errorf("error was not expected: %s", err)
	}

	mock.ExpectExec("INSERT INTO ohc_tasks.mission_dependencies").WillReturnError(fmt.Errorf("db error"))
	err = repo.AddMissionDependency(context.Background(), dep)
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}

func TestRepository_GetMissionDependencies(t *testing.T) {
	db, mock, repo := setupMockDB(t)
	defer db.Close()

	now := time.Now()
	rows := sqlmock.NewRows([]string{"id", "mission_id", "depends_on_mission_id", "created_at"}).
		AddRow("d1", "m1", "m2", now).
		AddRow("d2", "m1", "m3", now)

	mock.ExpectQuery(`SELECT id, mission_id, depends_on_mission_id, created_at\s+FROM ohc_tasks\.mission_dependencies`).
		WithArgs("m1").
		WillReturnRows(rows)

	deps, err := repo.GetMissionDependencies(context.Background(), "m1")
	if err != nil {
		t.Errorf("error was not expected: %s", err)
	}
	if len(deps) != 2 {
		t.Errorf("expected 2 dependencies, got %d", len(deps))
	}

	rowsErr := sqlmock.NewRows([]string{"id", "mission_id", "depends_on_mission_id", "created_at"}).
		AddRow("d1", "m1", "m2", "not a time")
	mock.ExpectQuery("SELECT").WithArgs("m3").WillReturnRows(rowsErr)
	_, err = repo.GetMissionDependencies(context.Background(), "m3")
	if err == nil {
		t.Errorf("expected error, got nil")
	}

	rowsIterErr := sqlmock.NewRows([]string{"id", "mission_id", "depends_on_mission_id", "created_at"}).
		AddRow("d1", "m1", "m2", now).RowError(0, fmt.Errorf("iter error"))
	mock.ExpectQuery("SELECT").WithArgs("m4").WillReturnRows(rowsIterErr)
	_, err = repo.GetMissionDependencies(context.Background(), "m4")
	if err == nil {
		t.Errorf("expected error, got nil")
	}

	mock.ExpectQuery("SELECT").WillReturnError(fmt.Errorf("db error"))
	_, err = repo.GetMissionDependencies(context.Background(), "m2")
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}

func TestRepository_SaveAutodreamVector(t *testing.T) {
	db, mock, repo := setupMockDB(t)
	defer db.Close()

	vec := &AutodreamVector{
		ID:        "v1",
		MissionID: "m1",
		Embedding: []float32{0.1, 0.2, 0.3},
		CreatedAt: time.Now(),
	}

	mock.ExpectExec("INSERT INTO ohc_memory.autodream_vectors").
		WithArgs(vec.ID, vec.MissionID, "[0.100000,0.200000,0.300000]", vec.CreatedAt).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err := repo.SaveAutodreamVector(context.Background(), vec)
	if err != nil {
		t.Errorf("error was not expected: %s", err)
	}

	vec.CreatedAt = time.Time{}
	mock.ExpectExec("INSERT INTO ohc_memory.autodream_vectors").
		WithArgs(vec.ID, vec.MissionID, "[0.100000,0.200000,0.300000]", sqlmock.AnyArg()).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = repo.SaveAutodreamVector(context.Background(), vec)
	if err != nil {
		t.Errorf("error was not expected: %s", err)
	}

	mock.ExpectExec("INSERT INTO ohc_memory.autodream_vectors").WillReturnError(fmt.Errorf("db error"))
	err = repo.SaveAutodreamVector(context.Background(), vec)
	if err == nil {
		t.Errorf("expected error, got nil")
	}
}
