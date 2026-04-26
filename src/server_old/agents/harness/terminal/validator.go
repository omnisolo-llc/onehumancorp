package terminal

import (
	"errors"
	"regexp"
	"strings"
)

var (
	ErrDangerousZSHBuiltin = errors.New("dangerous ZSH builtin detected")
	ErrProcessSubstitution = errors.New("process substitution detected")
	ErrUnsafeFlag          = errors.New("unsafe flag detected for read-only command")
)

type CommandValidator interface {
	Validate(cmd string) error
	ValidateReadOnly(cmd string) error
}

// Allowed flags for specific tools in read-only mode
var SafeFlags = map[string]map[string]bool{
	"grep": {"-i": true, "-r": true, "-n": true, "-v": true, "-E": true, "-l": true, "--color": true},
	"fd":   {"-H": true, "-I": true, "-t": true, "-e": true, "-E": true, "-u": true}, // -x is excluded as it's exec
	"find": {"-name": true, "-type": true, "-maxdepth": true, "-mindepth": true, "-mtime": true, "-size": true, "-print": true, "-print0": true}, // -exec is excluded
	"cat":  {"-n": true, "-b": true, "-s": true, "-v": true, "-E": true, "-T": true, "-A": true},
	"ls":   {"-l": true, "-a": true, "-h": true, "-t": true, "-r": true, "-S": true, "-R": true, "--color": true},
}

type DefaultCommandValidator struct{}

func NewDefaultCommandValidator() *DefaultCommandValidator {
	return &DefaultCommandValidator{}
}

func (v *DefaultCommandValidator) Validate(cmd string) error {
	// Block ZSH dangerous builtins at the start of the command or after a semicolon/pipe
	zshBuiltinRegex := regexp.MustCompile(`(?:^|[;|]\s*)\b(zmodload|emulate|zpty)\b`)
	if zshBuiltinRegex.MatchString(cmd) {
		return ErrDangerousZSHBuiltin
	}

	// Block process substitution
	processSubstRegex := regexp.MustCompile(`(=\(.*?\)|<\(.*?\)|>\(.*?\))`)
	if processSubstRegex.MatchString(cmd) {
		return ErrProcessSubstitution
	}

	return nil
}

func (v *DefaultCommandValidator) ValidateReadOnly(cmd string) error {
	if err := v.Validate(cmd); err != nil {
		return err
	}

	parts := strings.Fields(cmd)
	if len(parts) > 0 {
		tool := parts[0]
		if allowedFlags, ok := SafeFlags[tool]; ok {
			for _, part := range parts[1:] {
				if strings.HasPrefix(part, "-") {
					// Extract flag name (ignore value if using =)
					flagName := part
					if idx := strings.Index(part, "="); idx != -1 {
						flagName = part[:idx]
					}

					// Simple hack to support bundled short flags like -la
					if len(flagName) > 2 && !strings.HasPrefix(flagName, "--") {
					    for i := 1; i < len(flagName); i++ {
					        shortFlag := "-" + string(flagName[i])
					        if !allowedFlags[shortFlag] {
					            return ErrUnsafeFlag
					        }
					    }
					} else if !allowedFlags[flagName] {
						return ErrUnsafeFlag
					}
				}
			}
		}
	}

	return nil
}
