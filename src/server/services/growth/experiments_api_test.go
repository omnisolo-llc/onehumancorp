package growth

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/src/server/lib/analytics"
)

func TestGetVariantHandler(t *testing.T) {
	manager := NewExperimentManager()
	manager.AddExperiment("exp1", "Test", 1.0)
	tracker := analytics.NewTracker()
	api := NewExperimentsAPI(manager, tracker)

	req, err := http.NewRequest("GET", "/variant?experiment_id=exp1&user_id=user1", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(api.GetVariantHandler)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp map[string]string
	err = json.NewDecoder(rr.Body).Decode(&resp)
	if err != nil {
		t.Fatal(err)
	}

	if resp["variant"] != "treatment" {
		t.Errorf("expected variant to be treatment, got %s", resp["variant"])
	}
}

func TestRecordImpressionHandler(t *testing.T) {
	manager := NewExperimentManager()
	tracker := analytics.NewTracker()
	api := NewExperimentsAPI(manager, tracker)

	body := []byte(`{"experiment_id": "exp1", "variant": "treatment"}`)
	req, err := http.NewRequest("POST", "/impression", bytes.NewBuffer(body))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(api.RecordImpressionHandler)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}

func TestRecordConversionHandler(t *testing.T) {
	manager := NewExperimentManager()
	tracker := analytics.NewTracker()
	api := NewExperimentsAPI(manager, tracker)

	body := []byte(`{"experiment_id": "exp1", "variant": "treatment"}`)
	req, err := http.NewRequest("POST", "/conversion", bytes.NewBuffer(body))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(api.RecordConversionHandler)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}
