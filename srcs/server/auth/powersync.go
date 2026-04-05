package auth

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"os"
	"strings"
	"time"
)

// powerSyncJWTHeader represents the JWT header for PowerSync.
type powerSyncJWTHeader struct {
	Alg string `json:"alg"`
	Typ string `json:"typ"`
	Kid string `json:"kid"`
}

// powerSyncJWTClaims represents the required claims for PowerSync synchronization.
type powerSyncJWTClaims struct {
	Subject string `json:"sub"`
	Issuer  string `json:"iss"`
	Aud     string `json:"aud"`
	Exp     int64  `json:"exp"`
	Iat     int64  `json:"iat"`

	// Custom claims used by PowerSync sync_rules.yaml
	OrganizationID string `json:"organization_id"`
	Roles          string `json:"roles"`
}

var (
	// powerSyncKeyID is a static Key ID for rotating/identifying keys if needed.
	powerSyncKeyID = "ohc-powersync-key-1"

	// errPowerSyncKeyNotConfigured is returned if OHC_POWERSYNC_PRIV_KEY is missing.
	errPowerSyncKeyNotConfigured = errors.New("OHC_POWERSYNC_PRIV_KEY environment variable is not configured")
)

// getPowerSyncKeys decodes the environment variable to retrieve the Ed25519 keypair.
func getPowerSyncKeys() (ed25519.PublicKey, ed25519.PrivateKey, error) {
	keyBase64 := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
	if keyBase64 == "" {
		return nil, nil, errPowerSyncKeyNotConfigured
	}

	seed, err := base64.RawURLEncoding.DecodeString(keyBase64)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to decode OHC_POWERSYNC_PRIV_KEY: %w", err)
	}
	if len(seed) != ed25519.SeedSize {
		return nil, nil, fmt.Errorf("invalid OHC_POWERSYNC_PRIV_KEY size: expected %d, got %d", ed25519.SeedSize, len(seed))
	}

	priv := ed25519.NewKeyFromSeed(seed)
	pub := priv.Public().(ed25519.PublicKey)
	return pub, priv, nil
}

// HandlePowerSyncToken generates an Ed25519-signed JWT for PowerSync.
// GET /api/auth/powersync/token
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

	_, privKey, err := getPowerSyncKeys()
	if err != nil {
		jsonError(w, "powersync key configuration error", http.StatusInternalServerError)
		return
	}

	now := time.Now().UTC()
	tokenTTL := 24 * time.Hour

	psClaims := powerSyncJWTClaims{
		Subject:        claims.Subject,
		Issuer:         "ohc-backend",
		Aud:            "powersync",
		Exp:            now.Add(tokenTTL).Unix(),
		Iat:            now.Unix(),
		OrganizationID: claims.OrganizationID,
		Roles:          strings.Join(claims.Roles, ","),
	}

	hdr := powerSyncJWTHeader{
		Alg: "EdDSA",
		Typ: "JWT",
		Kid: powerSyncKeyID,
	}

	hdrBytes, err := json.Marshal(hdr)
	if err != nil {
		jsonError(w, "failed to marshal header", http.StatusInternalServerError)
		return
	}
	claimsBytes, err := json.Marshal(psClaims)
	if err != nil {
		jsonError(w, "failed to marshal claims", http.StatusInternalServerError)
		return
	}

	sigInput := base64.RawURLEncoding.EncodeToString(hdrBytes) + "." + base64.RawURLEncoding.EncodeToString(claimsBytes)

	signature := ed25519.Sign(privKey, []byte(sigInput))

	token := sigInput + "." + base64.RawURLEncoding.EncodeToString(signature)

	// PowerSync expects a JSON object with a `token` field.
	// https://docs.powersync.com/installation/authentication-setup/custom
	writeJSON(w, http.StatusOK, map[string]string{
		"token": token,
	})
}

// powerSyncJWK represents a JSON Web Key for the JWKS endpoint.
type powerSyncJWK struct {
	Kty string `json:"kty"`
	Crv string `json:"crv"`
	X   string `json:"x"`
	Kid string `json:"kid"`
	Use string `json:"use"`
}

// HandlePowerSyncJWKS exposes the public Ed25519 key used to sign PowerSync tokens.
// GET /api/auth/powersync/jwks
func (h *Handlers) HandlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	pubKey, _, err := getPowerSyncKeys()
	if err != nil {
		jsonError(w, "powersync key configuration error", http.StatusInternalServerError)
		return
	}

	key := powerSyncJWK{
		Kty: "OKP",
		Crv: "Ed25519",
		X:   base64.RawURLEncoding.EncodeToString(pubKey),
		Kid: powerSyncKeyID,
		Use: "sig",
	}

	writeJSON(w, http.StatusOK, map[string]interface{}{
		"keys": []powerSyncJWK{key},
	})
}
