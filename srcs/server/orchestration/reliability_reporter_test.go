package orchestration

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestGenerateReliabilityReport(t *testing.T) {
	modes := []string{"ModeA", "ModeB"}
	report := GenerateReliabilityReport(10, 2, modes)

	if !strings.Contains(report, "OHC Chaos Resilience Report") {
		t.Error("report missing title")
	}
	if !strings.Contains(report, "ModeA") || !strings.Contains(report, "ModeB") {
		t.Error("report missing chaos modes")
	}
	// Success rate is 10/12 = 83.333%
	if !strings.Contains(report, "83.3%") {
		t.Errorf("report missing or incorrect success rate, got: %s", report)
	}
	if !strings.Contains(report, "backdrop-filter: blur(20px)") {
		t.Error("report missing Glassmorphism styling")
	}
}

func TestGenerateReliabilityReport_ZeroTests(t *testing.T) {
	report := GenerateReliabilityReport(0, 0, []string{})
	// The space between Rate and | and | and 0.0% might be multiple spaces or tabs
	// Use a more robust check
	if !strings.Contains(report, "0.0%") {
		t.Errorf("expected 0.0%% in report for zero tests, got: %s", report)
	}
}

func TestSaveReliabilityReport(t *testing.T) {
	tmpDir := t.TempDir()
	path := filepath.Join(tmpDir, "report.md")
	modes := []string{"Chaos"}

	err := SaveReliabilityReport(path, 1, 0, modes)
	if err != nil {
		t.Fatalf("failed to save report: %v", err)
	}

	content, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("failed to read saved report: %v", err)
	}

	if !strings.Contains(string(content), "Chaos") {
		t.Error("saved report missing content")
	}
}
