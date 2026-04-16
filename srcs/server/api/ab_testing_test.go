package api

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandleABVariant(t *testing.T) {
	reqBody := ABVariantRequest{
		Experiment: "test_exp",
		Variants:   []string{"A", "B"},
		Weights:    []int{100, 0},
	}
	body, _ := json.Marshal(reqBody)

	req, _ := http.NewRequest(http.MethodPost, "/api/ab/variant", bytes.NewReader(body))
	rr := httptest.NewRecorder()

	HandleABVariant(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("Expected status 200, got %d", rr.Code)
	}

	var resp ABVariantResponse
	json.NewDecoder(rr.Body).Decode(&resp)

	if resp.Variant != "A" {
		t.Errorf("Expected variant A, got %s", resp.Variant)
	}
}

func TestHandleABConversion(t *testing.T) {
	reqBody := ABConversionRequest{
		Experiment: "test_exp",
		Variant:    "A",
	}
	body, _ := json.Marshal(reqBody)

	req, _ := http.NewRequest(http.MethodPost, "/api/ab/conversion", bytes.NewReader(body))
	rr := httptest.NewRecorder()

	HandleABConversion(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("Expected status 200, got %d", rr.Code)
	}
}
