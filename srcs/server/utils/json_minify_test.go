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
			name:     "Empty string",
			input:    "",
			expected: "",
		},
		{
			name:     "Whitespace only",
			input:    "   \n  \t ",
			expected: "   \n  \t ",
		},
		{
			name:     "Not JSON",
			input:    "Just a normal string",
			expected: "Just a normal string",
		},
		{
			name:     "Not JSON with whitespace",
			input:    "  Just a normal string  \n",
			expected: "  Just a normal string  \n",
		},
		{
			name: "Valid JSON Object",
			input: `{
				"key1": "value1",
				"key2": 2
			}`,
			expected: `{"key1":"value1","key2":2}`,
		},
		{
			name: "Valid JSON Object with surrounding whitespace",
			input: `  {
				"key1": "value1",
				"key2": 2
			}  `,
			expected: `{"key1":"value1","key2":2}`,
		},
		{
			name: "Valid JSON Array",
			input: `[
				"item1",
				"item2"
			]`,
			expected: `["item1","item2"]`,
		},
		{
			name: "Invalid JSON that looks like JSON",
			input: `{
				"key1": "value1",
				"key2": 2,
			}`,
			expected: `{
				"key1": "value1",
				"key2": 2,
			}`,
		},
		{
			name: "Invalid JSON that looks like JSON with surrounding whitespace",
			input: `  {
				"key1": "value1",
				"key2": 2,
			}  `,
			expected: `  {
				"key1": "value1",
				"key2": 2,
			}  `,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := MinifyJSONString(tt.input)
			if result != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, result)
			}
		})
	}
}
