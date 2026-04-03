package orchestration

import (
	"testing"
)

func TestOptimizePromptForCost(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "basic whitespace compression",
			input:    "This   is  a    test\n\n\nwith multiple   lines.",
			expected: "This   is  a    test\n\nwith multiple   lines.",
		},
		{
			name:     "markdown comments removed",
			input:    "Hello <!-- this is a hidden comment --> World",
			expected: "Hello  World",
		},
		{
			name:     "complex prompt with indentation and multiple newlines",
			input:    "System Prompt:\n\n\n\n    You are an AI.\n    <!-- secret instructions -->\n    Execute.",
			expected: "System Prompt:\n\n    You are an AI.\n    \n    Execute.",
		},
		{
			name:     "empty prompt",
			input:    "   \n\n  ",
			expected: "",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := OptimizePromptForCost(tt.input)
			if result != tt.expected {
				t.Errorf("OptimizePromptForCost() = %q, want %q", result, tt.expected)
			}
		})
	}
}
