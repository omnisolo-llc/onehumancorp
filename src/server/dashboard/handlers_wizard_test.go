package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"github.com/onehumancorp/mono/src/server/domain"
	"github.com/onehumancorp/mono/src/server/orchestration"
)

func TestWizardStateEndpoints(t *testing.T) {
	org := domain.Organization{ID: "test"}
	hub := orchestration.NewHub()
	srv := &Server{
		org: org,
		hub: hub,
		settings: hub.SettingsStore().Get(),
	}

	// Test Save
	savePayload := map[string]interface{}{
		"step":         2,
		"businessType": "Online Store",
	}
	body, _ := json.Marshal(savePayload)
	reqSave := httptest.NewRequest(http.MethodPost, "/api/wizard/state/save", bytes.NewReader(body))
	reqSave.Header.Set("Content-Type", "application/json")
	recSave := httptest.NewRecorder()

	srv.handleWizardStateSave(recSave, reqSave)

	if recSave.Code != http.StatusOK {
		t.Fatalf("Expected status 200, got %d", recSave.Code)
	}

	// Test Load
	reqLoad := httptest.NewRequest(http.MethodGet, "/api/wizard/state/load", nil)
	recLoad := httptest.NewRecorder()

	srv.handleWizardStateLoad(recLoad, reqLoad)

	if recLoad.Code != http.StatusOK {
		t.Fatalf("Expected status 200, got %d", recLoad.Code)
	}

	var loadedState map[string]interface{}
	json.NewDecoder(recLoad.Body).Decode(&loadedState)

	if step, ok := loadedState["step"].(float64); !ok || int(step) != 2 {
		t.Errorf("Expected step 2, got %v", loadedState["step"])
	}
	if bt, ok := loadedState["businessType"].(string); !ok || bt != "Online Store" {
		t.Errorf("Expected businessType 'Online Store', got %v", loadedState["businessType"])
	}
}
