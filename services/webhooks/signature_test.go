package webhooks

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"testing"
)

func TestVerifySignature(t *testing.T) {
	payload := []byte("hello world")
	secret := "my-secret-key"

	mac := hmac.New(sha256.New, []byte(secret))
	mac.Write(payload)
	validSignature := hex.EncodeToString(mac.Sum(nil))

	tests := []struct {
		name      string
		payload   []byte
		secret    string
		signature string
		expected  bool
	}{
		{
			name:      "valid signature",
			payload:   payload,
			secret:    secret,
			signature: validSignature,
			expected:  true,
		},
		{
			name:      "invalid signature",
			payload:   payload,
			secret:    secret,
			signature: "invalid-sig",
			expected:  false,
		},
		{
			name:      "wrong secret",
			payload:   payload,
			secret:    "wrong-secret",
			signature: validSignature,
			expected:  false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := VerifySignature(tt.payload, tt.secret, tt.signature)
			if result != tt.expected {
				t.Errorf("VerifySignature() = %v, want %v", result, tt.expected)
			}
		})
	}
}
