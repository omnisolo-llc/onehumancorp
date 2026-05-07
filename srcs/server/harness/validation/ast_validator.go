package validation

import (
	"context"
	"fmt"
	"strings"

	"mvdan.cc/sh/v3/syntax"
	"onehumancorp/srcs/server/telemetry"
)

type Config struct {
	BlockedCommands []string
}

type ASTValidator struct {
	config Config
}

func NewASTValidator(config Config) *ASTValidator {
	return &ASTValidator{config: config}
}

func (v *ASTValidator) Validate(cmd string) error {
	in := strings.NewReader(cmd)
	parser := syntax.NewParser()
	file, err := parser.Parse(in, "")
	if err != nil {
		return fmt.Errorf("failed to parse command: %w", err)
	}

	var validationErr error

	var getWordString func(word *syntax.Word) (string, bool)
	getWordString = func(word *syntax.Word) (string, bool) {
		var result string
		isLiteral := true
		for _, part := range word.Parts {
			switch p := part.(type) {
			case *syntax.Lit:
				result += p.Value
			case *syntax.SglQuoted:
				result += p.Value
			case *syntax.DblQuoted:
				for _, dp := range p.Parts {
					if lit, ok := dp.(*syntax.Lit); ok {
						result += lit.Value
					} else {
						isLiteral = false
					}
				}
			default:
				isLiteral = false
			}
		}
		return result, isLiteral
	}

	syntax.Walk(file, func(node syntax.Node) bool {
		if validationErr != nil {
			return false
		}

		if callExpr, ok := node.(*syntax.CallExpr); ok && len(callExpr.Args) > 0 {
			cmdNameNode := callExpr.Args[0]
			cmdName, isLit := getWordString(cmdNameNode)

			if isLit {
				for _, blocked := range v.config.BlockedCommands {
					if cmdName == blocked {
						telemetry.RecordHarnessViolation(context.Background(), "ast_validation_denied")
						validationErr = fmt.Errorf("blocked command detected: %s", blocked)
						return false
					}
				}
			} else {
				telemetry.RecordHarnessViolation(context.Background(), "ast_validation_denied")
				validationErr = fmt.Errorf("dynamic command execution not allowed")
				return false
			}
		}
		return true
	})

	return validationErr
}
