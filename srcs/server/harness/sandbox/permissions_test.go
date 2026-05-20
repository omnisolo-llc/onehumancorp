package sandbox

import "testing"

func TestPermissionEvaluatorAllowed(t *testing.T) {
	pe := NewPermissionEvaluator()

	allowed := []string{
		"echo 'hello world'",
		"ls -l /tmp",
		"cat file.txt | grep foo",
	}

	for _, cmd := range allowed {
		if !pe.Evaluate(cmd) {
			t.Errorf("Expected %q to be allowed", cmd)
		}
	}
}

func TestPermissionEvaluatorDeniedCommands(t *testing.T) {
	pe := NewPermissionEvaluator()

	denied := []string{
		"rm -rf /",
		"mkfs.ext4 /dev/sda1",
	}

	for _, cmd := range denied {
		if pe.Evaluate(cmd) {
			t.Errorf("Expected %q to be denied", cmd)
		}
	}
}

func TestPermissionEvaluatorDeniedPatterns(t *testing.T) {
	pe := NewPermissionEvaluator()

	denied := []string{
		"sudo apt-get update",
		"SUDO rm -rf /tmp/*",
		"chown root:root /etc/passwd",
	}

	for _, cmd := range denied {
		if pe.Evaluate(cmd) {
			t.Errorf("Expected %q to be denied", cmd)
		}
	}
}

func TestPermissionEvaluatorUpdatePolicy(t *testing.T) {
	pe := NewPermissionEvaluator()
	policy := SandboxPolicy{
		DisabledCommands: []string{"curl"},
		DisabledPatterns: []string{`(?i)\bwget\b`},
	}
	pe.UpdatePolicy(policy)

	denied := []string{
		"curl http://example.com",
		"wget http://example.com",
		"WGET http://example.com",
	}

	for _, cmd := range denied {
		if pe.Evaluate(cmd) {
			t.Errorf("Expected %q to be denied after update", cmd)
		}
	}

	if !pe.Evaluate("echo 'hello'") {
		t.Errorf("Expected 'echo' to be allowed after update")
	}
}
