package webhooks

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
)

// VerifySignature verifies the HMAC SHA256 signature of a webhook payload.
func VerifySignature(payload []byte, secret, signatureHeader string) bool {
	mac := hmac.New(sha256.New, []byte(secret))
	mac.Write(payload)
	expectedMAC := hex.EncodeToString(mac.Sum(nil))
	return hmac.Equal([]byte(expectedMAC), []byte(signatureHeader))
}
