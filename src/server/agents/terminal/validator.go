package terminal

import (
	"context"
	"fmt"
	"strings"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/trace"
	"mvdan.cc/sh/v3/syntax"
)

var tracer = otel.Tracer("terminal")

// ValidatorConfig holds configuration for the ASTValidator.
type ValidatorConfig struct {
	BlockedCommands []string
	DestructiveCommands []string
}

// DefaultValidatorConfig provides a safe default configuration.
func DefaultValidatorConfig() ValidatorConfig {
	return ValidatorConfig{
		BlockedCommands: []string{"zmodload", "nc", "ncat", "netcat", "wget", "curl"},
		DestructiveCommands: []string{"rm", "mkfs", "dd", "fdisk", "shred"},
	}
}

// ASTValidator checks shell commands against safety rules.
type ASTValidator struct {
	config ValidatorConfig
	parser *syntax.Parser
}

// NewASTValidator creates a new ASTValidator.
func NewASTValidator(config ValidatorConfig) *ASTValidator {
	return &ASTValidator{
		config: config,
		parser: syntax.NewParser(syntax.KeepComments(false)),
	}
}

// Validate checks if the command is safe to run.
func (v *ASTValidator) Validate(ctx context.Context, command string) error {
	ctx, span := tracer.Start(ctx, "ValidateCommand", trace.WithAttributes(attribute.String("command", command)))
	defer span.End()

	file, err := v.parser.Parse(strings.NewReader(command), "")
	if err != nil {
		span.RecordError(err)
		return fmt.Errorf("failed to parse command: %w", err)
	}

	var validationErr error
	syntax.Walk(file, func(node syntax.Node) bool {
		if validationErr != nil {
			return false
		}

		callExpr, ok := node.(*syntax.CallExpr)
		if !ok {
			return true
		}

		if len(callExpr.Args) > 0 {
			var cmdParts []string
			for _, part := range callExpr.Args[0].Parts {
				switch p := part.(type) {
				case *syntax.Lit:
					cmdParts = append(cmdParts, p.Value)
				case *syntax.SglQuoted:
					cmdParts = append(cmdParts, p.Value)
				case *syntax.DblQuoted:
					var dblParts []string
					for _, dp := range p.Parts {
						if dplit, ok := dp.(*syntax.Lit); ok {
							dblParts = append(dblParts, dplit.Value)
						}
					}
					cmdParts = append(cmdParts, strings.Join(dblParts, ""))
				}
			}
			cmdName := strings.Join(cmdParts, "")

			// Fallback
			if cmdName == "" && len(callExpr.Args[0].Parts) == 1 {
				if lit, ok := callExpr.Args[0].Parts[0].(*syntax.Lit); ok {
					cmdName = lit.Value
				}
			}

			cmdName = strings.ReplaceAll(cmdName, "\\", "")
			cmdName = strings.ReplaceAll(cmdName, "\"", "")

			// Check blocked commands
			for _, blocked := range v.config.BlockedCommands {
				if cmdName == blocked {
					validationErr = fmt.Errorf("blocked command detected: %s", cmdName)
					span.RecordError(validationErr)
					return false
				}
			}

			// Check destructive commands with specific args (like rm -rf)
			if cmdName == "rm" {
				hasRecursive := false
				hasForce := false
				for i := 1; i < len(callExpr.Args); i++ {
					var argParts []string
					for _, part := range callExpr.Args[i].Parts {
						switch p := part.(type) {
						case *syntax.Lit:
							argParts = append(argParts, p.Value)
						case *syntax.SglQuoted:
							argParts = append(argParts, p.Value)
						case *syntax.DblQuoted:
							var dblParts []string
							for _, dp := range p.Parts {
								if dplit, ok := dp.(*syntax.Lit); ok {
									dblParts = append(dblParts, dplit.Value)
								}
							}
							argParts = append(argParts, strings.Join(dblParts, ""))
						}
					}
					argStr := strings.Join(argParts, "")
					if argStr == "" && len(callExpr.Args[i].Parts) == 1 {
						if lit, ok := callExpr.Args[i].Parts[0].(*syntax.Lit); ok {
							argStr = lit.Value
						}
					}
					argStr = strings.ReplaceAll(argStr, "\\", "")
					argStr = strings.ReplaceAll(argStr, "\"", "")
					argStr = strings.ReplaceAll(argStr, "'", "")

					if argStr == "-r" || argStr == "-R" {
						hasRecursive = true
					} else if argStr == "-f" {
						hasForce = true
					} else if strings.HasPrefix(argStr, "-") {
						if strings.Contains(argStr, "r") || strings.Contains(argStr, "R") {
							hasRecursive = true
						}
						if strings.Contains(argStr, "f") {
							hasForce = true
						}
					}
				}

				if hasRecursive && hasForce {
					validationErr = fmt.Errorf("destructive command detected: rm -rf")
					span.RecordError(validationErr)
					return false
				}
			}
		}

		return true
	})

	return validationErr
}
