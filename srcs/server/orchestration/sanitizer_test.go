package orchestration

import (
	"testing"
)

func TestSanitizePayload(t *testing.T) {
	tests := []struct {
		name     string
		payload  string
		expected string
	}{
		{
			name:     "No private tags",
			payload:  `{"key": "value"}`,
			expected: `{"key": "value"}`,
		},
		{
			name:     "One private tag",
			payload:  `{"secret": "[PRIVATE:12345]"}`,
			expected: `{"secret": "[REDACTED]"}`,
		},
		{
			name:     "Multiple private tags",
			payload:  `{"secret1": "[PRIVATE:abc]", "secret2": "[PRIVATE:def]"}`,
			expected: `{"secret1": "[REDACTED]", "secret2": "[REDACTED]"}`,
		},
		{
			name:     "Empty private tag",
			payload:  `{"secret": "[PRIVATE:]"}`,
			expected: `{"secret": "[REDACTED]"}`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := SanitizePayload(tt.payload)
			if err != nil {
				t.Errorf("SanitizePayload() error = %v, wantErr %v", err, nil)
				return
			}
			if got != tt.expected {
				t.Errorf("SanitizePayload() = %v, want %v", got, tt.expected)
			}
		})
	}
}
