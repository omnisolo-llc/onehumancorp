package mcp

import (
	"context"
	"testing"
)

func TestNewSecretProvider_Local(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_MULTITENANT", "false")

	provider := NewSecretProvider()

	_, ok := provider.(*LocalSecretProvider)
	if !ok {
		t.Fatalf("expected LocalSecretProvider, got %T", provider)
	}

	secret, err := provider.GetSecret(context.Background(), "mock_key")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if secret != "mock_secret_local" {
		t.Errorf("expected mock_secret_local, got %s", secret)
	}
}

func TestNewSecretProvider_Cloud(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")

	provider := NewSecretProvider()

	_, ok := provider.(*CloudSecretProvider)
	if !ok {
		t.Fatalf("expected CloudSecretProvider, got %T", provider)
	}

	secret, err := provider.GetSecret(context.Background(), "mock_key")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if secret != "mock_secret_cloud" {
		t.Errorf("expected mock_secret_cloud, got %s", secret)
	}
}

func TestNewSecretProvider_Fallback(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "")
	t.Setenv("OHC_STANDALONE", "")

	provider := NewSecretProvider()

	_, ok := provider.(*LocalSecretProvider)
	if !ok {
		t.Fatalf("expected LocalSecretProvider for fallback, got %T", provider)
	}
}


func TestLocalSecretProvider_NotFound(t *testing.T) {
	provider := &LocalSecretProvider{}
	_, err := provider.GetSecret(context.Background(), "not_found")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}

func TestCloudSecretProvider_NotFound(t *testing.T) {
	provider := &CloudSecretProvider{}
	_, err := provider.GetSecret(context.Background(), "not_found")
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
}
