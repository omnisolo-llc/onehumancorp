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
    sb.WriteString("<div style=\"backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px;\">\n")
    sb.WriteString(fmt.Sprintf("  <h2>OHC Interactive Setup (%s)</h2>\n", mode))
    sb.WriteString("  <p>Please review your configuration options.</p>\n")
    sb.WriteString("</div>")

    return sb.String()
}
