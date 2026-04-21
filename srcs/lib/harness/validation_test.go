package harness

import "testing"

func TestValidateCommand(t *testing.T) {
	tests := []struct {
		cmd     string
		wantErr bool
	}{
		{"echo hello", false},
		{"ls --color=auto", false},
		{"export FOO=bar", false},
		{"echo <(ls)", true},
		{"echo >(ls)", true},
		{"echo $[1+1]", true},
		{"echo =ls", true},
		{"=ls", true},
		{"zmodload zsh/net/tcp", true},
		{"cat .git/config", true},
	}

	for _, tt := range tests {
		t.Run(tt.cmd, func(t *testing.T) {
			err := ValidateCommand(tt.cmd)
			if (err != nil) != tt.wantErr {
				t.Errorf("ValidateCommand(%q) error = %v, wantErr %v", tt.cmd, err, tt.wantErr)
			}
		})
	}
}
