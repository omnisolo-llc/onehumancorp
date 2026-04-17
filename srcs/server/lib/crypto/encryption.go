package crypto

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
)

// EncryptDeterministic encrypts plaintext using AES-GCM with a deterministic nonce
// derived from the plaintext and secret. This ensures that the same plaintext
// always results in the same ciphertext for a given secret, enabling
// direct database lookups of encrypted fields.
func EncryptDeterministic(plaintext string, secret string) (string, error) {
	if secret == "" {
		return "", errors.New("crypto: empty secret")
	}

	secretBytes := deriveKey(secret)
	plaintextBytes := []byte(plaintext)

	// Derive 12-byte nonce from plaintext and secret using HMAC-SHA256
	h := hmac.New(sha256.New, secretBytes)
	h.Write(plaintextBytes)
	nonce := h.Sum(nil)[:12]

	block, err := aes.NewCipher(secretBytes)
	if err != nil {
		return "", fmt.Errorf("crypto: failed to create cipher: %w", err)
	}

	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", fmt.Errorf("crypto: failed to create GCM: %w", err)
	}

	// Seal the plaintext. Since nonce is deterministic, the result is deterministic.
	ciphertext := aesgcm.Seal(nil, nonce, plaintextBytes, nil)

	// Prepend nonce to ciphertext and encode as hex for database storage
	combined := append(nonce, ciphertext...)
	return hex.EncodeToString(combined), nil
}

// Decrypt decrypts a hex-encoded ciphertext string previously encrypted with
// EncryptDeterministic.
func Decrypt(encodedCiphertext string, secret string) (string, error) {
	if secret == "" {
		return "", errors.New("crypto: empty secret")
	}

	combined, err := hex.DecodeString(encodedCiphertext)
	if err != nil {
		return "", fmt.Errorf("crypto: failed to decode hex: %w", err)
	}

	if len(combined) < 12 {
		return "", errors.New("crypto: ciphertext too short")
	}

	secretBytes := deriveKey(secret)
	nonce := combined[:12]
	ciphertext := combined[12:]

	block, err := aes.NewCipher(secretBytes)
	if err != nil {
		return "", fmt.Errorf("crypto: failed to create cipher: %w", err)
	}

	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", fmt.Errorf("crypto: failed to create GCM: %w", err)
	}

	plaintext, err := aesgcm.Open(nil, nonce, ciphertext, nil)
	if err != nil {
		return "", fmt.Errorf("crypto: decryption failed: %w", err)
	}

	return string(plaintext), nil
}

// deriveKey ensures we have a 32-byte key for AES-256 by hashing the input secret.
func deriveKey(secret string) []byte {
	h := sha256.Sum256([]byte(secret))
	return h[:]
}
