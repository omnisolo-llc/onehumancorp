package onboarding

import (
    "context"
    "testing"
)

func TestInteractiveWizard_Cloud(t *testing.T) {
    w := NewInteractiveWizard()
    cfg, err := w.RunInteractiveSetup(context.Background(), true)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if cfg["mode"] != "cloud" {
        t.Errorf("expected cloud mode, got %s", cfg["mode"])
    }
}

func TestInteractiveWizard_Standalone(t *testing.T) {
    w := NewInteractiveWizard()
    cfg, err := w.RunInteractiveSetup(context.Background(), false)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    if cfg["mode"] != "standalone" {
        t.Errorf("expected standalone mode, got %s", cfg["mode"])
    }
}

func TestResetEnvironment(t *testing.T) {
    w := NewInteractiveWizard()
    ctx := context.Background()

    // Ensure a clean slate
    _ = CleanupEnvironment(ctx, false)

    err := w.ResetEnvironment(ctx, false)
    if err != nil {
        t.Fatalf("expected nil error, got %v", err)
    }

    // Verify
    err = CheckEnvironment(false)
    if err != nil {
        t.Fatalf("expected environment to exist, got %v", err)
    }

    _ = CleanupEnvironment(ctx, false)
}
