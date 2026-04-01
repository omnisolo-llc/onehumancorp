package powersync

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"errors"
	"fmt"
	"log/slog"
	"math/big"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"github.com/golang-jwt/jwt/v5"
	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

var (
	rsaPrivateKey *rsa.PrivateKey
	rsaPublicKey  *rsa.PublicKey
	keyID         = "powersync-key-1"
)

func init() {
	// Generate or load RSA key pair for PowerSync JWKS
	if err := loadOrGenerateKeys(); err != nil {
		slog.Error("failed to load/generate PowerSync RSA keys", "error", err)
	}
}

func loadOrGenerateKeys() error {
	// Check if key is provided via environment
	if envKey := os.Getenv("POWERSYNC_RSA_PRIVATE_KEY"); envKey != "" {
		block, _ := pem.Decode([]byte(envKey))
		if block == nil {
			return errors.New("failed to decode PEM block from POWERSYNC_RSA_PRIVATE_KEY")
		}
		var err error
		rsaPrivateKey, err = x509.ParsePKCS1PrivateKey(block.Bytes)
		if err != nil {
			rsaPrivateKey, err = parsePKCS8PrivateKey(block.Bytes)
			if err != nil {
				return fmt.Errorf("failed to parse private key: %w", err)
			}
		}
		rsaPublicKey = &rsaPrivateKey.PublicKey
		return nil
	}

	// Fallback to local filesystem storage
	homeDir, err := os.UserHomeDir()
	if err != nil {
		return err
	}
	openclawDir := filepath.Join(homeDir, ".openclaw")
	if err := os.MkdirAll(openclawDir, 0700); err != nil {
		return err
	}
	keyPath := filepath.Join(openclawDir, "powersync_rsa.pem")

	// Try to load existing
	keyBytes, err := os.ReadFile(keyPath)
	if err == nil {
		block, _ := pem.Decode(keyBytes)
		if block != nil {
			rsaPrivateKey, err = x509.ParsePKCS1PrivateKey(block.Bytes)
			if err == nil {
				rsaPublicKey = &rsaPrivateKey.PublicKey
				return nil
			}
			rsaPrivateKey, err = parsePKCS8PrivateKey(block.Bytes)
			if err == nil {
				rsaPublicKey = &rsaPrivateKey.PublicKey
				return nil
			}
		}
	}

	// Generate new key
	slog.Info("generating new RSA key for PowerSync JWKS")
	rsaPrivateKey, err = rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		return err
	}
	rsaPublicKey = &rsaPrivateKey.PublicKey

	// Save to filesystem
	keyBytes = pem.EncodeToMemory(&pem.Block{
		Type:  "RSA PRIVATE KEY",
		Bytes: x509.MarshalPKCS1PrivateKey(rsaPrivateKey),
	})
	if err := os.WriteFile(keyPath, keyBytes, 0600); err != nil {
		return err
	}

	return nil
}

func parsePKCS8PrivateKey(der []byte) (*rsa.PrivateKey, error) {
	key, err := x509.ParsePKCS8PrivateKey(der)
	if err != nil {
		return nil, err
	}
	rsaKey, ok := key.(*rsa.PrivateKey)
	if !ok {
		return nil, errors.New("not an RSA private key")
	}
	return rsaKey, nil
}

// PowerSyncTokenHandler handles requests for PowerSync JWT tokens
func PowerSyncTokenHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		authClaims := auth.ClaimsFromContext(r.Context())
		if authClaims == nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		if rsaPrivateKey == nil {
			http.Error(w, "Internal Server Error: Missing RSA key", http.StatusInternalServerError)
			return
		}

		now := time.Now()
		claims := jwt.MapClaims{
			"iss": "openclaw-powersync",
			"sub": authClaims.Subject,
			"aud": "powersync",
			"iat": now.Unix(),
			"exp": now.Add(24 * time.Hour).Unix(),
			"jti": uuid.New().String(),
			// PowerSync-specific claims (e.g. for Tenant routing)
			"organization_id": authClaims.OrganizationID,
		}

		token := jwt.NewWithClaims(jwt.SigningMethodRS256, claims)
		token.Header["kid"] = keyID

		tokenString, err := token.SignedString(rsaPrivateKey)
		if err != nil {
			slog.Error("failed to sign PowerSync token", "error", err)
			http.Error(w, "Internal Server Error", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{
			"token":      tokenString,
			"expires_at": now.Add(24 * time.Hour).Format(time.RFC3339),
		})
	}
}

// JWKSHandler serves the public key in JWKS format for PowerSync
func JWKSHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if rsaPublicKey == nil {
			http.Error(w, "Internal Server Error: Missing RSA key", http.StatusInternalServerError)
			return
		}

		// Convert public key to JWK components manually
		n := base64.RawURLEncoding.EncodeToString(rsaPublicKey.N.Bytes())

		eBytes := big.NewInt(int64(rsaPublicKey.E)).Bytes()
		e := base64.RawURLEncoding.EncodeToString(eBytes)

		jwks := map[string]interface{}{
			"keys": []map[string]interface{}{
				{
					"kty": "RSA",
					"kid": keyID,
					"use": "sig",
					"alg": "RS256",
					"n":   n,
					"e":   e,
				},
			},
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(jwks)
	}
}
