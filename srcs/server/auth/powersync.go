package auth

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"

	"net/http"
	"os"
	"time"
)

// HandlePowerSyncToken generates a short-lived JWT for PowerSync clients.
// It embeds the user's organization_id for multi-tenant isolation.
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

	// Retrieve deterministic private key seed from environment
	seedBase64 := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
	if seedBase64 == "" {
		// Provide a fallback for local testing
		seedBase64 = base64.RawURLEncoding.EncodeToString([]byte("default-powersync-key-32-bytes-long"))
	}

	seed, err := base64.RawURLEncoding.DecodeString(seedBase64)
	if err != nil || len(seed) < ed25519.SeedSize {
		jsonError(w, "invalid server configuration for powersync", http.StatusInternalServerError)
		return
	}

	// The first 32 bytes of the seed are used
	privKey := ed25519.NewKeyFromSeed(seed[:ed25519.SeedSize])

	now := time.Now().UTC()

	// Prepare the PowerSync token payload
	// PowerSync expects a custom token containing tenant parameters
	payload := map[string]interface{}{
		"iss": "ohc-powersync-issuer",
		"sub": claims.Subject,
		"aud": "powersync",
		"exp": now.Add(5 * time.Minute).Unix(),
		"iat": now.Unix(),
		"parameters": map[string]string{
			"organization_id": claims.OrganizationID,
		},
	}

	header := map[string]interface{}{
		"alg": "EdDSA",
		"typ": "JWT",
		"kid": "powersync-key-1",
	}

	headerBytes, _ := json.Marshal(header)
	payloadBytes, _ := json.Marshal(payload)

	headerEnc := base64.RawURLEncoding.EncodeToString(headerBytes)
	payloadEnc := base64.RawURLEncoding.EncodeToString(payloadBytes)

	signingInput := headerEnc + "." + payloadEnc
	sig := ed25519.Sign(privKey, []byte(signingInput))
	sigEnc := base64.RawURLEncoding.EncodeToString(sig)

	tokenStr := signingInput + "." + sigEnc

	resp := map[string]string{
		"token": tokenStr,
		"expires_at": now.Add(5 * time.Minute).Format(time.RFC3339),
	}

	writeJSON(w, http.StatusOK, resp)
}

// HandlePowerSyncJWKS returns the public key corresponding to the PowerSync token signing key.
func (h *Handlers) HandlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	seedBase64 := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
	if seedBase64 == "" {
		seedBase64 = base64.RawURLEncoding.EncodeToString([]byte("default-powersync-key-32-bytes-long"))
	}

	seed, err := base64.RawURLEncoding.DecodeString(seedBase64)
	if err != nil || len(seed) < ed25519.SeedSize {
		jsonError(w, "invalid server configuration for powersync", http.StatusInternalServerError)
		return
	}

	privKey := ed25519.NewKeyFromSeed(seed[:ed25519.SeedSize])
	pubKey := privKey.Public().(ed25519.PublicKey)

	// Build the JWK entry
	jwk := map[string]interface{}{
		"kty": "OKP",
		"crv": "Ed25519",
		"x":   base64.RawURLEncoding.EncodeToString(pubKey),
		"use": "sig",
		"alg": "EdDSA",
		"kid": "powersync-key-1",
	}

	resp := map[string]interface{}{
		"keys": []map[string]interface{}{jwk},
	}

	writeJSON(w, http.StatusOK, resp)
}
