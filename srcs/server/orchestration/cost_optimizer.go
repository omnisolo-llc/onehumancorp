package orchestration

import (
	"regexp"
	"strings"
)

var (
	// Matches multiple empty lines and reduces them to a single empty line
	multipleNewlinesRegex = regexp.MustCompile(`\n{3,}`)
	// Matches markdown comments
	markdownCommentRegex = regexp.MustCompile(`(?s)<!--.*?-->`)
)

// OptimizePromptForCost reduces the LLM token footprint of a prompt string
// by stripping hidden markdown comments and compressing excessive newlines,
// while strictly preserving all horizontal whitespace (spaces/tabs) to maintain
// structural integrity for code blocks (Python, YAML, etc.).
// This directly translates to lower API token costs in Cloud Mode.
func OptimizePromptForCost(prompt string) string {
	// 1. Remove markdown comments which are never visible to the user but consume LLM tokens.
	optimized := markdownCommentRegex.ReplaceAllString(prompt, "")

	// 2. Collapse 3+ newlines into 2 newlines (a single empty line).
	optimized = multipleNewlinesRegex.ReplaceAllString(optimized, "\n\n")

	// 3. Trim leading and trailing whitespace.
	optimized = strings.TrimSpace(optimized)

	return optimized
}
