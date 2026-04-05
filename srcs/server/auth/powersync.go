package auth

import (
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"errors"
	"net/http"
	"os"
	"time"
)

// PowerSyncClaims represents the JWT payload required by PowerSync.
type PowerSyncClaims struct {
	Subject        string `json:"sub"`
	OrganizationID string `json:"organization_id"`
	IssuedAt       int64  `json:"iat"`
	Expires        int64  `json:"exp"`
	Issuer         string `json:"iss"`
	Audience       string `json:"aud"`
}

type powersyncHeader struct {
	Alg string `json:"alg"`
	Typ string `json:"typ"`
	Kid string `json:"kid"`
}

// Ensure the deterministic key generation uses the configured seed if present,
// or falls back to a random key to avoid failure.
func getPowerSyncKeys() (ed25519.PublicKey, ed25519.PrivateKey) {
	seedEnv := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
	if seedEnv != "" {
		seed, err := base64.RawURLEncoding.DecodeString(seedEnv)
		if err == nil && len(seed) == ed25519.SeedSize {
			priv := ed25519.NewKeyFromSeed(seed)
			pub := priv.Public().(ed25519.PublicKey)
			return pub, priv
		}
	}
	// Fallback to random for development if seed is unset or invalid
	pub, priv, _ := ed25519.GenerateKey(rand.Reader)
	return pub, priv
}

var (
	powersyncPubKey, powersyncPrivKey = getPowerSyncKeys()
	powersyncKeyID                    = "ohc-powersync-key-1"
)

// IssuePowerSyncToken generates an EdDSA (Ed25519) signed JWT for PowerSync.
func IssuePowerSyncToken(claims *Claims) (string, error) {
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("missing organization_id in claims")
	}

	now := time.Now().UTC()
	psClaims := PowerSyncClaims{
		Subject:        claims.Subject,
		OrganizationID: claims.OrganizationID,
		IssuedAt:       now.Unix(),
		Expires:        now.Add(1 * time.Hour).Unix(),
		Issuer:         "ohc-server",
		Audience:       "powersync",
	}

	hdr := powersyncHeader{Alg: "EdDSA", Typ: "JWT", Kid: powersyncKeyID}
	hdrBytes, err := json.Marshal(hdr)
	if err != nil {
		return "", err
	}

	payBytes, err := json.Marshal(psClaims)
	if err != nil {
		return "", err
	}

	sigInput := b64url(hdrBytes) + "." + b64url(payBytes)
	sig := ed25519.Sign(powersyncPrivKey, []byte(sigInput))

	return sigInput + "." + b64url(sig), nil
}

// HandlePowerSyncToken issues a PowerSync JWT to an authenticated user.
func HandlePowerSyncToken() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}

		token, err := IssuePowerSyncToken(claims)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{
			"token":      token,
			"expires_at": time.Now().Add(1 * time.Hour).Format(time.RFC3339),
		})
	}
}

// HandlePowerSyncJWKS returns the public keys required by PowerSync to verify JWTs.
func HandlePowerSyncJWKS() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// JWK formatting for Ed25519 public key
		jwk := map[string]interface{}{
			"kty": "OKP",
			"crv": "Ed25519",
			"kid": powersyncKeyID,
			"x":   b64url(powersyncPubKey),
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"keys": []interface{}{jwk},
		})
	}
}
