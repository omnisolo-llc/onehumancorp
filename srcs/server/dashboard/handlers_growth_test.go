package dashboard

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandleOnboardingMetrics(t *testing.T) {
	req, err := http.NewRequest("GET", "/api/dashboard/onboarding/metrics", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HandleOnboardingMetrics)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp []OnboardingMetric
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatal(err)
	}

	if len(resp) != 3 {
		t.Errorf("expected 3 metrics, got %d", len(resp))
	}
}

func TestHandleViralCoefficient(t *testing.T) {
	req, err := http.NewRequest("GET", "/api/dashboard/growth/viral-coefficient", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HandleViralCoefficient)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp ViralCoefficientResponse
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatal(err)
	}

	if resp.KFactor != 1.2 {
		t.Errorf("expected k_factor 1.2, got %f", resp.KFactor)
	}
}
