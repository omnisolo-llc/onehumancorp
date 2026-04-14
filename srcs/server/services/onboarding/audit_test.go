package onboarding

import (
	"context"
	"strings"
	"testing"
)

func TestGenerateAuditReport_Passed(t *testing.T) {
	LocalBaseDir = t.TempDir()
	ProvisionEnvironment(context.Background(), false)

	report := GenerateAuditReport(false)
	if !strings.Contains(report, "PASSED") {
		t.Fatalf("expected report to contain PASSED, got: %s", report)
	}
	if !strings.Contains(report, "backdrop-filter: blur(20px) saturate(200%)") {
		t.Fatalf("expected report to contain OHC Glassmorphism tokens, got: %s", report)
	}
}

func TestGenerateAuditReport_Failed(t *testing.T) {
	CloudBaseDir = t.TempDir()

	report := GenerateAuditReport(true)
	if !strings.Contains(report, "FAILED") {
		t.Fatalf("expected report to contain FAILED, got: %s", report)
	}
	if !strings.Contains(report, "does not exist") {
		t.Fatalf("expected report to contain details about missing directory, got: %s", report)
	}
}
