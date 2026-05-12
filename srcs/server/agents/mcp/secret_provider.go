package mcp

import (
	"context"
	"os"
)

// SecretProvider defines the interface for fetching secrets.
type SecretProvider interface {
	GetSecret(ctx context.Context, key string) (string, error)
}

// NewSecretProvider returns the appropriate SecretProvider based on the environment.
func NewSecretProvider() SecretProvider {
	isStandalone := os.Getenv("OHC_STANDALONE") == "true"
	isMultitenant := os.Getenv("OHC_MULTITENANT") == "true"

	if isMultitenant && !isStandalone {
		return NewCloudSecretProvider()
	}

	return NewLocalSecretProvider()
}
