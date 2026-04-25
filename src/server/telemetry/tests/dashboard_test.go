package tests

import (
	"encoding/json"
	"os"
	"strings"
	"testing"

	"github.com/bazelbuild/rules_go/go/runfiles"
)

func TestHarnessEfficiencyDashboardPanels(t *testing.T) {
	// Use rules_go runfiles to reliably find the JSON dashboard path
	dashboardPath, err := runfiles.Rlocation("_main/deploy/helm/ohc/dashboards/harness_efficiency.json")
	if err != nil {
		t.Fatalf("Failed to locate runfile harness_efficiency.json: %v", err)
	}

	data, err := os.ReadFile(dashboardPath)
	if err != nil {
		t.Fatalf("Failed to read dashboard file: %v", err)
	}

	var dashboard map[string]interface{}
	if err := json.Unmarshal(data, &dashboard); err != nil {
		t.Fatalf("Failed to parse dashboard JSON: %v", err)
	}

	panels, ok := dashboard["panels"].([]interface{})
	if !ok {
		t.Fatalf("Dashboard does not have 'panels' array")
	}

	expectedPanels := map[string]bool{
		"Harness Execution Latency (P95)":       false,
		"Sync Conflicts Resolved (Rate)":        false,
		"Context Bytes Routed (Rate by Tenant)": false,
	}

	for _, p := range panels {
		panel, ok := p.(map[string]interface{})
		if !ok {
			continue
		}

		title, ok := panel["title"].(string)
		if !ok {
			continue
		}

		for expectedTitle := range expectedPanels {
			if strings.Contains(title, expectedTitle) || title == expectedTitle {
				expectedPanels[expectedTitle] = true
			}
		}
	}

	for title, found := range expectedPanels {
		if !found {
			t.Errorf("Expected panel not found in dashboard: %s", title)
		}
	}
}
