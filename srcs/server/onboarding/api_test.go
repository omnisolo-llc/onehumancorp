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
	mux.HandleFunc("/api/onboarding/status", handler.HandleGetStatus)

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

	// 3. Poll Status (Using X-Tenant-ID header)
	req, _ := http.NewRequest("GET", ts.URL+"/api/onboarding/status", nil)
	req.Header.Set("X-Tenant-ID", startRes.TenantID)
	statusResp, err := http.DefaultClient.Do(req)
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

	// 5. Poll Status Again (Using X-Tenant-ID header)
	req2, _ := http.NewRequest("GET", ts.URL+"/api/onboarding/status", nil)
	req2.Header.Set("X-Tenant-ID", startRes.TenantID)
	statusResp2, err := http.DefaultClient.Do(req2)
	assert.NoError(t, err)
	assert.Equal(t, http.StatusOK, statusResp2.StatusCode)

	var pollRes2 OnboardingResponse
	err = json.NewDecoder(statusResp2.Body).Decode(&pollRes2)
	assert.NoError(t, err)
	statusResp2.Body.Close()

	assert.Equal(t, "READY", pollRes2.Status)
}
