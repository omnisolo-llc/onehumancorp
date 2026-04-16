package onboarding

import (
	"context"
	"testing"
)

func TestRunCLI_Cloud(t *testing.T) {
	ctx := context.Background()
	err := RunCLI(ctx, "cloud")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

func TestRunCLI_Standalone(t *testing.T) {
	ctx := context.Background()
	err := RunCLI(ctx, "standalone")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}

func TestRunCLI_Invalid(t *testing.T) {
	ctx := context.Background()
	err := RunCLI(ctx, "invalid")
	if err == nil {
		t.Errorf("expected error for invalid mode")
	}
}
