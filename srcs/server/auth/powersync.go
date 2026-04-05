package auth

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"time"
)

// PowerSyncTokenResponse represents the JWT payload required by PowerSync
type PowerSyncTokenResponse struct {
	Token     string `json:"token"`
	ExpiresAt int64  `json:"expiresAt"`
}

// PowerSyncJWKSResponse represents the JWKS response required by PowerSync
type PowerSyncJWKSResponse struct {
	Keys []JWK `json:"keys"`
}

// JWK represents a JSON Web Key
type JWK struct {
	Kty string `json:"kty"`
	Kid string `json:"kid"`
	Alg string `json:"alg"`
	Use string `json:"use"`
	Crv string `json:"crv"`
	X   string `json:"x"`
}

func getPowerSyncKeys() (ed25519.PublicKey, ed25519.PrivateKey, error) {
	keyBase64 := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
	if keyBase64 == "" {
		return nil, nil, fmt.Errorf("OHC_POWERSYNC_PRIV_KEY not set")
	}

	seed, err := base64.RawURLEncoding.DecodeString(keyBase64)
	if err != nil {
		return nil, nil, fmt.Errorf("failed to decode OHC_POWERSYNC_PRIV_KEY: %w", err)
	}
	if len(seed) != ed25519.SeedSize {
		return nil, nil, fmt.Errorf("invalid OHC_POWERSYNC_PRIV_KEY length: expected %d, got %d", ed25519.SeedSize, len(seed))
	}

	privKey := ed25519.NewKeyFromSeed(seed)
	pubKey := privKey.Public().(ed25519.PublicKey)
	return pubKey, privKey, nil
}

// HandlePowerSyncToken issues a JWT for PowerSync clients
func HandlePowerSyncToken(w http.ResponseWriter, r *http.Request) {
	claims := ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "unauthorized: missing claims", http.StatusUnauthorized)
		return
	}

	_, privKey, err := getPowerSyncKeys()
	if err != nil {
		http.Error(w, fmt.Sprintf("server error: %v", err), http.StatusInternalServerError)
		return
	}

	// For standard EdDSA JWT generation we construct it manually since standard golang-jwt
	// may not support EdDSA without additional imports or setup. PowerSync needs EdDSA.
	// Since we are not adding external libraries outside of what is allowed, we will build a minimal EdDSA JWT.

	header := map[string]string{
		"alg": "EdDSA",
		"typ": "JWT",
		"kid": "powersync-key-1",
	}
	headerBytes, _ := json.Marshal(header)

	now := time.Now()
	expiresAt := now.Add(5 * time.Minute)

	payload := map[string]interface{}{
		"sub": claims.Subject,
		"organization_id": claims.OrganizationID,
		"iss": "ohc-server",
		"iat": now.Unix(),
		"exp": expiresAt.Unix(),
		"aud": "powersync",
	}
	payloadBytes, _ := json.Marshal(payload)

	signingInput := base64.RawURLEncoding.EncodeToString(headerBytes) + "." + base64.RawURLEncoding.EncodeToString(payloadBytes)

	sig := ed25519.Sign(privKey, []byte(signingInput))

	token := signingInput + "." + base64.RawURLEncoding.EncodeToString(sig)

	resp := PowerSyncTokenResponse{
		Token:     token,
		ExpiresAt: expiresAt.Unix(),
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}

// HandlePowerSyncJWKS exposes the public key for PowerSync to verify the JWT
func HandlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	pubKey, _, err := getPowerSyncKeys()
	if err != nil {
		http.Error(w, fmt.Sprintf("server error: %v", err), http.StatusInternalServerError)
		return
	}

	resp := PowerSyncJWKSResponse{
		Keys: []JWK{
			{
				Kty: "OKP",
				Kid: "powersync-key-1",
				Alg: "EdDSA",
				Use: "sig",
				Crv: "Ed25519",
				X:   base64.RawURLEncoding.EncodeToString(pubKey),
			},
		},
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}
