package terminal

import (
	"testing"
)

func TestTokenValidator_Validate(t *testing.T) {
	v := NewTokenValidator()

	tests := []struct {
		name    string
		command string
		wantErr bool
	}{
		{"safe command", "ls -l", false},
		{"sudo block", "sudo rm -rf /", true},
		{"zmodload block", "zmodload zsh/net/tcp", true},
		{"emulate block", "emulate sh", true},
		{"zpty block", "zpty mypty cat", true},
		{"process substitution read", "cat <(ls)", true},
		{"process substitution write", "echo >(cat)", true},
		{"process substitution diff", "diff =(ls) =(ls)", true},
		{"process substitution newline", "cat <(\nls\n)", true},
		{"subshell block", "echo $(ls)", true},
		{"backtick block", "echo `ls`", true},
		{"safe bash array", "FOO=bar my_array=(1 2)", false},
		{"flag grep ok", "grep -i test file", false},
		{"flag grep comb ok", "grep -ivE test file", false},
		{"flag grep block", "grep -iP test file", true},
		{"flag ls long block", "ls --color=auto", true},
		{"flag find ok", "find -name *.txt", false},
		{"flag find block", "find -exec rm {} \\;", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if err := v.Validate(tt.command); (err != nil) != tt.wantErr {
				t.Errorf("TokenValidator.Validate() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}
