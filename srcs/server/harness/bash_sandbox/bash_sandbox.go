package bashsandbox

import (
	"context"
	"fmt"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/harness"
)

type SandboxPolicy = harness.Policy
type SandboxResult = harness.Result

// Sandbox provides semantic bash execution and isolation.
type Sandbox struct {
	runner    *harness.BwrapRunner
	validator *harness.ASTValidator
}

// NewSandbox creates a new Sandbox instance with AST validation and bwrap isolation.
func NewSandbox() *Sandbox {
	validator := harness.NewASTValidator()
	return &Sandbox{
		runner:    harness.NewBwrapRunner(validator),
		validator: validator,
	}
}

// SemanticBashResult provides additional semantic analysis of the command result.
type SemanticBashResult struct {
	SandboxResult
	IsSearchOrRead bool
	IsList         bool
}

// isSearchOrRead determines if a command is generally read-only (like cat, grep, ls).
func isSearchOrRead(command string) (bool, bool) {
	readCommands := map[string]bool{
		"cat": true, "head": true, "tail": true, "less": true, "more": true,
		"wc": true, "stat": true, "file": true, "strings": true,
		"jq": true, "awk": true, "cut": true, "sort": true, "uniq": true, "tr": true,
		"find": true, "grep": true, "rg": true, "ag": true, "ack": true, "locate": true, "which": true, "whereis": true,
		"ls": true, "tree": true, "du": true,
	}
	listCommands := map[string]bool{
		"ls": true, "tree": true, "du": true,
	}

	parts := strings.Fields(command)
	if len(parts) == 0 {
		return false, false
	}

	baseCmd := parts[0]
	isRead := readCommands[baseCmd]
	isList := listCommands[baseCmd]

	return isRead, isList
}

// Execute parses the AST, applies semantic policies, and runs the command in a sandbox.
func (s *Sandbox) Execute(ctx context.Context, command string, policy *SandboxPolicy) (SemanticBashResult, error) {
	// First, strictly validate the AST.
	if err := s.validator.Validate(ctx, command); err != nil {
		return SemanticBashResult{}, fmt.Errorf("security violation: %w", err)
	}

	isRead, isList := isSearchOrRead(command)

	// Execute via bwrap.
	res, err := s.runner.ExecuteWithPolicy(ctx, command, policy)
	if err != nil {
		return SemanticBashResult{}, err
	}

	return SemanticBashResult{
		SandboxResult:  res,
		IsSearchOrRead: isRead,
		IsList:         isList,
	}, nil
}
