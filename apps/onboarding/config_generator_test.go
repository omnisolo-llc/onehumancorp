package onboarding

import (
	"context"
	"testing"
)

func TestGenerateDayOneConfig_Cloud(t *testing.T) {
	ctx := context.Background()
	cfg, err := GenerateDayOneConfig(ctx, "cloud")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if cfg.Mode != "cloud" {
		t.Errorf("expected cloud mode")
	}
	if cfg.DatabaseURL == "" {
		t.Errorf("expected DB URL")
	}
}

func TestGenerateDayOneConfig_Standalone(t *testing.T) {
	ctx := context.Background()
	cfg, err := GenerateDayOneConfig(ctx, "standalone")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if cfg.Mode != "standalone" {
		t.Errorf("expected standalone mode")
	}
	if cfg.DatabaseURL != "sqlite:///.ohc-local-data/db/ohc.db" {
		t.Errorf("expected sqlite db")
	}
}

func TestGenerateDayOneConfig_Invalid(t *testing.T) {
	ctx := context.Background()
	_, err := GenerateDayOneConfig(ctx, "invalid")
	if err == nil {
		t.Errorf("expected error for invalid mode")
	}
}
