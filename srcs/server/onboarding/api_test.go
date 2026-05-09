package onboarding

import (
	"bytes"
	"context"

	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"onehumancorp/srcs/server/orchestration"

	_ "github.com/mattn/go-sqlite3"
	"github.com/stretchr/testify/assert"
)

func TestAPIEndToEndFlow(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	tenantStore := NewSqliteTenantStore(db)
	taskStore := orchestration.NewSqliteTaskStore(db)
	service := NewService(tenantStore, taskStore)
	handler := NewAPIHandler(service)

	// Setup mock router/mux
	mux := http.NewServeMux()
	mux.HandleFunc("/api/onboarding/start", handler.HandleStartOnboarding)
	mux.HandleFunc("/api/onboarding/status", TenantAuthMiddleware(handler.HandleGetStatus))

	// Create test server
	ts := httptest.NewServer(mux)
	defer ts.Close()

	// 1. Start Onboarding
	reqData := OnboardingRequest{
		Name:        "Carlos Repairs",
		Category:    "Service",
		Description: "General handyman repairs",
	}
	reqBody, _ := json.Marshal(reqData)

	resp, err := http.Post(ts.URL+"/api/onboarding/start", "application/json", bytes.NewBuffer(reqBody))
	assert.NoError(t, err)
	assert.Equal(t, http.StatusAccepted, resp.StatusCode)

	var startRes OnboardingResponse
	err = json.NewDecoder(resp.Body).Decode(&startRes)
	assert.NoError(t, err)
	resp.Body.Close()

	assert.NotEmpty(t, startRes.TenantID)
	assert.Equal(t, "PROVISIONING", startRes.Status)

	// 2. Check Tasks Dispatched in DB
	ctx := context.Background()
	tasks, err := taskStore.GetTasksByOrganization(ctx, startRes.TenantID)
	assert.NoError(t, err)
	assert.Len(t, tasks, 3)

	// 3. Poll Status
	req1, err := http.NewRequest("GET", ts.URL+"/api/onboarding/status", nil)
	assert.NoError(t, err)
	req1.Header.Set("X-Tenant-Id", startRes.TenantID)
	statusResp, err := http.DefaultClient.Do(req1)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, statusResp.StatusCode)

	var pollRes OnboardingResponse
	err = json.NewDecoder(statusResp.Body).Decode(&pollRes)
	assert.NoError(t, err)
	statusResp.Body.Close()

	assert.Equal(t, "PROVISIONING", pollRes.Status)

	// 4. Complete Tasks
	for _, task := range tasks {
		err := taskStore.UpdateTaskStatus(ctx, task.ID, "COMPLETED")
		assert.NoError(t, err)
	}

	// 5. Poll Status Again
	req2, _ := http.NewRequest("GET", ts.URL+"/api/onboarding/status", nil)
	req2.Header.Set("X-Tenant-Id", startRes.TenantID)
	statusResp2, err := http.DefaultClient.Do(req2)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, statusResp2.StatusCode)

	var pollRes2 OnboardingResponse
	err = json.NewDecoder(statusResp2.Body).Decode(&pollRes2)
	assert.NoError(t, err)
	statusResp2.Body.Close()

	assert.Equal(t, "READY", pollRes2.Status)
}

func TestAPIStateFlow(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	tenantStore := NewSqliteTenantStore(db)
	taskStore := orchestration.NewSqliteTaskStore(db)
	service := NewService(tenantStore, taskStore)
	handler := NewAPIHandler(service)

	// Setup mock router/mux
	mux := http.NewServeMux()
	mux.HandleFunc("/api/onboarding/state", TenantAuthMiddleware(func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost {
			handler.HandleSaveState(w, r)
		} else if r.Method == http.MethodGet {
			handler.HandleGetState(w, r)
		} else {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		}
	}))

	// Create test server
	ts := httptest.NewServer(mux)
	defer ts.Close()

    // 1. Create a tenant manually to test state against
    tenant := &Tenant{
		Name:        "Test State",
		Category:    "Tech",
		Status:      "PROVISIONING",
	}
	err := tenantStore.CreateTenant(context.Background(), tenant)
    assert.NoError(t, err)

    // 2. Save State
	reqData := TenantStateRequest{
		State: "{\"currentStep\":2}",
	}
	reqBody, _ := json.Marshal(reqData)

	req1, err := http.NewRequest("POST", ts.URL+"/api/onboarding/state", bytes.NewBuffer(reqBody))
	assert.NoError(t, err)
	req1.Header.Set("X-Tenant-Id", tenant.ID)
	resp, err := http.DefaultClient.Do(req1)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusNoContent, resp.StatusCode)
	resp.Body.Close()

    // 3. Get State
	req2, err := http.NewRequest("GET", ts.URL+"/api/onboarding/state", nil)
	assert.NoError(t, err)
	req2.Header.Set("X-Tenant-Id", tenant.ID)
	resp2, err := http.DefaultClient.Do(req2)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, resp2.StatusCode)

	var stateRes TenantStateResponse
	err = json.NewDecoder(resp2.Body).Decode(&stateRes)
	assert.NoError(t, err)
	resp2.Body.Close()

	assert.Equal(t, "{\"currentStep\":2}", stateRes.State)
}
