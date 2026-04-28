package terminal

import (
	"errors"

	"strings"
	"path/filepath"
)

type CommandValidator interface {
	Validate(command string) error
}

type TokenValidator struct{}

func NewTokenValidator() *TokenValidator {
	return &TokenValidator{}
}

func (v *TokenValidator) Validate(command string) error {
	// Simple shell tokenizer that respects quotes
	var tokens []string
	var currentToken strings.Builder
	inQuotes := false
	var quoteChar rune

	for _, c := range command {
		if inQuotes {
			if c == quoteChar {
				inQuotes = false
			} else {
				currentToken.WriteRune(c)
			}
		} else {
			if c == '"' || c == '\'' {
				inQuotes = true
				quoteChar = c
			} else if c == ' ' || c == '\t' || c == '\n' {
				if currentToken.Len() > 0 {
					tokens = append(tokens, currentToken.String())
					currentToken.Reset()
				}
			} else {
				currentToken.WriteRune(c)
			}
		}
	}
	if currentToken.Len() > 0 {
		tokens = append(tokens, currentToken.String())
	}

	if len(tokens) == 0 {
		return nil
	}

	binaryToken := tokens[0]
	binaryIdx := 0
	for i, token := range tokens {
		if strings.Contains(token, "=") && !strings.HasPrefix(token, "-") {
			continue
		}
		if token == "env" || token == "bash" || token == "sh" {
			if token == "bash" || token == "sh" {
				if i+1 < len(tokens) && tokens[i+1] == "-c" {
					if i+2 < len(tokens) {
						return v.Validate(tokens[i+2])
					}
				}
			}
			continue
		}
		binaryToken = token
		binaryIdx = i
		break
	}

	binary := filepath.Base(binaryToken)

	if binary == "sudo" {
		return errors.New("sudo is not allowed")
	}

	if binary == "zmodload" || binary == "emulate" || binary == "zpty" || strings.Contains(command, "zmodload") || strings.Contains(command, "emulate") || strings.Contains(command, "zpty") {
		return errors.New("dangerous zsh builtins are not allowed")
	}

	if strings.Contains(command, ">$") || strings.Contains(command, "<$") || strings.Contains(command, "`") || strings.Contains(command, "$(") {
		return errors.New("subshells and redirections are not allowed")
	}

	if strings.Contains(command, "IFS") {
		return errors.New("IFS injection is not allowed")
	}

	// Process substitution check avoiding array assignments
	parts := strings.Split(command, "(")
	for i := 0; i < len(parts)-1; i++ {
		part := strings.TrimRight(parts[i], " \t\n")
		if strings.HasSuffix(part, "=") || strings.HasSuffix(part, "<") || strings.HasSuffix(part, ">") {
			if strings.HasSuffix(part, "=") && !strings.HasSuffix(part, " =") && len(part) > 1 {
				runes := []rune(part)
				prevChar := runes[len(runes)-2]
				if (prevChar >= 'a' && prevChar <= 'z') || (prevChar >= 'A' && prevChar <= 'Z') || (prevChar >= '0' && prevChar <= '9') || prevChar == '_' {
					continue
				}
			}
			return errors.New("process substitution is not allowed")
		}
	}

	allowlists := map[string][]string{
		"grep": {"-i", "-v", "-E", "-n", "-r", "-l"},
		"fd":   {"-t", "-e", "-H", "-I"},
		"find": {"-name", "-type", "-maxdepth", "-mindepth"},
		"ls":   {"-l", "-a", "-h", "-t", "-r", "-1", "-F"},
		"cat":  {"-n", "-v", "-E", "-T"},
	}

	if allowedFlags, ok := allowlists[binary]; ok {
		for i := binaryIdx + 1; i < len(tokens); i++ {
			token := tokens[i]
			if strings.HasPrefix(token, "-") && token != "-" && token != "--" {
				isAllowed := false

				combinable := binary == "ls" || binary == "cat" || binary == "grep" || binary == "fd"

				if strings.HasPrefix(token, "--") {
					for _, flag := range allowedFlags {
						if token == flag || token == "-"+flag {
							isAllowed = true
							break
						}
					}
				} else if combinable {
					allowedChars := make(map[rune]bool)
					for _, flag := range allowedFlags {
						if len(flag) == 2 && strings.HasPrefix(flag, "-") {
							allowedChars[rune(flag[1])] = true
						}
					}

					allCharsAllowed := true
					for _, c := range token[1:] {
						if !allowedChars[c] {
							allCharsAllowed = false
							break
						}
					}

					if allCharsAllowed && len(token) > 1 {
						isAllowed = true
					} else {
						for _, flag := range allowedFlags {
							if token == flag {
								isAllowed = true
								break
							}
						}
					}
				} else {
					for _, flag := range allowedFlags {
						if token == flag {
							isAllowed = true
							break
						}
					}
				}

				if !isAllowed {
					return errors.New("Flag " + token + " is not allowed for binary " + binary)
				}
			}
		}
	}

	return nil
}
