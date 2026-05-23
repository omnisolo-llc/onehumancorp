package kairos

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/google/uuid"
	"database/sql"
	_ "github.com/lib/pq"
	"github.com/DATA-DOG/go-sqlmock"
)

func TestKairosRepositoryWithMock(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	repo := NewKairosRepository(db)
	ctx := context.Background()

	mission := &Mission{
		ID:     uuid.New(),
		EpicID: uuid.New(),
		Title:  "Test Mission",
		Status: "pending",
	}

	mock.ExpectExec("INSERT INTO ohc_tasks.missions").
		WithArgs(mission.ID, mission.EpicID, mission.Title, mission.Status, mission.AssignedAgentID).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = repo.CreateMission(ctx, mission)
	if err != nil {
		t.Errorf("error was not expected while inserting: %s", err)
	}

	mock.ExpectQuery("SELECT id, epic_id, title, status, assigned_agent_id FROM ohc_tasks.missions").
		WithArgs(mission.ID).
		WillReturnRows(sqlmock.NewRows([]string{"id", "epic_id", "title", "status", "assigned_agent_id"}).
			AddRow(mission.ID, mission.EpicID, mission.Title, mission.Status, mission.AssignedAgentID))

	fetched, err := repo.GetMission(ctx, mission.ID)
	if err != nil {
		t.Errorf("error was not expected while selecting: %s", err)
	}
	if fetched.Title != mission.Title {
		t.Errorf("Expected title %s, got %s", mission.Title, fetched.Title)
	}

	dep := &MissionDependency{
		ID:                 uuid.New(),
		MissionID:          mission.ID,
		DependsOnMissionID: mission.ID,
	}

	mock.ExpectExec("INSERT INTO ohc_tasks.mission_dependencies").
		WithArgs(dep.ID, dep.MissionID, dep.DependsOnMissionID).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = repo.CreateMissionDependency(ctx, dep)
	if err != nil {
		t.Errorf("error was not expected while inserting dep: %s", err)
	}

	vec := &AutodreamVector{
		ID:         uuid.New(),
		MissionID:  mission.ID,
		VectorData: "[0.1]",
		CreatedAt:  time.Now(),
	}

	mock.ExpectExec("INSERT INTO ohc_memory.autodream_vectors").
		WithArgs(vec.ID, vec.MissionID, vec.VectorData, vec.CreatedAt).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = repo.CreateAutodreamVector(ctx, vec)
	if err != nil {
		t.Errorf("error was not expected while inserting vec: %s", err)
	}

	vecNoTime := &AutodreamVector{
		ID:         uuid.New(),
		MissionID:  mission.ID,
		VectorData: "[0.1]",
	}

	mock.ExpectExec("INSERT INTO ohc_memory.autodream_vectors").
		WithArgs(vecNoTime.ID, vecNoTime.MissionID, vecNoTime.VectorData, sqlmock.AnyArg()).
		WillReturnResult(sqlmock.NewResult(1, 1))

	err = repo.CreateAutodreamVector(ctx, vecNoTime)
	if err != nil {
		t.Errorf("error was not expected while inserting vec without time: %s", err)
	}

	// Error scenarios
	mock.ExpectQuery("SELECT id, epic_id, title, status, assigned_agent_id FROM ohc_tasks.missions").
		WithArgs(mission.ID).
		WillReturnError(sql.ErrNoRows)

	_, err = repo.GetMission(ctx, mission.ID)
	if err == nil {
		t.Errorf("expected error, got nil")
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled expectations: %s", err)
	}
}

func TestKairosRepositoryReal(t *testing.T) {
	dbUrl := os.Getenv("DATABASE_URL")
	if dbUrl == "" {
		t.Skip("Skipping DB test as DATABASE_URL is not set")
	}

	db, err := sql.Open("postgres", dbUrl)
	if err != nil {
		t.Fatalf("Failed to open DB: %v", err)
	}
	defer db.Close()

	if err := db.Ping(); err != nil {
		t.Skipf("Skipping DB test as DB is not reachable: %v", err)
	}

	repo := NewKairosRepository(db)
	ctx := context.Background()

	mission := &Mission{
		ID:     uuid.New(),
		EpicID: uuid.New(),
		Title:  "Test Mission",
		Status: "pending",
	}

	err = repo.CreateMission(ctx, mission)
	if err != nil {
		t.Fatalf("Failed to create mission: %v", err)
	}
}
