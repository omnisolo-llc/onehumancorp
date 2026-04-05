package auth

import (
	"crypto/ed25519"
	"crypto/rand"
	"encoding/json"
	"net/http"
	"time"
)

// In a real application, keys should be loaded from secure storage.
// For this implemention, we generate a persistent Ed25519 keypair for PowerSync on startup.
var (
	powersyncPubKey  ed25519.PublicKey
	powersyncPrivKey ed25519.PrivateKey
)

func init() {
	pub, priv, err := ed25519.GenerateKey(rand.Reader)
	if err != nil {
		panic("failed to generate powersync keys: " + err.Error())
	}
	powersyncPubKey = pub
	powersyncPrivKey = priv
}

// HandlePowerSyncToken issues a JWT compatible with PowerSync's expected format.
// GET /api/auth/powersync/token
func (h *Handlers) HandlePowerSyncToken(w http.ResponseWriter, r *http.Request) {
	claims := ClaimsFromContext(r.Context())
	if claims == nil {
		jsonError(w, "not authenticated", http.StatusUnauthorized)
		return
	}

	// PowerSync standard claims plus custom fields for sync rules
	// For OHC, we must include the organization_id.
	now := time.Now().UTC()
	psClaims := map[string]interface{}{
		"iss": "ohc-server",
		"sub": claims.Subject,
		"iat": now.Unix(),
		"exp": now.Add(1 * time.Hour).Unix(),
		"parameters": map[string]interface{}{
			"organization_id": claims.OrganizationID,
			"user_id":         claims.Subject,
		},
	}

	hdr := map[string]string{"alg": "EdDSA", "typ": "JWT", "kid": "powersync-key-1"}
	hdrBytes, _ := json.Marshal(hdr)
	payBytes, _ := json.Marshal(psClaims)

	sigInput := b64url(hdrBytes) + "." + b64url(payBytes)
	sig := ed25519.Sign(powersyncPrivKey, []byte(sigInput))

	token := sigInput + "." + b64url(sig)

	writeJSON(w, http.StatusOK, map[string]interface{}{
		"token": token,
		"expires_at": now.Add(1 * time.Hour).Format(time.RFC3339),
	})
}

// HandlePowerSyncJWKS returns the JWKS for PowerSync to verify the token.
// GET /api/auth/powersync/jwks
func (h *Handlers) HandlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	// Ed25519 public key parameters for JWKS:
	// "kty": "OKP"
	// "crv": "Ed25519"
	// "x": base64url(publicKey)
	// "kid": "powersync-key-1"

	jwk := map[string]interface{}{
		"kty": "OKP",
		"crv": "Ed25519",
		"x":   b64url(powersyncPubKey),
		"kid": "powersync-key-1",
		"use": "sig",
	}

	jwks := map[string]interface{}{
		"keys": []map[string]interface{}{jwk},
	}

	writeJSON(w, http.StatusOK, jwks)
}
