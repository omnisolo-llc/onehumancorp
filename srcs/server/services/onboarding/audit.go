package onboarding

import (
	"fmt"
	"strings"
)

// GenerateAuditReport creates a Day One setup audit report formatted with OHC Glassmorphism design tokens.
func GenerateAuditReport(isCloud bool) string {
	err := CheckEnvironment(isCloud)
	status := "PASSED"
	details := "All required directories are present."
	if err != nil {
		status = "FAILED"
		details = err.Error()
	}

	mode := "Standalone"
	if isCloud {
		mode = "Cloud-native"
	}

	var sb strings.Builder
	sb.WriteString("<div style=\"backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px;\">\n")
	sb.WriteString(fmt.Sprintf("  <h2>Day One Audit Report (%s)</h2>\n", mode))
	sb.WriteString(fmt.Sprintf("  <p><strong>Status:</strong> %s</p>\n", status))
	sb.WriteString(fmt.Sprintf("  <p><strong>Details:</strong> %s</p>\n", details))
	sb.WriteString("</div>")

	return sb.String()
}
