package mcp_secret_vault

import (
	"context"

	"github.com/zalando/go-keyring"
)

const serviceName = "ohc-standalone"

// LocalAdapter implements SecretStorage using the OS keyring.
type LocalAdapter struct{}

// NewLocalAdapter creates a new LocalAdapter.
func NewLocalAdapter() *LocalAdapter {
	return &LocalAdapter{}
}

// GetSecret retrieves a secret from the OS keyring.
func (a *LocalAdapter) GetSecret(ctx context.Context, key string, tenantID string) (string, error) {
    // tenantID is ignored in local mode as it's a single-user standalone environment
	return keyring.Get(serviceName, key)
}

// SetSecret stores a secret in the OS keyring.
func (a *LocalAdapter) SetSecret(ctx context.Context, key string, value string, tenantID string) error {
	return keyring.Set(serviceName, key, value)
}
