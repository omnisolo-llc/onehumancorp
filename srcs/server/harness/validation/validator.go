package validation

import (
	"context"
	"fmt"
	"strings"

	sitter "github.com/smacker/go-tree-sitter"
	"github.com/smacker/go-tree-sitter/bash"
)

type SandboxViolationStore interface {
	RecordViolation(ctx context.Context, cmd string, errDetails string)
}

type BashASTValidator struct {
	store SandboxViolationStore
}

func NewBashASTValidator(store SandboxViolationStore) *BashASTValidator {
	return &BashASTValidator{
		store: store,
	}
}

// blockedCommands is a set of commands/builtins that are inherently dangerous or allow sandbox escapes.
var blockedCommands = map[string]bool{
	"eval":     true,
	"exec":     true,
	"alias":    true,
	"unalias":  true,
	"zmodload": true,
	"zparseopts": true,
	"sudo":     true,
	"chown":    true,
	"chmod":    true,
}

func (v *BashASTValidator) Validate(ctx context.Context, command string) error {
	parser := sitter.NewParser()
	defer parser.Close()
	parser.SetLanguage(bash.GetLanguage())

	src := []byte(command)
	tree, err := parser.ParseCtx(ctx, nil, src)
	if tree != nil {
		defer tree.Close()
	}
	if err != nil {
		// If we can't parse it, fail safe or ignore? Usually tree-sitter parses everything, it just produces ERROR nodes.
		return fmt.Errorf("failed to parse command: %v", err)
	}

	var validationErr error

	var walk func(*sitter.Node)
	walk = func(n *sitter.Node) {
		if validationErr != nil {
			return
		}

		nodeType := n.Type()

		// Block dangerous command names
		if nodeType == "command_name" {
			cmdName := n.Content(src)
			if blockedCommands[cmdName] {
				validationErr = fmt.Errorf("blocked command: %s", cmdName)
				return
			}
		}

		// Block specific arguments to commands, e.g., rm -rf /
		if nodeType == "command" {
			var cmdName string
			args := []string{}

			for i := 0; i < int(n.ChildCount()); i++ {
				child := n.Child(i)
				if child.Type() == "command_name" {
					cmdName = child.Content(src)
				} else if child.Type() == "word" || child.Type() == "string" || child.Type() == "raw_string" {
					args = append(args, child.Content(src))
				}
			}

			if cmdName == "rm" {
				hasRecursive := false
				hasForce := false
				hasRoot := false

				for _, arg := range args {
					if strings.HasPrefix(arg, "-") && (strings.Contains(arg, "r") || strings.Contains(arg, "R")) {
						hasRecursive = true
					}
					if strings.HasPrefix(arg, "-") && strings.Contains(arg, "f") {
						hasForce = true
					}
					if arg == "/" || arg == "/*" {
						hasRoot = true
					}
				}

				if hasRecursive && hasForce && hasRoot {
					validationErr = fmt.Errorf("blocked dangerous command: rm -rf /")
					return
				}
			}
		}

		// Block process substitution
		if nodeType == "process_substitution" {
			validationErr = fmt.Errorf("blocked process substitution")
			return
		}

		// Block network redirections
		if nodeType == "file_redirect" {
			for i := 0; i < int(n.ChildCount()); i++ {
				child := n.Child(i)
				if child.Type() == "word" {
					content := child.Content(src)
					if strings.HasPrefix(content, "/dev/tcp") || strings.HasPrefix(content, "/dev/udp") {
						validationErr = fmt.Errorf("blocked network redirection to %s", content)
						return
					}
				}
			}
		}

		for i := 0; i < int(n.ChildCount()); i++ {
			walk(n.Child(i))
		}
	}

	walk(tree.RootNode())

	if validationErr != nil {
		if v.store != nil {
			v.store.RecordViolation(ctx, command, validationErr.Error())
		}
		return validationErr
	}

	return nil
}
