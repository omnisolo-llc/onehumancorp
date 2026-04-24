package onboarding

import (
	"fmt"
	"runtime"
	"strings"
)

type PreflightResult struct {
	OS      string
	Arch    string
	NumCPUs int
	Passed  bool
	Message string
}

func RunPreflightCheck(isCloud bool) PreflightResult {
	res := PreflightResult{
		OS:      runtime.GOOS,
		Arch:    runtime.GOARCH,
		NumCPUs: runtime.NumCPU(),
		Passed:  true,
		Message: "System meets minimum requirements.",
	}

	if isCloud && res.NumCPUs < 2 {
		res.Passed = false
		res.Message = "Cloud-native mode requires at least 2 CPUs."
	}
	return res
}

func GeneratePreflightReport(res PreflightResult) string {
	status := "PASSED"
	if !res.Passed {
		status = "FAILED"
	}

	var sb strings.Builder
	sb.WriteString("<div style=\"backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px;\">\n")
	sb.WriteString("  <h2>Day One Preflight Checker</h2>\n")
	sb.WriteString(fmt.Sprintf("  <p><strong>OS:</strong> %s</p>\n", res.OS))
	sb.WriteString(fmt.Sprintf("  <p><strong>Arch:</strong> %s</p>\n", res.Arch))
	sb.WriteString(fmt.Sprintf("  <p><strong>CPUs:</strong> %d</p>\n", res.NumCPUs))
	sb.WriteString(fmt.Sprintf("  <p><strong>Status:</strong> %s</p>\n", status))
	sb.WriteString(fmt.Sprintf("  <p><strong>Message:</strong> %s</p>\n", res.Message))
	sb.WriteString("</div>")

	return sb.String()
}
