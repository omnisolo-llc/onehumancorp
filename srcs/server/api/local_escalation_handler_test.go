package api

import (
	"bytes"
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	_ "modernc.org/sqlite"
	"github.com/onehumancorp/mono/srcs/server/services/sync"
	"go.opentelemetry.io/otel/metric/noop"
)

func TestHandleLocalEscalate(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open in-memory db: %v", err)
	}
	defer db.Close()

	escalator := sync.NewEscalator(db)
	ctx := context.Background()

	err = escalator.InitSchema(ctx)
	if err != nil {
		t.Fatalf("InitSchema failed: %v", err)
	}

	meter := noop.NewMeterProvider().Meter("test")
	err = escalator.InitWithMeter(meter)
	if err != nil {
		t.Fatalf("InitWithMeter failed: %v", err)
	}

	_, err = db.ExecContext(ctx, "INSERT INTO local_mcp_rag_tasks (id, payload, escalation_status) VALUES (?, ?, ?)", "task-1", "{}", "pending")
	if err != nil {
		t.Fatalf("failed to insert test task: %v", err)
	}

	reqBody := LocalEscalateRequest{TaskID: "task-1"}
	bodyBytes, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/api/v1/orchestration/escalate", bytes.NewBuffer(bodyBytes))
	rec := httptest.NewRecorder()

	handler := HandleLocalEscalate(escalator)
	handler.ServeHTTP(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected status OK, got %d", rec.Code)
	}

	var res LocalEscalateResponse
	if err := json.NewDecoder(rec.Body).Decode(&res); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if res.Status != "success" {
		t.Fatalf("expected status 'success', got '%s'", res.Status)
	}
}
