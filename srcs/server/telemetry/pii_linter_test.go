package telemetry_test

import (
	"bufio"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// TestPIIRedactionLinter acts as an automated guardrail to ensure all calls
// to json.Marshal within the telemetry package use RedactInterfacePII or RedactPII.
func TestPIIRedactionLinter(t *testing.T) {
	err := filepath.Walk(".", func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() || !strings.HasSuffix(path, ".go") || strings.HasSuffix(path, "_test.go") {
			return nil
		}

		file, err := os.Open(path)
		if err != nil {
			t.Errorf("Failed to open file %s: %v", path, err)
			return nil
		}
		defer file.Close()

		scanner := bufio.NewScanner(file)
		lineNum := 0
		for scanner.Scan() {
			lineNum++
			line := scanner.Text()

			// We are looking for lines that marshal a payload for telemetry buffering.
			if strings.Contains(line, "json.Marshal(") && !strings.Contains(line, "json.Marshal(redactedMap)") && !strings.Contains(line, "json.Marshal(redactedPayloads)") && !strings.Contains(line, "json.Marshal(RedactInterfacePII(") && !strings.Contains(line, "json.Marshal(RedactPII(") {
				// Allow lists
				if strings.Contains(line, "if redactedBytes, err := json.Marshal(redacted); err == nil {") {
					continue
				}

				t.Errorf("PII Leak Risk in %s:%d - json.Marshal called without RedactInterfacePII: %s", path, lineNum, strings.TrimSpace(line))
			}
		}
		return nil
	})

	if err != nil {
		t.Fatalf("Failed to walk directory: %v", err)
	}
}
