package powersync

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

var privateKey *rsa.PrivateKey

func init() {
	var err error
	privateKey, err = loadOrGenerateKey()
	if err != nil {
		slog.Error("Failed to initialize PowerSync RSA key", "error", err)
	}
}

func loadOrGenerateKey() (*rsa.PrivateKey, error) {
	// Try environment variable first
	if envKey := os.Getenv("POWERSYNC_RSA_PRIVATE_KEY"); envKey != "" {
		block, _ := pem.Decode([]byte(envKey))
		if block == nil {
			return nil, fmt.Errorf("failed to decode PEM block containing private key")
		}
		key, err := x509.ParsePKCS1PrivateKey(block.Bytes)
		if err == nil {
			return key, nil
		}
		// Try parsing as PKCS8
		if key8, err := x509.ParsePKCS8PrivateKey(block.Bytes); err == nil {
			if rsaKey, ok := key8.(*rsa.PrivateKey); ok {
				return rsaKey, nil
			}
		}
		return nil, fmt.Errorf("failed to parse private key")
	}

	// Local fallback for standalone mode
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return generateKey()
	}

	openclawDir := filepath.Join(homeDir, ".openclaw")
	keyPath := filepath.Join(openclawDir, "powersync_rsa.pem")

	if _, err := os.Stat(keyPath); err == nil {
		// Key exists, load it
		keyBytes, err := os.ReadFile(keyPath)
		if err == nil {
			block, _ := pem.Decode(keyBytes)
			if block != nil {
				if key, err := x509.ParsePKCS1PrivateKey(block.Bytes); err == nil {
					return key, nil
				}
			}
		}
	}

	// Generate a new key
	key, err := generateKey()
	if err != nil {
		return nil, err
	}

	// Save the key for future use
	err = os.MkdirAll(openclawDir, 0700)
	if err == nil {
		keyBytes := x509.MarshalPKCS1PrivateKey(key)
		pemBlock := &pem.Block{
			Type:  "RSA PRIVATE KEY",
			Bytes: keyBytes,
		}
		_ = os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)
	}

	return key, nil
}

func generateKey() (*rsa.PrivateKey, error) {
	return rsa.GenerateKey(rand.Reader, 2048)
}

// JWKSHandler returns the JWKS for PowerSync authentication.
func JWKSHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if privateKey == nil {
			http.Error(w, "Private key not initialized", http.StatusInternalServerError)
			return
		}

		pubKey := &privateKey.PublicKey
		n := pubKey.N.Bytes()
		e := pubKey.E

		// Base64url encode n and e for JWKS format
		var eBytes []byte
		if e < 256 {
			eBytes = []byte{byte(e)}
		} else if e < 65536 {
			eBytes = []byte{byte(e >> 8), byte(e)}
		} else {
			eBytes = []byte{byte(e >> 16), byte(e >> 8), byte(e)}
		}

		jwks := map[string]interface{}{
			"keys": []map[string]interface{}{
				{
					"kty": "RSA",
					"kid": "powersync-key-1",
					"use": "sig",
					"alg": "RS256",
					"n":   base64.RawURLEncoding.EncodeToString(n),
					"e":   base64.RawURLEncoding.EncodeToString(eBytes),
				},
			},
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(jwks)
	}
}

// TokenHandler generates a PowerSync JWT token.
func TokenHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if privateKey == nil {
			http.Error(w, "Private key not initialized", http.StatusInternalServerError)
			return
		}

		userID := r.URL.Query().Get("user_id")
		if userID == "" {
			userID = "anonymous"
		}

		token := jwt.NewWithClaims(jwt.SigningMethodRS256, jwt.MapClaims{
			"iss": "openclaw-server",
			"sub": userID,
			"aud": "powersync",
			"exp": time.Now().Add(time.Hour * 24).Unix(),
			"iat": time.Now().Unix(),
		})

		token.Header["kid"] = "powersync-key-1"

		tokenString, err := token.SignedString(privateKey)
		if err != nil {
			http.Error(w, "Failed to sign token", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{
			"token": tokenString,
		})
	}
}
