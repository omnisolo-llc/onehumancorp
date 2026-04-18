package telemetry_test

import (
	"bufio"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestGlobalPIIRedactionLinter(t *testing.T) {
	serverPath := "srcs/server"

	if _, err := os.Stat(serverPath); os.IsNotExist(err) {
	    t.Skipf("Skipping test due to missing directory %v", err)
	}

	err := filepath.Walk(serverPath, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() || !strings.HasSuffix(path, ".go") || strings.HasSuffix(path, "_test.go") {
			return nil
		}

		if !strings.Contains(path, "telemetry") && !strings.Contains(path, "log") && !strings.Contains(path, "bridge") {
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

			if strings.Contains(line, "json.Marshal(") && !strings.Contains(line, "json.Marshal(redactedMap)") && !strings.Contains(line, "json.Marshal(RedactInterfacePII(") && !strings.Contains(line, "json.Marshal(telemetry.RedactInterfacePII(") && !strings.Contains(line, "json.Marshal(RedactPII(") {
				if strings.Contains(line, "if redactedBytes, err := json.Marshal(redacted); err == nil {") ||
					strings.Contains(line, "if redactedBytes, err := json.Marshal(parsedIface); err == nil {") ||
					strings.Contains(line, "payload, err = json.Marshal(decoded)") ||
					strings.Contains(line, "payload, err = json.Marshal(redacted)") ||
					strings.Contains(line, "payload, err := json.Marshal(redacted)") {
					continue
				}

				if strings.Contains(line, "json.Marshal(payload)") || strings.Contains(line, "json.Marshal(raw)") || strings.Contains(line, "json.Marshal(logEntry)") {
					t.Errorf("PII Leak Risk in %s:%d - json.Marshal called without RedactInterfacePII: %s", path, lineNum, strings.TrimSpace(line))
				}
			}
		}
		return nil
	})

	if err != nil {
		t.Fatalf("Failed to walk directory: %v", err)
	}
}
