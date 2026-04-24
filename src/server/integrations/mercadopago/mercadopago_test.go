package mercadopago

import (
	"context"
	"testing"
)

func TestProvider(t *testing.T) {
	p := NewProvider()
	if p.ID() != "mercadopago" {
		t.Errorf("expected id mercadopago, got %s", p.ID())
	}
	if p.Name() != "Mercado Pago" {
		t.Errorf("expected name Mercado Pago, got %s", p.Name())
	}
	caps := p.Capabilities()
	if len(caps) != 3 {
		t.Errorf("expected 3 capabilities, got %d", len(caps))
	}
	err := p.Initialize(context.Background(), nil)
	if err != nil {
		t.Errorf("unexpected error on Initialize: %v", err)
	}

	intent, err := p.ProcessPayment(context.Background(), 100, "BRL")
	if err != nil {
		t.Errorf("unexpected error on ProcessPayment: %v", err)
	}
	if intent == "" {
		t.Errorf("expected intent to be generated")
	}

	_, err = p.ProcessPayment(context.Background(), 0, "BRL")
	if err == nil {
		t.Errorf("expected error on ProcessPayment with zero amount")
	}

	err = p.HandleWebhook(context.Background(), nil)
	if err != nil {
		t.Errorf("unexpected error on HandleWebhook: %v", err)
	}
}
