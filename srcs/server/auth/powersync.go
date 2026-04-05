package auth

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"os"
	"time"
)

// PowerSyncTokenResponse is the response format expected by the PowerSync client SDK
type PowerSyncTokenResponse struct {
	Token     string `json:"token"`
	ExpiresAt string `json:"expiresAt"`
}

// JWKS is the JSON Web Key Set format
type JWKS struct {
	Keys []JWK `json:"keys"`
}

type JWK struct {
	Kty string `json:"kty"`
	Kid string `json:"kid"`
	Crv string `json:"crv"`
	X   string `json:"x"`
	Use string `json:"use"`
	Alg string `json:"alg"`
}

// HandlePowerSyncToken generates a short-lived JWT for PowerSync clients.  GET /api/auth/powersync/token
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

	privKeyEnv := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
	if privKeyEnv == "" {
		jsonError(w, "server configuration error: missing OHC_POWERSYNC_PRIV_KEY", http.StatusInternalServerError)
		return
	}

	seed, err := base64.RawURLEncoding.DecodeString(privKeyEnv)
	if err != nil || len(seed) != ed25519.SeedSize {
		jsonError(w, "server configuration error: invalid OHC_POWERSYNC_PRIV_KEY format", http.StatusInternalServerError)
		return
	}

	privKey := ed25519.NewKeyFromSeed(seed)

	now := time.Now().UTC()
	exp := now.Add(5 * time.Minute) // Short-lived token

	// Standard JWT claims + custom PowerSync claims
	psClaims := map[string]interface{}{
		"iss": "ohc-backend",
		"sub": claims.Subject,
		"iat": now.Unix(),
		"exp": exp.Unix(),
		"organization_id": claims.OrganizationID,
	}

	hdr, err := json.Marshal(map[string]string{"alg": "EdDSA", "typ": "JWT", "kid": "powersync-key-1"})
	if err != nil {
		jsonError(w, "failed to generate token", http.StatusInternalServerError)
		return
	}

	pay, err := json.Marshal(psClaims)
	if err != nil {
		jsonError(w, "failed to generate token", http.StatusInternalServerError)
		return
	}

	sigInput := b64url(hdr) + "." + b64url(pay)
	sig := ed25519.Sign(privKey, []byte(sigInput))

	token := sigInput + "." + b64url(sig)

	writeJSON(w, http.StatusOK, PowerSyncTokenResponse{
		Token:     token,
		ExpiresAt: exp.Format(time.RFC3339),
	})
}

// HandlePowerSyncJWKS returns the public key in JWKS format.  GET /api/auth/powersync/jwks
func (h *Handlers) HandlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	privKeyEnv := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
	if privKeyEnv == "" {
		jsonError(w, "server configuration error: missing OHC_POWERSYNC_PRIV_KEY", http.StatusInternalServerError)
		return
	}

	seed, err := base64.RawURLEncoding.DecodeString(privKeyEnv)
	if err != nil || len(seed) != ed25519.SeedSize {
		jsonError(w, "server configuration error: invalid OHC_POWERSYNC_PRIV_KEY format", http.StatusInternalServerError)
		return
	}

	privKey := ed25519.NewKeyFromSeed(seed)
	pubKey := privKey.Public().(ed25519.PublicKey)

	jwks := JWKS{
		Keys: []JWK{
			{
				Kty: "OKP",
				Kid: "powersync-key-1",
				Crv: "Ed25519",
				X:   b64url(pubKey),
				Use: "sig",
				Alg: "EdDSA",
			},
		},
	}

	writeJSON(w, http.StatusOK, jwks)
}
