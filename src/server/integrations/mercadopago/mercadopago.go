package mercadopago

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"sync"
	"time"
)

type Config struct {
	AccessToken string `json:"access_token"`
	PublicKey   string `json:"public_key"`
}

type Provider struct {
	mu     sync.RWMutex
	client *http.Client
}

func NewProvider() *Provider {
	return &Provider{
		client: &http.Client{Timeout: 10 * time.Second},
	}
}

func (p *Provider) ID() string {
	return "mercadopago"
}

func (p *Provider) Name() string {
	return "Mercado Pago"
}

func (p *Provider) Description() string {
	return "Process local payments in LATAM (Pix, OXXO, etc)."
}

func (p *Provider) Capabilities() []string {
	return []string{"payments", "checkout", "webhooks"}
}

func (p *Provider) Initialize(ctx context.Context, cfg map[string]interface{}) error {
	p.mu.Lock()
	defer p.mu.Unlock()
	return nil
}

func (p *Provider) ProcessPayment(ctx context.Context, amount float64, currency string) (string, error) {
	p.mu.RLock()
	defer p.mu.RUnlock()
	if amount <= 0 {
		return "", errors.New("invalid amount")
	}

	// Example of actual API call structure for Mercado Pago Checkout API
	// req, err := http.NewRequestWithContext(ctx, "POST", "https://api.mercadopago.com/checkout/preferences", payload)
	// req.Header.Set("Authorization", "Bearer " + config.AccessToken)
	// resp, err := p.client.Do(req)

	return fmt.Sprintf("mp_intent_%d", time.Now().UnixNano()), nil
}

func (p *Provider) HandleWebhook(ctx context.Context, payload []byte) error {
	p.mu.RLock()
	defer p.mu.RUnlock()
	// Parse the webhook payload
	// Verify signature if provided by Mercado Pago
	// Handle events such as payment.updated, payment.created, etc.
	return nil
}
