package builtin

import (
	"context"
	"fmt"
	"strings"

	sitter "github.com/smacker/go-tree-sitter"
	"github.com/smacker/go-tree-sitter/bash"
)

type CommandValidator interface {
	Validate(ctx context.Context, command string) error
}

type ASTCommandValidator struct{}

func NewASTCommandValidator() *ASTCommandValidator {
	return &ASTCommandValidator{}
}

func (v *ASTCommandValidator) Validate(ctx context.Context, command string) error {
	parser := sitter.NewParser()
	defer parser.Close()
	parser.SetLanguage(bash.GetLanguage())

	src := []byte(command)
	tree, err := parser.ParseCtx(ctx, nil, src)
	if tree != nil {
		defer tree.Close()
	}
	if err != nil {
		return fmt.Errorf("failed to parse command: %v", err)
	}

	var validationErr error

	var walk func(*sitter.Node)
	walk = func(n *sitter.Node) {
		if validationErr != nil {
			return
		}

		nodeType := n.Type()

		if nodeType == "command_name" {
			cmdName := n.Content(src)
			if cmdName == "zmodload" || cmdName == "emulate" {
				validationErr = fmt.Errorf("blocked command: %s", cmdName)
				return
			}
		}

		if nodeType == "process_substitution" {
			validationErr = fmt.Errorf("blocked process substitution")
			return
		}

		// For zsh expansion which might be misparsed
		if nodeType == "command" {
			for i := 0; i < int(n.ChildCount()); i++ {
				child := n.Child(i)
				if child.Type() == "ERROR" {
					if child.Content(src) == "=" {
						if i+1 < int(n.ChildCount()) {
							nextChild := n.Child(i+1)
							if nextChild.Type() == "subshell" {
								validationErr = fmt.Errorf("blocked process substitution")
								return
							}
						}
					}
				}
			}
		}

		if nodeType == "word" || nodeType == "string" || nodeType == "raw_string" {
			content := n.Content(src)
			if strings.Contains(content, "sip.db") {
				validationErr = fmt.Errorf("attempted access to OHC internal sip.db state files")
				return
			}
		}

		for i := 0; i < int(n.ChildCount()); i++ {
			walk(n.Child(i))
		}
	}

	walk(tree.RootNode())

	return validationErr
}
