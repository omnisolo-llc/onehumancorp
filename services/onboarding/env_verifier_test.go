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
