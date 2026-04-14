package onboarding

import (
	"context"
	"os"
	"strings"
	"testing"
)

func TestGenerateAuditReport_Passed(t *testing.T) {
	os.RemoveAll(".ohc-local-data")
	ProvisionEnvironment(context.Background(), false)
	defer os.RemoveAll(".ohc-local-data")

	report := GenerateAuditReport(false)
	if !strings.Contains(report, "PASSED") {
		t.Fatalf("expected report to contain PASSED, got: %s", report)
	}
	if !strings.Contains(report, "backdrop-filter: blur(20px) saturate(200%)") {
		t.Fatalf("expected report to contain OHC Glassmorphism tokens, got: %s", report)
	}
}

func TestGenerateAuditReport_Failed(t *testing.T) {
	os.RemoveAll(".ohc-cloud-data")

	report := GenerateAuditReport(true)
	if !strings.Contains(report, "FAILED") {
		t.Fatalf("expected report to contain FAILED, got: %s", report)
	}
	if !strings.Contains(report, "does not exist") {
		t.Fatalf("expected report to contain details about missing directory, got: %s", report)
	}
}

func TestRunFrictionAnalysis_Passed(t *testing.T) {
	os.RemoveAll(".ohc-local-data")
	ProvisionEnvironment(context.Background(), false)
	defer os.RemoveAll(".ohc-local-data")

	analysis := RunFrictionAnalysis(context.Background(), false)
	if analysis.FrictionScore != 10 {
		t.Fatalf("expected FrictionScore 10, got: %d", analysis.FrictionScore)
	}
}

func TestRunFrictionAnalysis_Failed(t *testing.T) {
	os.RemoveAll(".ohc-cloud-data")

	analysis := RunFrictionAnalysis(context.Background(), true)
	if analysis.FrictionScore != 80 {
		t.Fatalf("expected FrictionScore 80, got: %d", analysis.FrictionScore)
	}
}
