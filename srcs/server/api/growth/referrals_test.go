package growth

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandleReferrals(t *testing.T) {
	handler := HandleReferrals()

	reqBody := []byte(`{"email":"test@example.com", "source":"standalone", "campaign_id":"c1"}`)
	req, err := http.NewRequest("POST", "/api/growth/referrals", bytes.NewBuffer(reqBody))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var response map[string]interface{}
	err = json.Unmarshal(rr.Body.Bytes(), &response)
	if err != nil {
		t.Fatal(err)
	}

	if response["status"] != "success" {
		t.Errorf("expected success status, got %v", response["status"])
	}
	if response["redacted_email"] != "[REDACTED_EMAIL]" {
		t.Errorf("expected email to be redacted, got %v", response["redacted_email"])
	}
}
