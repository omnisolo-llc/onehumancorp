package harness

import (
	"context"
	"fmt"
	"sync"

	sitter "github.com/smacker/go-tree-sitter"
	"github.com/smacker/go-tree-sitter/bash"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter          = otel.Meter("ohc_agent_harness")
	violationCount metric.Int64Counter
)

func init() {
	var err error
	violationCount, err = meter.Int64Counter("ohc_sandbox_violation_total",
		metric.WithDescription("Total number of sandbox violations prevented by AST validation or bwrap policies"))
	if err != nil {
		panic(err)
	}
}

// ASTValidator validates bash commands using tree-sitter.
type ASTValidator struct {
	mu     sync.Mutex
	parser *sitter.Parser
}

// NewASTValidator creates a new ASTValidator.
func NewASTValidator() *ASTValidator {
	parser := sitter.NewParser()
	parser.SetLanguage(bash.GetLanguage())
	return &ASTValidator{
		parser: parser,
	}
}

// Validate parses the bash command and checks for unsafe operations.
func (v *ASTValidator) Validate(ctx context.Context, command string) error {
	v.mu.Lock()
	defer v.mu.Unlock()

	tree := v.parser.Parse(nil, []byte(command))
	if tree == nil {
		return fmt.Errorf("failed to parse command")
	}

	if err := v.walkAndValidate(tree.RootNode(), command); err != nil {
		violationCount.Add(ctx, 1)
		return err
	}

	return nil
}

func (v *ASTValidator) walkAndValidate(node *sitter.Node, command string) error {
	nodeType := node.Type()

	if nodeType == "command_substitution" || nodeType == "process_substitution" || nodeType == "subshell" {
		return fmt.Errorf("subshells and command substitutions are not allowed")
	}

	if nodeType == "file_redirect" || nodeType == "redirected_statement" || nodeType == "heredoc_redirect" || nodeType == "herestring_redirect" {
		return fmt.Errorf("redirections are not allowed")
	}

	if nodeType == "variable_name" {
		varName := command[node.StartByte():node.EndByte()]
		if varName == "IFS" {
			return fmt.Errorf("IFS injection is not allowed")
		}
	}

	if nodeType == "command_name" || nodeType == "command" {
		cmdName := command[node.StartByte():node.EndByte()]
		if cmdName == "zmodload" {
			return fmt.Errorf("zsh dangerous command zmodload is blocked")
		}
		if cmdName == "sudo" {
			return fmt.Errorf("sudo is not allowed")
		}
	}

	for i := 0; i < int(node.ChildCount()); i++ {
		child := node.Child(i)
		if err := v.walkAndValidate(child, command); err != nil {
			return err
		}
	}

	return nil
}
