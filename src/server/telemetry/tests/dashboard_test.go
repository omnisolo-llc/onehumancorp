package tests

import (
	"encoding/json"
	"io/ioutil"
	"path/filepath"
	"testing"
	"os"
	"strings"
)

func TestHarnessEfficiencyDashboardPanels(t *testing.T) {
	// Adjust path for bazel test execution
	// In bazel tests, the workspace root is the runfiles directory
	runfilesDir := os.Getenv("TEST_SRCDIR")
	workspaceName := os.Getenv("TEST_WORKSPACE")

	dashboardPath := ""
	if runfilesDir != "" && workspaceName != "" {
		dashboardPath = filepath.Join(runfilesDir, workspaceName, "deploy", "helm", "ohc", "dashboards", "harness_efficiency.json")
	} else {
		workspaceRoot := os.Getenv("BUILD_WORKSPACE_DIRECTORY")
		if workspaceRoot == "" {
			workspaceRoot = "../../../.."
		}
		dashboardPath = filepath.Join(workspaceRoot, "deploy", "helm", "ohc", "dashboards", "harness_efficiency.json")
	}

	data, err := ioutil.ReadFile(dashboardPath)
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
		"Harness Execution Latency (P95)": false,
		"Sync Conflicts Resolved (Rate)": false,
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
