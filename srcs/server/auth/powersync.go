package auth

import (
	"crypto"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"math/big"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	powerSyncPrivateKey *rsa.PrivateKey
	powerSyncPublicKey  *rsa.PublicKey
	powerSyncKeyID      = "powersync-rs256-key"
)

func init() {
	// Generate or load an RS256 keypair for signing PowerSync tokens
	// to ensure JWTs remain valid across server restarts and scaled pods.

	// If a private key is provided via environment, use it (Cloud Mode)
	if envKey := os.Getenv("POWERSYNC_RSA_PRIVATE_KEY"); envKey != "" {
		block, _ := pem.Decode([]byte(envKey))
		if block != nil {
			parsedKey, err := x509.ParsePKCS1PrivateKey(block.Bytes)
			if err == nil {
				powerSyncPrivateKey = parsedKey
				powerSyncPublicKey = &powerSyncPrivateKey.PublicKey
				return
			}
		}
	}

	// Fallback to local filesystem state (Standalone Mode)
	homeDir, err := os.UserHomeDir()
	if err != nil {
		homeDir = "."
	}
	openclawDir := filepath.Join(homeDir, ".openclaw")
	_ = os.MkdirAll(openclawDir, 0700)
	keyPath := filepath.Join(openclawDir, "powersync_rsa.pem")

	if keyData, err := os.ReadFile(keyPath); err == nil {
		block, _ := pem.Decode(keyData)
		if block != nil {
			parsedKey, err := x509.ParsePKCS1PrivateKey(block.Bytes)
			if err == nil {
				powerSyncPrivateKey = parsedKey
				powerSyncPublicKey = &powerSyncPrivateKey.PublicKey
				return
			}
		}
	}

	powerSyncPrivateKey, err = rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		panic("failed to generate RSA key for PowerSync: " + err.Error())
	}
	powerSyncPublicKey = &powerSyncPrivateKey.PublicKey

	// Save to file for next startup
	keyBytes := x509.MarshalPKCS1PrivateKey(powerSyncPrivateKey)
	pemBlock := &pem.Block{
		Type:  "RSA PRIVATE KEY",
		Bytes: keyBytes,
	}
	_ = os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)
}


var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/auth")
	powerSyncTokensGenerated, _ = meter.Int64Counter(
		"auth.powersync.tokens_generated",
		metric.WithDescription("Number of PowerSync tokens generated"),
	)
)

// HandleJWKS serves the JSON Web Key Set containing the public key for verifying PowerSync tokens.
func (h *Handlers) HandleJWKS(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	n := base64.RawURLEncoding.EncodeToString(powerSyncPublicKey.N.Bytes())
	e := base64.RawURLEncoding.EncodeToString(big.NewInt(int64(powerSyncPublicKey.E)).Bytes())

	jwks := map[string]interface{}{
		"keys": []map[string]interface{}{
			{
				"kty": "RSA",
				"kid": powerSyncKeyID,
				"alg": "RS256",
				"use": "sig",
				"n":   n,
				"e":   e,
			},
		},
	}

	writeJSON(w, http.StatusOK, jwks)
}

// HandlePowerSyncToken provides a token specifically for the PowerSync client.
// It exchanges the standard OHC session token for a PowerSync compatible token.
func (h *Handlers) HandlePowerSyncToken(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// 1. Authenticate the request using the existing auth middleware / token
	// Actually, this endpoint needs to be authenticated.
	tokenStr := extractToken(r)
	if tokenStr == "" {
		jsonError(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	claims, err := h.store.ValidateToken(tokenStr)
	if err != nil {
		jsonError(w, "invalid token", http.StatusUnauthorized)
		return
	}

	// 2. Generate a PowerSync token
	// PowerSync expects standard claims (sub, exp) and optionally others.
	now := time.Now().UTC()
	psClaims := map[string]interface{}{
		"sub":   claims.Subject,
		"user_id": claims.Subject, // PowerSync uses this via bucket.user_id
		"iat":   now.Unix(),
		"exp":   now.Add(24 * time.Hour).Unix(),
		"aud":   "powersync",
	}

	// Sign it using RS256 and the generated private key
	hdr, _ := json.Marshal(map[string]string{
		"alg": "RS256",
		"typ": "JWT",
		"kid": powerSyncKeyID,
	})
	pay, _ := json.Marshal(psClaims)
	sigInput := base64.RawURLEncoding.EncodeToString(hdr) + "." + base64.RawURLEncoding.EncodeToString(pay)

	hashed := sha256.Sum256([]byte(sigInput))
	signatureBytes, err := rsa.SignPKCS1v15(rand.Reader, powerSyncPrivateKey, crypto.SHA256, hashed[:])
	if err != nil {
		jsonError(w, "failed to sign token", http.StatusInternalServerError)
		return
	}
	psToken := sigInput + "." + base64.RawURLEncoding.EncodeToString(signatureBytes)

	resp := map[string]interface{}{
		"token":     psToken,
		"expiresAt": psClaims["exp"],
	}

	// Record token generation metric via OpenTelemetry
	powerSyncTokensGenerated.Add(r.Context(), 1, metric.WithAttributes(
		attribute.String("subject", claims.Subject),
	))

	writeJSON(w, http.StatusOK, resp)
}
