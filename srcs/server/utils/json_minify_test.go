package utils

import (
	"testing"
)

func TestMinifyJSONString(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "valid json object",
			input:    "{\n  \"hello\": \"world\"\n}",
			expected: `{"hello":"world"}`,
		},
		{
			name:     "valid json array",
			input:    "[\n  1,\n  2\n]",
			expected: `[1,2]`,
		},
		{
			name:     "invalid json",
			input:    "{\n  \"hello\": \"world\",\n}",
			expected: "{\n  \"hello\": \"world\",\n}",
		},
		{
			name:     "not json",
			input:    "hello world",
			expected: "hello world",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := MinifyJSONString(tt.input)
			if got != tt.expected {
				t.Errorf("expected %q, got %q", tt.expected, got)
			}
		})
	}
}
