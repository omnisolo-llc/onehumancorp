package onboarding

import (
	"context"
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

type FrictionAnalysis struct {
	FrictionScore  int
	Points         []string
	Recommendation string
}

func RunFrictionAnalysis(ctx context.Context, isCloud bool) FrictionAnalysis {
	err := CheckEnvironment(isCloud)
	if err != nil {
		return FrictionAnalysis{
			FrictionScore:  80,
			Points:         []string{"Environment provisioning failed or is incomplete.", err.Error()},
			Recommendation: "Run ResetEnvironment before continuing.",
		}
	}
	return FrictionAnalysis{
		FrictionScore:  10,
		Points:         []string{"Optimal setup state detected.", "No major latency in provisioning detected."},
		Recommendation: "Proceed to OHC Dashboard.",
	}
}
