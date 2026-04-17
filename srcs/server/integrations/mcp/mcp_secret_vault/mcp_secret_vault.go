package mcp_secret_vault

import (
	"context"
	"errors"
	"fmt"
)

// SecretStorage defines the interface for secret storage adapters.
type SecretStorage interface {
	GetSecret(ctx context.Context, key string, tenantID string) (string, error)
	SetSecret(ctx context.Context, key string, value string, tenantID string) error
}

// MCPSecretVault is the unified tool for secret management.
type MCPSecretVault struct {
	mode    string
	adapter SecretStorage
}

// NewMCPSecretVault creates a new MCPSecretVault.
func NewMCPSecretVault(mode string, adapter SecretStorage) (*MCPSecretVault, error) {
	if mode != "cloud" && mode != "local" {
		return nil, errors.New("invalid mode, must be 'cloud' or 'local'")
	}
	if adapter == nil {
		return nil, errors.New("adapter cannot be nil")
	}
	return &MCPSecretVault{
		mode:    mode,
		adapter: adapter,
	}, nil
}

// GetSecret retrieves a secret.
func (v *MCPSecretVault) GetSecret(ctx context.Context, key string, tenantID string) (string, error) {
	if key == "" {
		return "", errors.New("key cannot be empty")
	}
	if v.mode == "cloud" && tenantID == "" {
		return "", errors.New("tenantID is required in cloud mode")
	}

	val, err := v.adapter.GetSecret(ctx, key, tenantID)
	if err != nil {
		return "", fmt.Errorf("failed to get secret: %w", err)
	}
	return val, nil
}

// SetSecret stores a secret.
func (v *MCPSecretVault) SetSecret(ctx context.Context, key string, value string, tenantID string) error {
	if key == "" {
		return errors.New("key cannot be empty")
	}
	if value == "" {
		return errors.New("value cannot be empty")
	}
	if v.mode == "cloud" && tenantID == "" {
		return errors.New("tenantID is required in cloud mode")
	}

	err := v.adapter.SetSecret(ctx, key, value, tenantID)
	if err != nil {
		return fmt.Errorf("failed to set secret: %w", err)
	}
	return nil
}
