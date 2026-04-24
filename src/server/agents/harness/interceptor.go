package harness

import (
	"context"
	"fmt"
	"strings"

	sitter "github.com/smacker/go-tree-sitter"
	"github.com/smacker/go-tree-sitter/bash"
	"github.com/onehumancorp/mono/src/server/telemetry"
)

type ASTParser struct {}

func NewASTParser() *ASTParser {
	return &ASTParser{}
}

func (p *ASTParser) parseForSecurity(ctx context.Context, command string) error {
	parser := sitter.NewParser()
	defer parser.Close()
	parser.SetLanguage(bash.GetLanguage())

	tree, err := parser.ParseCtx(ctx, nil, []byte(command))
	if tree != nil {
		defer tree.Close()
	}
	if err != nil {
		return fmt.Errorf("failed to parse command: %w", err)
	}
	if tree == nil {
		return fmt.Errorf("failed to parse command")
	}

	return p.walkAndValidate(tree.RootNode(), command)
}

func (p *ASTParser) walkAndValidate(node *sitter.Node, command string) error {
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
		if err := p.walkAndValidate(child, command); err != nil {
			return err
		}
	}

	return nil
}

type PermissionInterceptor struct {
	next   IsolationHarness
	parser *ASTParser
}

func NewPermissionInterceptor(next IsolationHarness) *PermissionInterceptor {
	return &PermissionInterceptor{
		next:   next,
		parser: NewASTParser(),
	}
}

func (i *PermissionInterceptor) Execute(ctx context.Context, execCtx ExecutionContext) ([]byte, error) {
	// Reconstruct the actual bash command payload to ensure tree-sitter parses nested scripts properly
	// If it's a "bash -c 'something'", we must parse the 'something'
	var cmdToParse string
	if len(execCtx.Command) >= 3 && (strings.HasSuffix(execCtx.Command[0], "bash") || strings.HasSuffix(execCtx.Command[0], "sh")) && execCtx.Command[1] == "-c" {
		cmdToParse = execCtx.Command[2]
	} else {
		cmdToParse = strings.Join(execCtx.Command, " ")
	}

	if err := i.parser.parseForSecurity(ctx, cmdToParse); err != nil {
		telemetry.RecordBubblewrapViolation(ctx)
		return nil, fmt.Errorf("security violation: %w", err)
	}

	telemetry.RecordBubblewrapSpawn(ctx) // Prometheus tracking for all bash executions

	return i.next.Execute(ctx, execCtx)
}
