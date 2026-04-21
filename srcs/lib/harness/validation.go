package harness

import (
	"fmt"
	"regexp"
)

var blockedPatterns = []*regexp.Regexp{
	regexp.MustCompile(`<\(`),
	regexp.MustCompile(`>\(`),
	regexp.MustCompile(`\$\[`),
	// Only match equals expansion if it's at the start of a word (not preceded by non-space chars)
	regexp.MustCompile(`(^|\s)=([a-zA-Z0-9_-]+)`),
	regexp.MustCompile(`(?i)\bzmodload\b`),
	regexp.MustCompile(`\.git/`),
}

func ValidateCommand(cmd string) error {
	for _, pattern := range blockedPatterns {
		if pattern.MatchString(cmd) {
			return fmt.Errorf("command violates security policy: matched %s", pattern.String())
		}
	}
	return nil
}
