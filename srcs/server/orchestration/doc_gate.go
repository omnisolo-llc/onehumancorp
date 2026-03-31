package orchestration

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"regexp"
)

// CheckDocumentationGate enforces the Documentation Mandate.
// It scans the task description for a feature mention (e.g., "[Feature: identity-security]").
// If found, it checks if the design-doc.md, cuj.md, and test-plan.md exist in docs/features/<feature>.
func CheckDocumentationGate(instruction string) error {
	re := regexp.MustCompile(`\[Feature:\s*([a-zA-Z0-9-]+)\]`)
	matches := re.FindStringSubmatch(instruction)
	if len(matches) < 2 {
		// If no feature is explicitly tagged, we can't enforce the gate at the directory level.
		// To strictly enforce the mandate, we could return an error here, but we will assume
		// the gate applies to feature-tagged tasks. Let's enforce that if it looks like a feature epic.
		return nil
	}

	featureName := strings.ToLower(matches[1])
	basePath := filepath.Join("docs", "features", featureName)

	requiredDocs := []string{"design-doc.md", "cuj.md", "test-plan.md"}
	for _, doc := range requiredDocs {
		path := filepath.Join(basePath, doc)
		info, err := os.Stat(path)
		if os.IsNotExist(err) || info.Size() < 50 {
			return fmt.Errorf("Documentation Gate Failed: missing or insufficient %s for feature %s", doc, featureName)
		}
	}
	return nil
}
