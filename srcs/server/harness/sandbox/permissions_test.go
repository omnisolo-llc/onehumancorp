package sandbox

import "testing"

func TestPermissionEvaluator_IsAllowed(t *testing.T) {
	eval := NewPermissionEvaluator([]string{"rm -rf /"}, []string{"^echo.*", "^ls.*"})

	if eval.IsAllowed("rm -rf /") {
		t.Error("Expected 'rm -rf /' to be blocked")
	}

	if !eval.IsAllowed("echo hello") {
		t.Error("Expected 'echo hello' to be allowed")
	}

	if eval.IsAllowed("cat /etc/shadow") {
		t.Error("Expected 'cat /etc/shadow' to be blocked as it does not match allowed patterns")
	}
}

func TestPermissionEvaluator_NoPatterns(t *testing.T) {
	eval := NewPermissionEvaluator([]string{"bad_cmd"}, nil)

	if eval.IsAllowed("bad_cmd") {
		t.Error("Expected 'bad_cmd' to be blocked")
	}

	if !eval.IsAllowed("any_other_cmd") {
		t.Error("Expected 'any_other_cmd' to be allowed when no allowed patterns are defined")
	}
}
