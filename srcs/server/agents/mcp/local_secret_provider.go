package mcp

import (
	"context"
	"fmt"
)

// LocalSecretProvider implements SecretProvider for standalone/local environment.
type LocalSecretProvider struct {
	// mock local store could be represented here, e.g., reading from local file or env var
}

// NewLocalSecretProvider creates a new LocalSecretProvider.
func NewLocalSecretProvider() *LocalSecretProvider {
	return &LocalSecretProvider{}
}

// GetSecret fetches the secret from the local mock store.
func (p *LocalSecretProvider) GetSecret(ctx context.Context, key string) (string, error) {
	// For testing purposes, just return a mock local secret
	if key == "" {
		return "", fmt.Errorf("empty secret key")
	}
	return fmt.Sprintf("local-secret-for-%s", key), nil
}
