package billing

import (
	"bytes"

	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/services/billing"
)

func TestGetInvoices(t *testing.T) {
	service := billing.NewService()
	handler := &Handler{Service: service}

	req := httptest.NewRequest("GET", "/invoices?tenant_id=test-tenant", nil)
	rr := httptest.NewRecorder()

	handler.GetInvoices(rr, req)

	if rr.Code != http.StatusOK {
		t.Errorf("expected status OK, got %v", rr.Code)
	}
}

func TestRecordUsage(t *testing.T) {
	service := billing.NewService()
	handler := &Handler{Service: service}

	body := []byte(`{"tenant_id": "test-tenant", "resource_type": "rag_query", "units": 1}`)
	req := httptest.NewRequest("POST", "/usage", bytes.NewBuffer(body))
	rr := httptest.NewRecorder()

	handler.RecordUsage(rr, req)

	if rr.Code != http.StatusCreated {
		t.Errorf("expected status Created, got %v", rr.Code)
	}
}
