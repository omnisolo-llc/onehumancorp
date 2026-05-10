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

	// 1. Start Onboarding
	reqPayload := OnboardingRequest{
		Name:        "Carlos Fixes It",
		Category:    "Service",
		Description: "Local handyman services",
	}
	body, _ := json.Marshal(reqPayload)
	req := httptest.NewRequest(http.MethodPost, "/api/onboarding/start", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	rr := httptest.NewRecorder()

	mux.ServeHTTP(rr, req)
	assert.Equal(t, http.StatusAccepted, rr.Code)

	var res OnboardingResponse
	err := json.Unmarshal(rr.Body.Bytes(), &res)
	assert.NoError(t, err)
	assert.NotEmpty(t, res.TenantID)
	assert.Equal(t, "PROVISIONING", res.Status)

	tenantID := res.TenantID

	// 2. Check Tasks Dispatched
	tasks, err := taskStore.GetTasksByOrganization(context.Background(), tenantID)
	assert.NoError(t, err)
	assert.Len(t, tasks, 3) // We expect 3 initial onboarding tasks

	// 3. Check Status (Should still be PROVISIONING)
	req2 := httptest.NewRequest(http.MethodGet, "/api/onboarding/status", nil)
	req2.Header.Set("x-tenant-id", tenantID)
	rr2 := httptest.NewRecorder()

	mux.ServeHTTP(rr2, req2)
	assert.Equal(t, http.StatusOK, rr2.Code)

	var statusRes OnboardingResponse
	err = json.Unmarshal(rr2.Body.Bytes(), &statusRes)
	assert.NoError(t, err)
	assert.Equal(t, "PROVISIONING", statusRes.Status)

	// 4. Mock Agents Completing Tasks
	for _, task := range tasks {
		err := taskStore.UpdateTaskStatus(context.Background(), task.ID, "COMPLETED")
		assert.NoError(t, err)
	}

	// 5. Check Status Again (Should be READY)
	req3 := httptest.NewRequest(http.MethodGet, "/api/onboarding/status", nil)
	req3.Header.Set("x-tenant-id", tenantID)
	rr3 := httptest.NewRecorder()

	mux.ServeHTTP(rr3, req3)
	assert.Equal(t, http.StatusOK, rr3.Code)

	var statusRes2 OnboardingResponse
	err = json.Unmarshal(rr3.Body.Bytes(), &statusRes2)
	assert.NoError(t, err)
	assert.Equal(t, "READY", statusRes2.Status)
}

func TestAPIStateFlow(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	tenantStore := NewSqliteTenantStore(db)
	taskStore := orchestration.NewSqliteTaskStore(db)
	service := NewService(tenantStore, taskStore)
	handler := NewAPIHandler(service)

	// Create dummy tenant to update state on
	tenantID := "tenant-for-state"
	_ = tenantStore.CreateTenant(context.Background(), &Tenant{
		ID:    tenantID,
		Name:  "Stateful Co",
		State: "{}",
	})

	mux := http.NewServeMux()
	mux.HandleFunc("/api/onboarding/state", TenantAuthMiddleware(handler.HandleSaveState))
    mux.HandleFunc("/api/onboarding/state/get", TenantAuthMiddleware(handler.HandleGetState))

	// 1. Update State
	statePayload := map[string]interface{}{
		"state": "{\"currentStep\":2}",
	}
	body, _ := json.Marshal(statePayload)
	req := httptest.NewRequest(http.MethodPost, "/api/onboarding/state", bytes.NewBuffer(body))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("x-tenant-id", tenantID)
	rr := httptest.NewRecorder()

	mux.ServeHTTP(rr, req)
	assert.Equal(t, http.StatusNoContent, rr.Code)

	// 2. Get State
	req2 := httptest.NewRequest(http.MethodGet, "/api/onboarding/state/get", nil)
	req2.Header.Set("x-tenant-id", tenantID)
	rr2 := httptest.NewRecorder()

	mux.ServeHTTP(rr2, req2)
	assert.Equal(t, http.StatusOK, rr2.Code)

	var fetchedState map[string]interface{}
	err := json.Unmarshal(rr2.Body.Bytes(), &fetchedState)
	assert.NoError(t, err)

	// In the real DB it might be a JSON string, depending on store implementation,
	// for our sqlite mock it's a string, so we'll just check raw bytes.
	assert.Equal(t, `{"state":"{\"currentStep\":2}"}
`, string(rr2.Body.Bytes()))
}
