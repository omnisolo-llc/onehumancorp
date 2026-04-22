package sandbox

import (
	"regexp"
)

type PermissionEvaluator struct {
	disabledCommands []string
	allowedPatterns  []*regexp.Regexp
}

func NewPermissionEvaluator(disabledCommands []string, allowedRegexes []string) *PermissionEvaluator {
	var patterns []*regexp.Regexp
	for _, r := range allowedRegexes {
		if re, err := regexp.Compile(r); err == nil {
			patterns = append(patterns, re)
		}
	}
	return &PermissionEvaluator{
		disabledCommands: disabledCommands,
		allowedPatterns:  patterns,
	}
}

func (p *PermissionEvaluator) IsAllowed(command string) bool {
	for _, disabled := range p.disabledCommands {
		if command == disabled {
			return false
		}
	}
	if len(p.allowedPatterns) > 0 {
		for _, re := range p.allowedPatterns {
			if re.MatchString(command) {
				return true
			}
		}
		return false
	}
	return true
}
