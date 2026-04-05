package auth

import (
	"crypto"
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"os"
	"sync"
	"time"
)

var (
	powersyncKeyOnce sync.Once
	powersyncPrivKey crypto.PrivateKey
	powersyncPubKey  crypto.PublicKey
)

// initPowerSyncKey initializes the Ed25519 keypair for PowerSync tokens
// based on an environment variable or randomly generates one if missing.
func initPowerSyncKey() {
	powersyncKeyOnce.Do(func() {
		seedHex := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
		if seedHex != "" {
			seed, err := base64.RawURLEncoding.DecodeString(seedHex)
			if err == nil && len(seed) == ed25519.SeedSize {
				powersyncPrivKey = ed25519.NewKeyFromSeed(seed)
				powersyncPubKey = powersyncPrivKey.(ed25519.PrivateKey).Public()
				return
			}
		}

		// Fallback to random if not set
		pub, priv, err := ed25519.GenerateKey(rand.Reader)
		if err == nil {
			powersyncPrivKey = priv
			powersyncPubKey = pub
		}
	})
}

// PowerSyncJWKSHandler exposes the public key for PowerSync to verify tokens.
// Accessible at /api/auth/powersync/jwks without authentication.
func (h *Handlers) PowerSyncJWKSHandler(w http.ResponseWriter, r *http.Request) {
	initPowerSyncKey()

	if powersyncPubKey == nil {
		jsonError(w, "PowerSync key not configured", http.StatusInternalServerError)
		return
	}

	pub, ok := powersyncPubKey.(ed25519.PublicKey)
	if !ok {
		jsonError(w, "Invalid key type", http.StatusInternalServerError)
		return
	}

	x := base64.RawURLEncoding.EncodeToString(pub)

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

	writeJSON(w, http.StatusOK, jwks)
}

// PowerSyncTokenHandler generates a short-lived JWT for PowerSync clients.
// Requires standard authentication to invoke.
func (h *Handlers) PowerSyncTokenHandler(w http.ResponseWriter, r *http.Request) {
	initPowerSyncKey()

	if powersyncPrivKey == nil {
		jsonError(w, "PowerSync key not configured", http.StatusInternalServerError)
		return
	}

	claims := ClaimsFromContext(r.Context())
	if claims == nil {
		jsonError(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	now := time.Now().UTC()

	// Create token payload. PowerSync needs `sub` and any parameters used in sync rules.
	payload := map[string]interface{}{
		"sub": claims.Subject,
		"iat": now.Unix(),
		"exp": now.Add(1 * time.Hour).Unix(),
		"jti": generateID(),
		"organization_id": claims.OrganizationID,
	}

	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		jsonError(w, "Failed to encode token payload", http.StatusInternalServerError)
		return
	}

	header := map[string]interface{}{
		"alg": "EdDSA",
		"typ": "JWT",
		"kid": "powersync-key-1",
	}

	headerBytes, err := json.Marshal(header)
	if err != nil {
		jsonError(w, "Failed to encode token header", http.StatusInternalServerError)
		return
	}

	unsignedToken := base64.RawURLEncoding.EncodeToString(headerBytes) + "." + base64.RawURLEncoding.EncodeToString(payloadBytes)

	priv, ok := powersyncPrivKey.(ed25519.PrivateKey)
	if !ok {
		jsonError(w, "Invalid key type", http.StatusInternalServerError)
		return
	}

	signature := ed25519.Sign(priv, []byte(unsignedToken))
	signedToken := unsignedToken + "." + base64.RawURLEncoding.EncodeToString(signature)

	response := map[string]string{
		"token": signedToken,
		"endpoint": os.Getenv("POWERSYNC_URL"), // Provide the PowerSync URL to clients
	}

	writeJSON(w, http.StatusOK, response)
}

// For testability
func PowerSyncTokenHandlerForTest(w http.ResponseWriter, r *http.Request) {
	h := &Handlers{}
	h.PowerSyncTokenHandler(w, r)
}
