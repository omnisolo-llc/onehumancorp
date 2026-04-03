package onboarding

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestRunHealthCheck_Standalone(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("DATABASE_URL")

	ctx := context.Background()
	provider, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}
	defer provider.Close()

	status := RunHealthCheck(ctx, provider)

	if !status.IsStandalone {
		t.Errorf("Expected IsStandalone to be true")
	}
	if !status.DatabaseReady {
		t.Errorf("Expected DatabaseReady to be true for SQLite")
	}
	if len(status.MissingEnvVars) > 0 {
		t.Errorf("Expected no missing env vars, got %v", status.MissingEnvVars)
	}
}

func TestRunHealthCheck_Cloud(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	// Intentionally omit REDIS_URL to test missing variables logic
	os.Setenv("DATABASE_URL", "postgres://test")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("DATABASE_URL")
	defer os.Unsetenv("REDIS_URL")

	ctx := context.Background()

	status := RunHealthCheck(ctx, nil) // use nil to simulate failed connection/postgres mock

	if status.IsStandalone {
		t.Errorf("Expected IsStandalone to be false")
	}
	if status.DatabaseReady {
		t.Errorf("Expected DatabaseReady to be false since provider is nil")
	}
	if len(status.MissingEnvVars) != 1 || status.MissingEnvVars[0] != "REDIS_URL" {
		t.Errorf("Expected REDIS_URL to be missing, got %v", status.MissingEnvVars)
	}
}
