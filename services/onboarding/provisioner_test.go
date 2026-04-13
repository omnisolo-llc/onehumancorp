package onboarding

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
)

func TestProvisionEnvironment_Local(t *testing.T) {
	err := ProvisionEnvironment(context.Background(), false)
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}

	expectedDirs := []string{
		filepath.Join(".ohc-local-data", "db"),
		filepath.Join(".ohc-local-data", "blob"),
		filepath.Join(".ohc-local-data", "config"),
	}

	for _, dir := range expectedDirs {
		if _, err := os.Stat(dir); os.IsNotExist(err) {
			t.Errorf("expected directory %s to exist", dir)
		}
	}

	os.RemoveAll(".ohc-local-data")
}

func TestProvisionEnvironment_Cloud(t *testing.T) {
	err := ProvisionEnvironment(context.Background(), true)
	if err != nil {
		t.Fatalf("expected nil error, got %v", err)
	}

	expectedDirs := []string{
		filepath.Join(".ohc-cloud-data", "db"),
		filepath.Join(".ohc-cloud-data", "blob"),
		filepath.Join(".ohc-cloud-data", "config"),
	}

	for _, dir := range expectedDirs {
		if _, err := os.Stat(dir); os.IsNotExist(err) {
			t.Errorf("expected directory %s to exist", dir)
		}
	}

	os.RemoveAll(".ohc-cloud-data")
}

func TestCheckEnvironment_Local(t *testing.T) {
	// Ensure clean state
	os.RemoveAll(".ohc-local-data")

	err := CheckEnvironment(false)
	if err == nil {
		t.Fatalf("expected error for missing environment, got nil")
	}

	ProvisionEnvironment(context.Background(), false)
	err = CheckEnvironment(false)
	if err != nil {
		t.Fatalf("expected nil error for provisioned environment, got %v", err)
	}
	os.RemoveAll(".ohc-local-data")
}

func TestCheckEnvironment_Cloud(t *testing.T) {
	// Ensure clean state
	os.RemoveAll(".ohc-cloud-data")

	err := CheckEnvironment(true)
	if err == nil {
		t.Fatalf("expected error for missing environment, got nil")
	}

	ProvisionEnvironment(context.Background(), true)
	err = CheckEnvironment(true)
	if err != nil {
		t.Fatalf("expected nil error for provisioned environment, got %v", err)
	}
	os.RemoveAll(".ohc-cloud-data")
}

func TestHealthHandler_Local(t *testing.T) {
	os.RemoveAll(".ohc-local-data")

	req, err := http.NewRequest("GET", "/health", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HealthHandler)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusInternalServerError {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusInternalServerError)
	}

	var res map[string]interface{}
	json.NewDecoder(rr.Body).Decode(&res)
	if res["status"] != "error" {
		t.Errorf("handler returned wrong json status: got %v want error", res["status"])
	}
	if details, ok := res["details"].(map[string]interface{}); ok {
		if details["db"] != false {
			t.Errorf("expected db details to be false")
		}
	} else {
		t.Errorf("expected details to be present in error response")
	}

	ProvisionEnvironment(context.Background(), false)

	rr2 := httptest.NewRecorder()
	handler.ServeHTTP(rr2, req)

	if status := rr2.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}

	var res2 map[string]interface{}
	json.NewDecoder(rr2.Body).Decode(&res2)
	if res2["status"] != "ok" {
		t.Errorf("handler returned wrong json status: got %v want ok", res2["status"])
	}
	if details, ok := res2["details"].(map[string]interface{}); ok {
		if details["db"] != true {
			t.Errorf("expected db details to be true")
		}
	} else {
		t.Errorf("expected details to be present in ok response")
	}

	os.RemoveAll(".ohc-local-data")
}

func TestHealthHandler_Cloud(t *testing.T) {
	os.RemoveAll(".ohc-cloud-data")

	req, err := http.NewRequest("GET", "/health?cloud=true", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HealthHandler)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusInternalServerError {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusInternalServerError)
	}

	var res map[string]interface{}
	json.NewDecoder(rr.Body).Decode(&res)
	if res["status"] != "error" {
		t.Errorf("handler returned wrong json status: got %v want error", res["status"])
	}
	if details, ok := res["details"].(map[string]interface{}); ok {
		if details["db"] != false {
			t.Errorf("expected db details to be false")
		}
	} else {
		t.Errorf("expected details to be present in error response")
	}

	ProvisionEnvironment(context.Background(), true)

	rr2 := httptest.NewRecorder()
	handler.ServeHTTP(rr2, req)

	if status := rr2.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}

	var res2 map[string]interface{}
	json.NewDecoder(rr2.Body).Decode(&res2)
	if res2["status"] != "ok" {
		t.Errorf("handler returned wrong json status: got %v want ok", res2["status"])
	}
	if details, ok := res2["details"].(map[string]interface{}); ok {
		if details["db"] != true {
			t.Errorf("expected db details to be true")
		}
	} else {
		t.Errorf("expected details to be present in ok response")
	}

	os.RemoveAll(".ohc-cloud-data")
}
