package db

import (
	"testing"
)

func TestConvertQuery(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "no args",
			input:    "SELECT * FROM users",
			expected: "SELECT * FROM users",
		},
		{
			name:     "positional args",
			input:    "SELECT * FROM users WHERE id = $1 AND name = $2",
			expected: "SELECT * FROM users WHERE id = ?1 AND name = ?2",
		},
		{
			name:     "string literal with $1",
			input:    "SELECT * FROM users WHERE name = '$1' AND id = $1",
			expected: "SELECT * FROM users WHERE name = '$1' AND id = ?1",
		},
		{
			name:     "for update skip locked",
			input:    "SELECT * FROM users WHERE status = 'pending' FOR UPDATE SKIP LOCKED",
			expected: "SELECT * FROM users WHERE status = 'pending'",
		},
		{
			name:     "json operator ::json->>",
			input:    "SELECT id FROM table WHERE payload::json->>'role' = $1",
			expected: "SELECT id FROM table WHERE json_extract(payload, '$.role') = ?1",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			actual := convertQuery(tt.input)
			if actual != tt.expected {
				t.Errorf("expected %q, got %q", tt.expected, actual)
			}
		})
	}
}
