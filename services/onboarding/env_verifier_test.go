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
	if config.DatabaseURL != "file:./ohc.db?cache=shared" {
		t.Errorf("expected default database url, got %s", config.DatabaseURL)
	}
}

func TestVerifyEnvironment_Cloud(t *testing.T) {
	env := map[string]string{
		"OHC_SOURCE_MODE": "cloud",
		"OHC_MULTITENANT": "true",
		"OHC_POSTGRES_URL": "postgres://ohc-cloud-spiffe-user@localhost:5432/ohc",
		"OHC_REDIS_URL": "redis://ohc-cloud-spiffe-user@localhost:6379",
	}

	config, err := VerifyEnvironment(env)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if config.Mode != "cloud" {
		t.Errorf("expected mode cloud, got %s", config.Mode)
	}
	if !config.MultiTenant {
		t.Errorf("expected multitenant true")
	}
	if config.DatabaseURL == "" || config.RedisURL == "" {
		t.Errorf("expected db and redis urls to be set")
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

func TestVerifyEnvironment_CloudMissingDb(t *testing.T) {
	env := map[string]string{
		"OHC_SOURCE_MODE": "cloud",
		"OHC_MULTITENANT": "true",
		"OHC_REDIS_URL": "redis://ohc-cloud-spiffe-user@localhost:6379",
	}

	_, err := VerifyEnvironment(env)
	if err == nil {
		t.Fatalf("expected error for cloud mode without db")
	}
}

func TestVerifyEnvironment_CloudInvalidSpiffe(t *testing.T) {
	env := map[string]string{
		"OHC_SOURCE_MODE": "cloud",
		"OHC_MULTITENANT": "true",
		"OHC_POSTGRES_URL": "postgres://user:pass@localhost:5432/ohc",
		"OHC_REDIS_URL": "redis://ohc-cloud-spiffe-user@localhost:6379",
	}

	_, err := VerifyEnvironment(env)
	if err == nil {
		t.Fatalf("expected error for cloud mode with invalid spiffe string")
	}
}

func TestVerifyEnvironment_CloudInvalidRedisSpiffe(t *testing.T) {
	env := map[string]string{
		"OHC_SOURCE_MODE": "cloud",
		"OHC_MULTITENANT": "true",
		"OHC_POSTGRES_URL": "postgres://ohc-cloud-spiffe-user@localhost:5432/ohc",
		"OHC_REDIS_URL": "redis://user:pass@localhost:6379",
	}

	_, err := VerifyEnvironment(env)
	if err == nil {
		t.Fatalf("expected error for cloud mode with invalid redis spiffe string")
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
