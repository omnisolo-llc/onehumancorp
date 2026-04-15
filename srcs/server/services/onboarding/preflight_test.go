package onboarding

import (
	"strings"
	"testing"
)

func TestRunPreflightCheck(t *testing.T) {
	res := RunPreflightCheck(false)
	if !res.Passed {
		t.Errorf("Expected standalone mode to pass on this machine")
	}

	resCloud := RunPreflightCheck(true)
	// We only fail if the actual number of CPUs < 2
	if resCloud.NumCPUs < 2 && resCloud.Passed {
		t.Errorf("Expected cloud mode to fail with < 2 CPUs")
	}
}

func TestGeneratePreflightReport(t *testing.T) {
	res := PreflightResult{
		OS:      "linux",
		Arch:    "amd64",
		NumCPUs: 4,
		Passed:  true,
		Message: "System meets minimum requirements.",
	}

	report := GeneratePreflightReport(res)

	if !strings.Contains(report, "Day One Preflight Checker") {
		t.Errorf("Report missing title")
	}
	if !strings.Contains(report, "blur(20px)") {
		t.Errorf("Report missing glassmorphism styling")
	}
	if !strings.Contains(report, "PASSED") {
		t.Errorf("Report missing PASSED status")
	}
	if !strings.Contains(report, "linux") {
		t.Errorf("Report missing OS")
	}
}
