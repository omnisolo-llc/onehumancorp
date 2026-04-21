package auth

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/hmac"
	"crypto/sha256"
	"encoding/base64"
	"os"
)

func getCryptoKey() []byte {
	key := os.Getenv("OHC_SQLITE_KEY")
	if key == "" {
		key = os.Getenv("OHC_SQLITE_ENCRYPTION_KEY")
	}
	if key == "" {
		if os.Getenv("OHC_STANDALONE") == "true" {
			key = "standalone_ephemeral_key"
		} else {
			key = "transient_memory_key"
		}
	}
	// Pad or truncate to 32 bytes for AES-256
	hash := sha256.Sum256([]byte(key))
	return hash[:]
}

// EncryptDeterministic encrypts a string using AES-GCM with a deterministic nonce
// derived from the plaintext. This allows for exact-match database lookups.
func EncryptDeterministic(plaintext string) string {
	if plaintext == "" {
		return ""
	}
	key := getCryptoKey()
	block, err := aes.NewCipher(key)
	if err != nil {
		panic(err) // Should not happen with valid 32-byte key
	}
	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		panic(err)
	}

	// Derive nonce from plaintext using HMAC with the encryption key to ensure determinism securely
	h := hmac.New(sha256.New, key)
	h.Write([]byte(plaintext))
	nonceHash := h.Sum(nil)
	nonce := nonceHash[:aesgcm.NonceSize()]

	ciphertext := aesgcm.Seal(nil, nonce, []byte(plaintext), nil)

	finalMsg := append(nonce, ciphertext...)
	return base64.StdEncoding.EncodeToString(finalMsg)
}

func DecryptDeterministic(ciphertextB64 string) string {
	if ciphertextB64 == "" {
		return ""
	}

	ciphertext, err := base64.StdEncoding.DecodeString(ciphertextB64)
	if err != nil {
		return ciphertextB64 // Fallback if not encrypted
	}

	key := getCryptoKey()
	block, err := aes.NewCipher(key)
	if err != nil {
		panic(err)
	}
	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		panic(err)
	}

	nonceSize := aesgcm.NonceSize()
	if len(ciphertext) < nonceSize {
		return ciphertextB64 // Fallback
	}

	nonce, ciphertextData := ciphertext[:nonceSize], ciphertext[nonceSize:]
	plaintext, err := aesgcm.Open(nil, nonce, ciphertextData, nil)
	if err != nil {
		return ciphertextB64 // Fallback to plaintext if decryption fails
	}

	return string(plaintext)
}
