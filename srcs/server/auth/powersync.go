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

// PowerSyncClaims defines the JWT claims specific to PowerSync
type PowerSyncClaims struct {
	jwt.RegisteredClaims
	OrganizationID string `json:"organization_id"`
	UserID         string `json:"user_id"`
}

// GeneratePowerSyncKeypair generates an Ed25519 keypair from a seed or randomly.
func GeneratePowerSyncKeypair() (ed25519.PublicKey, ed25519.PrivateKey) {
	seed := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
	if seed != "" {
		decoded, err := base64.RawURLEncoding.DecodeString(seed)
		if err == nil && len(decoded) == ed25519.SeedSize {
			priv := ed25519.NewKeyFromSeed(decoded)
			return priv.Public().(ed25519.PublicKey), priv
		}
	}
	pub, priv, err := ed25519.GenerateKey(nil)
	if err != nil {
		panic(err)
	}
	return pub, priv
}

var (
	powersyncPublicKey  ed25519.PublicKey
	powersyncPrivateKey ed25519.PrivateKey
)

func init() {
	powersyncPublicKey, powersyncPrivateKey = GeneratePowerSyncKeypair()
}

// HandlePowerSyncToken generates a short-lived JWT for PowerSync clients.
func (h *Handlers) HandlePowerSyncToken(w http.ResponseWriter, r *http.Request) {
	claims := ClaimsFromContext(r.Context())
	if claims == nil {
		jsonError(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	now := time.Now().UTC()
	tokenClaims := PowerSyncClaims{
		RegisteredClaims: jwt.RegisteredClaims{
			Subject:   claims.Subject,
			Issuer:    "ohc-api",
			IssuedAt:  jwt.NewNumericDate(now),
			ExpiresAt: jwt.NewNumericDate(now.Add(1 * time.Hour)),
			ID:        generateID(),
		},
		OrganizationID: claims.OrganizationID,
		UserID:         claims.Subject,
	}

	token := jwt.NewWithClaims(jwt.SigningMethodEdDSA, tokenClaims)
	token.Header["kid"] = "powersync-key-1" // Provide a key ID

	signedToken, err := token.SignedString(powersyncPrivateKey)
	if err != nil {
		jsonError(w, "internal server error", http.StatusInternalServerError)
		return
	}

	resp := map[string]interface{}{
		"token": signedToken,
		"expires_at": tokenClaims.ExpiresAt.Time.Format(time.RFC3339),
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}

// HandlePowerSyncJWKS exposes the public keys for PowerSync.
func (h *Handlers) HandlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	// Base64URL encode the raw public key
	x := base64.RawURLEncoding.EncodeToString(powersyncPublicKey)

	jwks := map[string]interface{}{
		"keys": []map[string]interface{}{
			{
				"kty": "OKP",
				"crv": "Ed25519",
				"kid": "powersync-key-1",
				"use": "sig",
				"x":   x,
			},
		},
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(jwks)
}
