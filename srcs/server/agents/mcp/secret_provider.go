package mcp

// added for Hybrid Secrets Management MCP Proxy

import (
	"context"
	"os"
)

// SecretProvider defines the interface for retrieving secrets from various storage backends.
// It allows agents to access secrets securely without directly handling sensitive data.
type SecretProvider interface {
	GetSecret(ctx context.Context, key string) (string, error)
}

// NewSecretProvider evaluates OHC_MULTITENANT and OHC_STANDALONE to return the correct provider.
// This factory pattern evaluates environment variables at boot time and injects the appropriate provider.
func NewSecretProvider() SecretProvider {
	isMultiTenant := os.Getenv("OHC_MULTITENANT") == "true"
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"

	if isMultiTenant {
		return &CloudSecretProvider{}
	}

	if isStandalone {
		return &LocalSecretProvider{}
	}

	// Fallback behavior if neither are set
	return &LocalSecretProvider{}
}
