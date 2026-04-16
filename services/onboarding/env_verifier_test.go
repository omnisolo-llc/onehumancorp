package onboarding

import (
	"testing"
)

func TestVerifyEnvironment_Standalone(t *testing.T) {
	env := map[string]string{
		"OHC_SOURCE_MODE": "standalone",
		"OHC_MULTITENANT": "false",
	}

	config, err := VerifyEnvironment(env)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if config.Mode != "standalone" {
		t.Errorf("expected mode standalone, got %s", config.Mode)
	}
	if config.MultiTenant {
		t.Errorf("expected multitenant false")
	}
}

func TestVerifyEnvironment_CloudInvalid(t *testing.T) {
	env := map[string]string{
		"OHC_SOURCE_MODE": "cloud",
		"OHC_MULTITENANT": "false",
	}

	_, err := VerifyEnvironment(env)
	if err == nil {
		t.Fatalf("expected error for cloud mode without multitenant")
	}
}

func TestVerifyEnvironment_MissingMode(t *testing.T) {
	env := map[string]string{
		"OHC_MULTITENANT": "false",
	}

	_, err := VerifyEnvironment(env)
	if err == nil {
		t.Fatalf("expected error for missing mode")
	}
}

func TestVerifyEnvironment_ThinClient(t *testing.T) {
	env := map[string]string{
		"OHC_SOURCE_MODE":     "thin_client",
		"OHC_REMOTE_ENDPOINT": "https://api.onehumancorp.com",
	}

	config, err := VerifyEnvironment(env)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if config.Mode != "thin_client" {
		t.Errorf("expected mode thin_client, got %s", config.Mode)
	}
	if config.RemoteEndpoint != "https://api.onehumancorp.com" {
		t.Errorf("expected remote endpoint https://api.onehumancorp.com, got %s", config.RemoteEndpoint)
	}
}

func TestVerifyEnvironment_ThinClientMissingEndpoint(t *testing.T) {
	env := map[string]string{
		"OHC_SOURCE_MODE": "thin_client",
	}

	_, err := VerifyEnvironment(env)
	if err == nil {
		t.Fatalf("expected error for thin_client mode without endpoint")
	}
}
