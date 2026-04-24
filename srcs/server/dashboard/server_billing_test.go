package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/billing/stripe"
)

func TestHandleBillingPlansReturnsPlans(t *testing.T) {
	app, server, _ := newTestServer(t)
	defer server.Close()

	req := httptest.NewRequest(http.MethodGet, "/api/billing/plans", nil)
	rec := httptest.NewRecorder()
	app.handleBillingPlans(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", rec.Code)
	}

	var plans []stripe.PlanDetail
	if err := json.NewDecoder(rec.Body).Decode(&plans); err != nil {
		t.Fatalf("decode plans: %v", err)
	}
	if len(plans) == 0 {
		t.Fatalf("expected non-empty plans list")
	}
}

func TestHandleBillingCheckoutReturnsURL(t *testing.T) {
	app, server, _ := newTestServer(t)
	defer server.Close()

	body := bytes.NewBufferString(`{"planId":"plan_starter"}`)
	req := httptest.NewRequest(http.MethodPost, "/api/billing/checkout", body)
	req.Header.Set("Content-Type", "application/json")
	rec := httptest.NewRecorder()
	app.handleBillingCheckout(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", rec.Code)
	}

	var res CheckoutResponse
	if err := json.NewDecoder(rec.Body).Decode(&res); err != nil {
		t.Fatalf("decode checkout response: %v", err)
	}
	if res.URL == "" {
		t.Fatalf("expected non-empty url")
	}
}

func TestHandleBillingPortalReturnsURL(t *testing.T) {
	app, server, _ := newTestServer(t)
	defer server.Close()

	req := httptest.NewRequest(http.MethodPost, "/api/billing/portal", nil)
	rec := httptest.NewRecorder()
	app.handleBillingPortal(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", rec.Code)
	}

	var res CheckoutResponse
	if err := json.NewDecoder(rec.Body).Decode(&res); err != nil {
		t.Fatalf("decode portal response: %v", err)
	}
	if res.URL == "" {
		t.Fatalf("expected non-empty url")
	}
}
