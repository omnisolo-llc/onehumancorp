package auth

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/hmac"
	"crypto/rand"
	"crypto/sha256"
	"encoding/base64"
	"encoding/hex"
	"os"
	"path/filepath"
	"strings"
)

func getCryptoKey() []byte {
	key := os.Getenv("OHC_SQLITE_KEY")
	if key == "" {
		key = os.Getenv("OHC_SQLITE_ENCRYPTION_KEY")
	}
	if key == "" {
		if os.Getenv("OHC_STANDALONE") == "true" {
			if os.Getenv("CI") != "true" && !strings.Contains(os.Args[0], "test") {
				homeDir, err := os.UserHomeDir()
				keyDir := ""
				if err == nil {
					keyDir = filepath.Join(homeDir, ".ohc")
				} else {
					keyDir = os.TempDir()
				}
				if err := os.MkdirAll(keyDir, 0700); err == nil {
					keyFile := filepath.Join(keyDir, ".ohc_key")
					if keyData, err := os.ReadFile(keyFile); err == nil {
						key = string(keyData)
					} else {
						newKey := make([]byte, 32)
						if _, err := rand.Read(newKey); err == nil {
							key = hex.EncodeToString(newKey)
							_ = os.WriteFile(keyFile, []byte(key), 0600)
						} else {
							key = "standalone_ephemeral_key"
						}
					}
				} else {
					key = "standalone_ephemeral_key"
				}
			} else {
				key = "standalone_ephemeral_key"
			}
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
