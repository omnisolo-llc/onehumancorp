package onboarding

import (
    "context"
    "fmt"
    "strings"
)

type InteractiveWizard struct{}

func NewInteractiveWizard() *InteractiveWizard {
    return &InteractiveWizard{}
}

func (w *InteractiveWizard) RunInteractiveSetup(ctx context.Context, isCloud bool) (map[string]string, error) {
    // Run preflight check before setting up
    preflightRes := RunPreflightCheck(isCloud)
    if !preflightRes.Passed {
        return nil, fmt.Errorf("preflight check failed: %s", preflightRes.Message)
    }

    if isCloud {
        return map[string]string{
            "mode":  "cloud",
            "db":    "postgres",
            "cache": "redis",
        }, nil
    }
    return map[string]string{
        "mode":  "standalone",
        "db":    "sqlite",
        "cache": "memory",
    }, nil
}

func (w *InteractiveWizard) GenerateWizardUI(isCloud bool) string {
    mode := "Standalone"
    if isCloud {
        mode = "Cloud-native"
    }

    var sb strings.Builder
    sb.WriteString("<div style=\"backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 24px; border-radius: 16px; border: 1px solid rgba(255, 255, 255, 0.1); box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);\">\n")
    sb.WriteString(fmt.Sprintf("  <h2 style=\"margin-top: 0; color: #ffffff; font-weight: 600; font-size: 24px;\">OHC Interactive Setup (%s)</h2>\n", mode))
    sb.WriteString("  <p style=\"color: rgba(255, 255, 255, 0.7); font-size: 16px; line-height: 1.5; margin-bottom: 0;\">Please review your configuration options.</p>\n")
    sb.WriteString("</div>")

    return sb.String()
}

func (w *InteractiveWizard) ResetEnvironment(ctx context.Context, isCloud bool) error {
    if err := CleanupEnvironment(ctx, isCloud); err != nil {
        return fmt.Errorf("cleanup failed: %w", err)
    }
    if err := ProvisionEnvironment(ctx, isCloud); err != nil {
        return fmt.Errorf("provision failed: %w", err)
    }
    return nil
}
