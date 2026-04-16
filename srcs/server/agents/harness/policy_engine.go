package harness

import (
	"context"
	"strings"
	"sync"

	"mvdan.cc/sh/v3/syntax"
)

// PolicyEngine is a granular execution policy engine for KAIROS to explicitly approve
// or deny specific commands executed by agents.
type PolicyEngine struct {
	mu          sync.RWMutex
	allowRules  []string
	denyRules   []string
}

// NewPolicyEngine creates a new Execution Policy Engine.
func NewPolicyEngine() *PolicyEngine {
	return &PolicyEngine{
		allowRules: make([]string, 0),
		denyRules:  make([]string, 0),
	}
}

// AddAllowRule adds an explicit rule that allows a command or prefix.
func (pe *PolicyEngine) AddAllowRule(rule string) {
	pe.mu.Lock()
	defer pe.mu.Unlock()
	pe.allowRules = append(pe.allowRules, rule)
}

// AddDenyRule adds an explicit rule that denies a command or prefix.
func (pe *PolicyEngine) AddDenyRule(rule string) {
	pe.mu.Lock()
	defer pe.mu.Unlock()
	pe.denyRules = append(pe.denyRules, rule)
}

// CheckPolicy validates if the given command string is allowed.
// A command is denied if it matches any deny rule.
// A command is allowed if it matches any allow rule.
// It parses the bash script AST to handle compound commands safely.
func (pe *PolicyEngine) CheckPolicy(ctx context.Context, command string) bool {
	pe.mu.RLock()
	defer pe.mu.RUnlock()

	parser := syntax.NewParser()
	file, err := parser.Parse(strings.NewReader(command), "")
	if err != nil {
		// If it's not a valid bash syntax, we might deny it, but it might just be a simple command
		// that failed parsing. To be safe, if we can't parse it, we deny.
		return false
	}

	allowed := true
	syntax.Walk(file, func(node syntax.Node) bool {
		switch x := node.(type) {
		case *syntax.CallExpr:
			if len(x.Args) > 0 {
				cmdName := x.Args[0].Lit()

				// Deny rules take precedence
				for _, rule := range pe.denyRules {
					// We only check the base command name or basic prefix matching here.
					// A more sophisticated implementation might check arguments too.
					if strings.HasPrefix(cmdName, rule) {
						allowed = false
						return false
					}
					// Also check if the rule is trying to match arguments like "rm -rf"
                    // To do this simply, we re-construct the command parts
                    var fullCmdBuilder strings.Builder
                    for i, arg := range x.Args {
                        if i > 0 {
                            fullCmdBuilder.WriteString(" ")
                        }
                        fullCmdBuilder.WriteString(arg.Lit())
                    }
                    if strings.HasPrefix(fullCmdBuilder.String(), rule) {
                        allowed = false
                        return false
                    }
				}

				// Allow rules
				// We don't implement strict allow-only mode by default for backward compatibility
				// But we check if it hits an allow rule
				// If we wanted to make this default-deny, we would set a flag here.
			}
		}
		return true // continue walking
	})

	return allowed
}

// LoadPoliciesFromDB would simulate loading rules from Postgres/SQLite.
func (pe *PolicyEngine) LoadPoliciesFromDB() error {
	pe.AddAllowRule("ls")
	pe.AddAllowRule("echo")
	pe.AddAllowRule("cat")
	pe.AddDenyRule("rm -rf")
	pe.AddDenyRule("sudo")
	pe.AddDenyRule("wget")
	pe.AddDenyRule("curl")
	return nil
}
