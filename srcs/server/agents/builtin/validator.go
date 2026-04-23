package builtin

import (
	"context"
	"fmt"
	"sync"
	"strings"

	sitter "github.com/smacker/go-tree-sitter"
	"github.com/smacker/go-tree-sitter/bash"
)

// CommandValidator defines the interface for validating bash commands
type CommandValidator interface {
	Validate(ctx context.Context, command string) error
}

// ASTCommandValidator uses tree-sitter to validate bash commands
type ASTCommandValidator struct {
	mu     sync.Mutex
	parser *sitter.Parser
}

// NewASTCommandValidator creates a new validator instance
func NewASTCommandValidator() *ASTCommandValidator {
	parser := sitter.NewParser()
	parser.SetLanguage(bash.GetLanguage())
	return &ASTCommandValidator{
		parser: parser,
	}
}

// Validate checks a command against the blacklist
func (v *ASTCommandValidator) Validate(ctx context.Context, command string) error {
	v.mu.Lock()
	defer v.mu.Unlock()

	tree := v.parser.Parse(nil, []byte(command))
	if tree == nil {
		return fmt.Errorf("failed to parse command")
	}

	return v.walkAndValidate(tree.RootNode(), command)
}

func (v *ASTCommandValidator) walkAndValidate(node *sitter.Node, command string) error {
	nodeType := node.Type()

	if nodeType == "command_substitution" || nodeType == "process_substitution" || nodeType == "subshell" {
		return fmt.Errorf("subshells and command substitutions are not allowed")
	}

	if nodeType == "command_name" || nodeType == "command" {
		cmdName := command[node.StartByte():node.EndByte()]
		if cmdName == "zmodload" || cmdName == "emulate" {
			return fmt.Errorf("zsh dangerous command %s is blocked", cmdName)
		}
	}

    if nodeType == "word" {
        word := command[node.StartByte():node.EndByte()]
        if strings.Contains(word, "sip.db") {
            return fmt.Errorf("access to sip.db is blocked")
        }
        if strings.HasPrefix(word, "=") {
            return fmt.Errorf("zsh process substitution is blocked")
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
