package auth

import (
	"encoding/base64"
	"encoding/json"
	"net/http"
)

// HandlePowerSyncJWKS returns the public key in JWKS format for PowerSync to verify JWTs.
func (h *Handlers) HandlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	if err := initPowerSyncKeys(); err != nil || !cachedPowerSyncKeyInit {
		http.Error(w, "internal server error: failed to initialize keys", http.StatusInternalServerError)
		return
	}

	// Build JWKS response
	// Note: Ed25519 keys use 'OKP' as kty and 'Ed25519' as crv
	jwks := map[string]interface{}{
		"keys": []map[string]interface{}{
			{
				"kty": "OKP",
				"use": "sig",
				"crv": "Ed25519",
				"kid": "powersync-key-1",
				"x":   base64.RawURLEncoding.EncodeToString(cachedPowerSyncPubKey),
			},
		},
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(jwks); err != nil {
		// Log error if needed
	}
}
