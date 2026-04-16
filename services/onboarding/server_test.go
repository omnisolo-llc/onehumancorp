package onboarding

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestProvisionHandler(t *testing.T) {
	reqBody, _ := json.Marshal(ProvisionRequest{})
	req := httptest.NewRequest(http.MethodPost, "/api/provision", bytes.NewReader(reqBody))
	req.Header.Set("Content-Type", "application/json")

	rr := httptest.NewRecorder()
	ProvisionHandler(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var res map[string]string
	json.NewDecoder(rr.Body).Decode(&res)
	if res["status"] != "provisioned" {
		t.Errorf("handler returned unexpected body: got %v", res)
	}
}
