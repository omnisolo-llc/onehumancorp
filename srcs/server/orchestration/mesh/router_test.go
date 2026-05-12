package mesh

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/DATA-DOG/go-sqlmock"
	"github.com/go-redis/redismock/v8"
)

func TestRouter_RouteJob_Success(t *testing.T) {
	// Mock Postgres
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	// Setup expectations for Postgres
	rows := sqlmock.NewRows([]string{"id"}).AddRow("agent-123")
	mock.ExpectQuery("SELECT id FROM agent_profiles WHERE status = 'available' AND \\$1 = ANY\\(skills\\) LIMIT 1").
		WithArgs("support").
		WillReturnRows(rows)

	// Mock Redis
	redisClient, redisMock := redismock.NewClientMock()

	// Expected Redis Payload
	expectedPayload := map[string]interface{}{
		"agent_id": "agent-123",
		"action":   "TaskAssigned",
		"status":   "pending",
		"payload": map[string]interface{}{
			"job_id":         "job-999",
			"required_skill": "support",
			"data": map[string]interface{}{
				"customer_id": "cust-001",
			},
		},
	}
	jsonBytes, _ := json.Marshal(expectedPayload)

	redisMock.ExpectPublish("mesh:agent:agent-123", jsonBytes).SetVal(1)

	// Create Router
	router := NewRouter(db, redisClient)

	// Execute RouteJob
	ctx := context.Background()
	payload := map[string]interface{}{
		"customer_id": "cust-001",
	}

	err = router.RouteJob(ctx, "job-999", "support", payload)
	if err != nil {
		t.Errorf("expected no error, but got: %v", err)
	}

	// Verify expectations
	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled Postgres expectations: %s", err)
	}
	if err := redisMock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled Redis expectations: %s", err)
	}
}

func TestRouter_RouteJob_NoAvailableAgents(t *testing.T) {
	db, mock, err := sqlmock.New()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer db.Close()

	mock.ExpectQuery("SELECT id FROM agent_profiles WHERE status = 'available' AND \\$1 = ANY\\(skills\\) LIMIT 1").
		WithArgs("sales").
		WillReturnRows(sqlmock.NewRows([]string{"id"})) // empty rows

	redisClient, _ := redismock.NewClientMock()
	router := NewRouter(db, redisClient)

	ctx := context.Background()
	err = router.RouteJob(ctx, "job-888", "sales", nil)
	if err == nil {
		t.Error("expected error due to no available agents, but got nil")
	}

	if err := mock.ExpectationsWereMet(); err != nil {
		t.Errorf("there were unfulfilled Postgres expectations: %s", err)
	}
}
