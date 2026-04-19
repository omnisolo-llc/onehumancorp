package terminal

import (
	"testing"
)

func TestCommandValidator_ZSHBuiltins(t *testing.T) {
	validator := NewDefaultCommandValidator()

	tests := []struct {
		cmd     string
		wantErr error
	}{
		{"echo hello", nil},
		{"zmodload zsh/net/tcp", ErrDangerousZSHBuiltin},
		{"emulate -R zsh", ErrDangerousZSHBuiltin},
		{"zpty -b", ErrDangerousZSHBuiltin},
		{"ls zmodload", nil}, // just a filename
	}

	for _, tt := range tests {
		t.Run(tt.cmd, func(t *testing.T) {
			err := validator.Validate(tt.cmd)
			if err != tt.wantErr {
				t.Errorf("Validate(%q) error = %v, wantErr %v", tt.cmd, err, tt.wantErr)
			}
		})
	}
}

func TestCommandValidator_ProcessSubstitution(t *testing.T) {
	validator := NewDefaultCommandValidator()

	tests := []struct {
		cmd     string
		wantErr error
	}{
		{"diff file1 file2", nil},
		{"diff <(ls) <(ls -l)", ErrProcessSubstitution},
		{"cat >(grep foo)", ErrProcessSubstitution},
		{"cmd =(ls)", ErrProcessSubstitution},
		{"echo '<(not real)'", ErrProcessSubstitution}, // Note: regex currently blocks strings containing the pattern too, which is safer
	}

	for _, tt := range tests {
		t.Run(tt.cmd, func(t *testing.T) {
			err := validator.Validate(tt.cmd)
			if err != tt.wantErr {
				t.Errorf("Validate(%q) error = %v, wantErr %v", tt.cmd, err, tt.wantErr)
			}
		})
	}
}

func TestCommandValidator_ReadOnly(t *testing.T) {
	validator := NewDefaultCommandValidator()

	tests := []struct {
		cmd     string
		wantErr error
	}{
		{"grep -r text", nil},
		{"find . -name test -exec rm {} \\;", ErrUnsafeFlag},
		{"fd -H pattern", nil},
		{"fd -x rm pattern", ErrUnsafeFlag},
		{"cat -n file", nil},
	}

	for _, tt := range tests {
		t.Run(tt.cmd, func(t *testing.T) {
			err := validator.ValidateReadOnly(tt.cmd)
			if tt.wantErr == ErrUnsafeFlag && err != ErrUnsafeFlag {
				t.Errorf("ValidateReadOnly(%q) error = %v, wantErr %v", tt.cmd, err, tt.wantErr)
			}
			if tt.wantErr == nil && err != nil {
			    t.Errorf("ValidateReadOnly(%q) error = %v, wantErr %v", tt.cmd, err, tt.wantErr)
			}
		})
	}
}
