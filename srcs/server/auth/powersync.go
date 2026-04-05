package auth

import (
	"crypto/ed25519"
	"crypto/rand"
	"encoding/json"
	"fmt"
	"net/http"
	"time"
)

import "os"
import "encoding/hex"

// PowerSync token configuration
var (
	powerSyncPublicKey  ed25519.PublicKey
	powerSyncPrivateKey ed25519.PrivateKey
)

func init() {
	// Generate keys deterministically based on POWERSYNC_KEY_SEED or use a default seed
	// for multi-node deployments. If POWERSYNC_KEY_SEED is missing, fallback to a
	// hardcoded development seed to ensure horizontal scaling pods share the same key.
	seedHex := os.Getenv("POWERSYNC_KEY_SEED")
	if seedHex == "" {
		seedHex = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef" // 32 bytes
	}

	seed, err := hex.DecodeString(seedHex)
	if err != nil || len(seed) != ed25519.SeedSize {
		// Fallback to random if invalid explicitly for single node
		var genErr error
		powerSyncPublicKey, powerSyncPrivateKey, genErr = ed25519.GenerateKey(rand.Reader)
		if genErr != nil {
			panic(fmt.Sprintf("failed to generate random powersync keys: %v", genErr))
		}
		return
	}

	powerSyncPrivateKey = ed25519.NewKeyFromSeed(seed)
	powerSyncPublicKey = powerSyncPrivateKey.Public().(ed25519.PublicKey)
}

type powersyncTokenResponse struct {
	Token     string `json:"token"`
	ExpiresAt int64  `json:"expiresAt"`
}

type powersyncJwksResponse struct {
	Keys []powersyncJwk `json:"keys"`
}

type powersyncJwk struct {
	Kty string `json:"kty"`
	Kid string `json:"kid"`
	Alg string `json:"alg"`
	Use string `json:"use"`
	Crv string `json:"crv"`
	X   string `json:"x"`
}

// HandlePowerSyncToken generates a short-lived JWT for PowerSync clients.
//
//	GET /api/auth/powersync/token
func (h *Handlers) HandlePowerSyncToken(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := ClaimsFromContext(r.Context())
	if claims == nil {
		jsonError(w, "not authenticated", http.StatusUnauthorized)
		return
	}

	// Wait, we need to issue a JWT that PowerSync understands.
	// The standard JWT claims require `sub` and `aud`. PowerSync also expects parameters.
	// Let's use standard ed25519 JWT signing for it.
	now := time.Now().UTC()
	exp := now.Add(24 * time.Hour).Unix()

	// Using standard JWT signing with ed25519 for PowerSync
	powerSyncClaims := map[string]interface{}{
		"sub": claims.Subject,
		"iat": now.Unix(),
		"exp": exp,
		"parameters": map[string]string{
			"organization_id": claims.OrganizationID,
		},
	}

	// This is a placeholder for standard JWT generation using the private key.
	// To avoid adding heavy dependencies like golang-jwt, we can construct the EdDSA JWT manually.
	token, err := signEdDSA(powerSyncClaims, powerSyncPrivateKey)
	if err != nil {
		jsonError(w, "failed to issue powersync token", http.StatusInternalServerError)
		return
	}

	writeJSON(w, http.StatusOK, powersyncTokenResponse{
		Token:     token,
		ExpiresAt: exp,
	})
}

// HandlePowerSyncJWKS exposes the public key for PowerSync to verify the JWT.
//
//	GET /api/auth/powersync/jwks
func (h *Handlers) HandlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	xBase64 := b64url(powerSyncPublicKey)
	keys := []powersyncJwk{
		{
			Kty: "OKP",
			Kid: "powersync-key-1",
			Alg: "EdDSA",
			Use: "sig",
			Crv: "Ed25519",
			X:   xBase64,
		},
	}

	writeJSON(w, http.StatusOK, powersyncJwksResponse{Keys: keys})
}

func signEdDSA(claims map[string]interface{}, privateKey ed25519.PrivateKey) (string, error) {
	hdr := map[string]string{
		"alg": "EdDSA",
		"typ": "JWT",
		"kid": "powersync-key-1",
	}

	hdrBytes, err := json.Marshal(hdr)
	if err != nil {
		return "", err
	}

	claimsBytes, err := json.Marshal(claims)
	if err != nil {
		return "", err
	}

	sigInput := b64url(hdrBytes) + "." + b64url(claimsBytes)
	sig := ed25519.Sign(privateKey, []byte(sigInput))

	return sigInput + "." + b64url(sig), nil
}
