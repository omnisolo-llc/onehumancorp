package telemetry

import (
	"bufio"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestNoPIILoggingStatements(t *testing.T) {
	searchDirs := []string{"../"}

	if workspaceDir := os.Getenv("BUILD_WORKSPACE_DIRECTORY"); workspaceDir != "" {
		searchDirs = append(searchDirs, filepath.Join(workspaceDir, "srcs/server"))
	}

	violations := []string{}
	checkedFiles := 0

	sensitiveKeys := []string{
		"tenant_id", "organization_id", "org_id", "session_data", "session_id",
		"payload", "email", "password", "pii", "api_key", "secret_key",
	}

	logFuncs := []string{
		"log.Printf", "log.Println", "log.Print", "log.Fatalf", "log.Fatalln",
		"logrus.Info", "logrus.Warn", "logrus.Error", "logrus.Debug", "logrus.Trace",
		"zap.L().Info", "zap.L().Warn", "zap.L().Error", "zap.L().Debug",
	}

	for _, dir := range searchDirs {
		err := filepath.Walk(dir, func(path string, info os.FileInfo, err error) error {
			if err != nil {
				return nil
			}
			if !info.IsDir() && strings.HasSuffix(info.Name(), ".go") && !strings.Contains(path, "external") {
				checkedFiles++
				file, err := os.Open(path)
				if err != nil {
					return nil
				}
				defer file.Close()

				scanner := bufio.NewScanner(file)
				lineNum := 1
				for scanner.Scan() {
					line := scanner.Text()
					lowerLine := strings.ToLower(line)

					hasLog := false
					for _, lf := range logFuncs {
						if strings.Contains(lowerLine, strings.ToLower(lf)) {
							hasLog = true
							break
						}
					}

					if hasLog {
						for _, sk := range sensitiveKeys {
							if strings.Contains(lowerLine, sk) {
								violations = append(violations, fmt.Sprintf("%s:%d: %s", path, lineNum, strings.TrimSpace(line)))
								break
							}
						}
					}
					lineNum++
				}
			}
			return nil
		})
		if err != nil {
			t.Logf("Error walking directory %s: %v", dir, err)
		}
	}

	if checkedFiles == 0 {
		t.Logf("No .go files found to test.")
	}

	if len(violations) > 0 {
		t.Fatalf("Found PII logging violations in the following lines:\n%v", strings.Join(violations, "\n"))
	}
}
