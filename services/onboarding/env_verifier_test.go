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

func TestVerifyEnvironment_StandaloneTelemetry(t *testing.T) {
	env := map[string]string{
		"OHC_SOURCE_MODE": "standalone",
		"OHC_TELEMETRY_ENABLED": "true",
	}

	config, err := VerifyEnvironment(env)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if !config.TelemetryEnabled {
		t.Errorf("expected telemetry to be true")
	}
}

func TestVerifyEnvironment_ThinClient(t *testing.T) {
	env := map[string]string{
		"OHC_SOURCE_MODE": "thin_client",
		"OHC_API_ENDPOINT": "https://api.ohc.io",
	}

	config, err := VerifyEnvironment(env)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if config.Mode != "thin_client" {
		t.Errorf("expected mode thin_client, got %s", config.Mode)
	}
	if config.ApiEndpoint != "https://api.ohc.io" {
		t.Errorf("expected api endpoint https://api.ohc.io, got %s", config.ApiEndpoint)
	}
	if !config.TelemetryEnabled {
		t.Errorf("expected telemetry to be true by default in thin_client")
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
