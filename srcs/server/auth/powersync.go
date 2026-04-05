package auth

import (
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"os"
	"time"
)

var (
	// Generate an in-memory key pair for PowerSync JWT signing.
	// In production, these should be securely stored and loaded,
	// but for this hybrid architecture demonstration, dynamic is sufficient.
	powerSyncPublicKey  ed25519.PublicKey
	powerSyncPrivateKey ed25519.PrivateKey
)

func init() {

	var err error
	keyB64 := os.Getenv("POWERSYNC_PRIVATE_KEY_B64")
	if keyB64 != "" {
		keyBytes, err := base64.StdEncoding.DecodeString(keyB64)
		if err == nil && len(keyBytes) == ed25519.PrivateKeySize {
			powerSyncPrivateKey = ed25519.PrivateKey(keyBytes)
			powerSyncPublicKey = powerSyncPrivateKey.Public().(ed25519.PublicKey)
			return
		}
	}
	// Fallback for local development
	powerSyncPublicKey, powerSyncPrivateKey, err = ed25519.GenerateKey(rand.Reader)
	if err != nil {
		panic("failed to generate ed25519 key pair for PowerSync")
	}

}

// HandlePowerSyncToken issues a short-lived JWT for PowerSync replication authentication.
// Endpoint: GET /api/auth/powersync/token
func (h *Handlers) HandlePowerSyncToken(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := ClaimsFromContext(r.Context())
	if claims == nil {
		jsonError(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	orgID := claims.OrganizationID
	if orgID == "" {
		orgID = "default_org"
	}

	now := time.Now().UTC()

	// Create minimal manual JWT for EdDSA (Ed25519)
	hdr := map[string]string{
		"alg": "EdDSA",
		"typ": "JWT",
	}
	hdrBytes, _ := json.Marshal(hdr)

	pay := map[string]interface{}{
		"sub": claims.Subject,
		"iat": now.Unix(),
		"exp": now.Add(24 * time.Hour).Unix(),
		"iss": "ohc-hybrid-auth",
		"aud": "powersync",
		"parameters": map[string]string{
			"organization_id": orgID,
		},
	}
	payBytes, _ := json.Marshal(pay)

	sigInput := b64url(hdrBytes) + "." + b64url(payBytes)
	sig := ed25519.Sign(powerSyncPrivateKey, []byte(sigInput))
	tokenString := sigInput + "." + b64url(sig)

	writeJSON(w, http.StatusOK, map[string]string{
		"token": tokenString,
		"powerSyncUrl": "http://localhost:8081",
		"expiresAt": now.Add(24 * time.Hour).Format(time.RFC3339),
	})
}

// HandlePowerSyncJWKS exposes the public key for PowerSync to verify JWT signatures.
// Endpoint: GET /api/auth/powersync/jwks
func (h *Handlers) HandlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	x := base64.RawURLEncoding.EncodeToString(powerSyncPublicKey)

	jwks := map[string]interface{}{
		"keys": []map[string]interface{}{
			{
				"kty": "OKP",
				"use": "sig",
				"crv": "Ed25519",
				"kid": "powersync-key-1",
				"x":   x,
				"alg": "EdDSA",
			},
		},
	}

	writeJSON(w, http.StatusOK, jwks)
}
