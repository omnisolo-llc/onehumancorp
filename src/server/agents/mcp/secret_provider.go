package mcp

// added for Hybrid Secrets Management MCP Proxy

import (
	"context"
	"os"
)

type SecretProvider interface {
	GetSecret(ctx context.Context, key string) (string, error)
}

func NewSecretProvider() SecretProvider {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return &CloudSecretProvider{}
	}
	if os.Getenv("OHC_STANDALONE") == "true" {
		return &LocalSecretProvider{}
	}
	// Fallback behavior if neither are set
	return &LocalSecretProvider{}
}
