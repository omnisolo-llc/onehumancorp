package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestUltraPlanAPIEndpoints(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	prov := db.NewTestProvider(t)
	defer prov.Close()

	ctx := context.Background()
	_, _ = prov.Exec(ctx, `
		CREATE TABLE IF NOT EXISTS swarm_ultra_plans (
			id TEXT PRIMARY KEY,
			mission_id TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'DELIBERATING',
			state_machine TEXT DEFAULT '{}',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS ultraplan_proposals (id TEXT PRIMARY KEY, plan_id TEXT NOT NULL, status TEXT NOT NULL);
		CREATE TABLE IF NOT EXISTS ultraplan_votes (plan_id TEXT NOT NULL, agent_id TEXT NOT NULL, vote TEXT NOT NULL);
	`)

	upm := NewUltraPlanManager(prov, nil, nil)
	api := NewUltraPlanAPI(upm)
	mux := http.NewServeMux()
	api.RegisterRoutes(mux)

	// Create
	reqBody := `{"mission_id":"m-123","state_machine":{}}`
	req := httptest.NewRequest(http.MethodPost, "/api/ultraplan/create", bytes.NewBufferString(reqBody))
	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", rr.Code)
	}

	var plan UltraPlan
	json.NewDecoder(rr.Body).Decode(&plan)

	// Vote
	reqBody = `{"plan_id":"` + plan.ID + `","agent_id":"a-1","vote":"APPROVE"}`
	req = httptest.NewRequest(http.MethodPost, "/api/ultraplan/vote", bytes.NewBufferString(reqBody))
	rr = httptest.NewRecorder()
	mux.ServeHTTP(rr, req)
	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", rr.Code)
	}

	// Finalize
	reqBody = `{"plan_id":"` + plan.ID + `"}`
	req = httptest.NewRequest(http.MethodPost, "/api/ultraplan/finalize", bytes.NewBufferString(reqBody))
	rr = httptest.NewRecorder()
	mux.ServeHTTP(rr, req)
	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", rr.Code)
	}
}
