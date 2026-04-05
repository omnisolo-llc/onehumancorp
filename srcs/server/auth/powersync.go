package auth

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"os"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

var (
	powerSyncPubKey  ed25519.PublicKey
	powerSyncPrivKey ed25519.PrivateKey
	powerSyncKeyID   = "powersync-ed25519-1"
)

func init() {
	// Initialize deterministic Ed25519 keys for PowerSync from environment to ensure
	// all API pods in Cloud-Native K8s deployment share the same signing key for JWTs.
	// If OHC_POWERSYNC_PRIV_KEY is not set (e.g. local dev), we fall back to a hardcoded
	// deterministic seed.

	seedHex := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
	var seedBytes []byte

	if seedHex != "" {
		var err error
		seedBytes, err = base64.StdEncoding.DecodeString(seedHex)
		if err != nil || len(seedBytes) != ed25519.SeedSize {
			// fallback to deterministic key if env var is misconfigured
			seedBytes = getDeterministicSeed()
		}
	} else {
		seedBytes = getDeterministicSeed()
	}

	powerSyncPrivKey = ed25519.NewKeyFromSeed(seedBytes)
	powerSyncPubKey = powerSyncPrivKey.Public().(ed25519.PublicKey)
}

func getDeterministicSeed() []byte {
	// 32-byte deterministic seed for development / fallback
	return []byte("ohc-powersync-deterministic-seed")
}

type powerSyncTokenResponse struct {
	Token     string    `json:"token"`
	ExpiresAt time.Time `json:"expiresAt"`
}

// HandlePowerSyncToken generates a short-lived JWT for PowerSync clients.
// Accepts parameters: h *Handlers (No Constraints).
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

	orgID := claims.OrganizationID
	if orgID == "" {
		orgID = "default"
	}

	now := time.Now().UTC()
	expiresAt := now.Add(1 * time.Hour)

	jwtClaims := jwt.MapClaims{
		"iss": "onehumancorp",
		"sub": claims.Subject,
		"iat": now.Unix(),
		"exp": expiresAt.Unix(),
		"parameters": map[string]string{
			"organization_id": orgID,
			"user_id":         claims.Subject,
		},
	}

	token := jwt.NewWithClaims(jwt.SigningMethodEdDSA, jwtClaims)
	token.Header["kid"] = powerSyncKeyID

	signedToken, err := token.SignedString(powerSyncPrivKey)
	if err != nil {
		jsonError(w, "failed to sign powersync token", http.StatusInternalServerError)
		return
	}

	writeJSON(w, http.StatusOK, powerSyncTokenResponse{
		Token:     signedToken,
		ExpiresAt: expiresAt,
	})
}

type jwksResponse struct {
	Keys []map[string]interface{} `json:"keys"`
}

// HandlePowerSyncJWKS returns the public keys used to sign PowerSync tokens.
func (h *Handlers) HandlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	pubKeyBytes := []byte(powerSyncPubKey)

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)

	// Since it's Ed25519, standard JWK format applies:
	jwks := jwksResponse{
		Keys: []map[string]interface{}{
			{
				"kty": "OKP",
				"crv": "Ed25519",
				"kid": powerSyncKeyID,
				"x":   base64.RawURLEncoding.EncodeToString(pubKeyBytes),
				"use": "sig",
			},
		},
	}

	_ = json.NewEncoder(w).Encode(jwks)
}
