package wizard

import (
    "bytes"
    "net/http"
    "net/http/httptest"
    "testing"
)

func TestHandleConfigWizard(t *testing.T) {
    payload := []byte(`{"role": "support", "provider": "openai", "capabilities": {"Read my emails": true}, "work_hours": 4.0}`)
    req, err := http.NewRequest("POST", "/api/wizard/configure", bytes.NewBuffer(payload))
    if err != nil {
        t.Fatal(err)
    }

    rr := httptest.NewRecorder()
    handler := http.HandlerFunc(HandleConfigWizard)
    handler.ServeHTTP(rr, req)

    if status := rr.Code; status != http.StatusOK {
        t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
    }
}

func TestIsExpertMode(t *testing.T) {
    if IsExpertMode(map[string]string{"expert_mode": "true"}) != true {
        t.Errorf("expected true")
    }
}
