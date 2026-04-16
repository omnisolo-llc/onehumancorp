package crypto

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/hmac"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"io"
)

// Encrypt encrypts data using AES-GCM with a key derived from the provided secret.
func Encrypt(plaintext []byte, secret string) (string, error) {
	if secret == "" {
		return "", errors.New("encryption secret is required")
	}

	key := sha256.Sum256([]byte(secret))
	block, err := aes.NewCipher(key[:])
	if err != nil {
		return "", err
	}

	gcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", err
	}

	nonce := make([]byte, gcm.NonceSize())
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		return "", err
	}

	ciphertext := gcm.Seal(nonce, nonce, plaintext, nil)
	return hex.EncodeToString(ciphertext), nil
}

// EncryptDeterministic encrypts data using AES-GCM with a nonce derived from the plaintext and secret.
// This allows for searchable encrypted fields while remaining cryptographically strong.
func EncryptDeterministic(plaintext []byte, secret string) (string, error) {
	if secret == "" {
		return "", errors.New("encryption secret is required")
	}

	key := sha256.Sum256([]byte(secret))
	block, err := aes.NewCipher(key[:])
	if err != nil {
		return "", err
	}

	gcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", err
	}

	// Generate deterministic nonce using HMAC-SHA256 of plaintext keyed with the secret key
	h := hmac.New(sha256.New, key[:])
	h.Write(plaintext)
	nonce := h.Sum(nil)[:gcm.NonceSize()]

	ciphertext := gcm.Seal(nil, nonce, plaintext, nil)
	// We prefix with the nonce (even though it is deterministic) so Decrypt can use the same code path
	final := append(nonce, ciphertext...)
	return hex.EncodeToString(final), nil
}

// Decrypt decrypts data using AES-GCM with a key derived from the provided secret.
func Decrypt(ciphertextHex string, secret string) ([]byte, error) {
	if secret == "" {
		return nil, errors.New("encryption secret is required")
	}

	ciphertext, err := hex.DecodeString(ciphertextHex)
	if err != nil {
		return nil, fmt.Errorf("decode hex: %w", err)
	}

	key := sha256.Sum256([]byte(secret))
	block, err := aes.NewCipher(key[:])
	if err != nil {
		return nil, err
	}

	gcm, err := cipher.NewGCM(block)
	if err != nil {
		return nil, err
	}

	nonceSize := gcm.NonceSize()
	if len(ciphertext) < nonceSize {
		return nil, errors.New("ciphertext too short")
	}

	nonce, actualCiphertext := ciphertext[:nonceSize], ciphertext[nonceSize:]
	plaintext, err := gcm.Open(nil, nonce, actualCiphertext, nil)
	if err != nil {
		return nil, fmt.Errorf("decrypt: %w", err)
	}

	return plaintext, nil
}
