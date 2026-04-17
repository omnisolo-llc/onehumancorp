package standalone

import (
	"os"
	"os/exec"
	"path/filepath"
	"testing"
)

func TestCleanupTmpFiles(t *testing.T) {
	// Create a temporary directory to act as HOME
	tempHome := t.TempDir()

	// Set up the state dir
	stateDir := filepath.Join(tempHome, ".openclaw")
	err := os.MkdirAll(stateDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create state dir: %v", err)
	}

	// Create a dummy Linear file
	linearFile := filepath.Join(stateDir, "dummy_Linear_file.txt")
	err = os.WriteFile(linearFile, []byte("test content"), 0644)
	if err != nil {
		t.Fatalf("Failed to create Linear file: %v", err)
	}

	// Create a dummy shell script to source the function and execute it
	scriptPath := filepath.Join(tempHome, "test_script.sh")
	scriptContent := `#!/usr/bin/env bash
# Mock functions needed by the script
resolve_script_dir() { echo "/tmp"; }
find_runfiles_root() { echo "/tmp"; }
find_server_bin() { echo "/bin/true"; }
config_string() { echo ""; }
resolve_port() { echo "18789"; }

# Extract the cleanup_tmp_files function
sed -n '/^cleanup_tmp_files() {/,/^}/p' $(find ${RUNFILES_DIR:-.} -name standalone_ohc.sh | head -n 1) > ` + tempHome + `/cleanup_funcs.sh

source ` + tempHome + `/cleanup_funcs.sh

# Run it
STATE_DIR="` + stateDir + `" OHC_STANDALONE=true cleanup_tmp_files
`
	err = os.WriteFile(scriptPath, []byte(scriptContent), 0755)
	if err != nil {
		t.Fatalf("Failed to create test script: %v", err)
	}

	// Execute the script
	cmd := exec.Command("bash", scriptPath)
	out, err := cmd.CombinedOutput()
	if err != nil {
		t.Fatalf("Script failed: %v\nOutput: %s", err, string(out))
	}

	// Check if the Linear file still exists
	if _, err := os.Stat(linearFile); os.IsNotExist(err) {
		t.Fatalf("Linear file was deleted! It should have been preserved.")
	} else if err != nil {
		t.Fatalf("Failed to stat Linear file: %v", err)
	}
}
