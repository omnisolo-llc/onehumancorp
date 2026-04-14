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
