package sandbox

import (
	"os"
	"regexp"
	"strings"
)

type PermissionEvaluator struct {
	disabledCommands []string
	disabledPatterns []*regexp.Regexp
}

func NewPermissionEvaluator() *PermissionEvaluator {
	pe := &PermissionEvaluator{
		disabledCommands: []string{"rm -rf /", "mkfs"},
		disabledPatterns: []*regexp.Regexp{
			regexp.MustCompile(`(?i)\bsudo\b`),
			regexp.MustCompile(`(?i)\bchown\b`),
		},
	}

	envDisabledCmds := os.Getenv("OOS_SANDBOX_DISABLED_COMMANDS")
	if envDisabledCmds != "" {
		cmds := strings.Split(envDisabledCmds, ",")
		for _, cmd := range cmds {
			cmd = strings.TrimSpace(cmd)
			if cmd != "" {
				pe.disabledCommands = append(pe.disabledCommands, cmd)
			}
		}
	}

	return pe
}

func (pe *PermissionEvaluator) UpdatePolicy(policy SandboxPolicy) {
	pe.disabledCommands = append(pe.disabledCommands, policy.DisabledCommands...)
	for _, pattern := range policy.DisabledPatterns {
		if re, err := regexp.Compile(pattern); err == nil {
			pe.disabledPatterns = append(pe.disabledPatterns, re)
		}
	}
}

func (pe *PermissionEvaluator) Evaluate(cmd string) bool {
	for _, disabled := range pe.disabledCommands {
		if strings.Contains(cmd, disabled) {
			return false
		}
	}

	for _, pattern := range pe.disabledPatterns {
		if pattern.MatchString(cmd) {
			return false
		}
	}

	return true
}
