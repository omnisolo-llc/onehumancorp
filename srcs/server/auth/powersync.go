package auth

import (
	"crypto/ed25519"
	"crypto/rand"
	"encoding/json"
	"net/http"
	"os"
	"time"
	"encoding/base64"
	"sync"
)

// In a real app we'd load this from an env var, we'll auto-generate one for this demo if not provided.
var (
	powerSyncKeyMu sync.Mutex
	powerSyncPriv ed25519.PrivateKey
	powerSyncPub  ed25519.PublicKey
)

func getPowerSyncKeys() (ed25519.PrivateKey, ed25519.PublicKey) {
	powerSyncKeyMu.Lock()
	defer powerSyncKeyMu.Unlock()

	if powerSyncPriv == nil {
		seedHex := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
		if seedHex != "" {
			// Try to parse the seeded key
			seed, err := base64.StdEncoding.DecodeString(seedHex)
			if err == nil && len(seed) == ed25519.SeedSize {
				powerSyncPriv = ed25519.NewKeyFromSeed(seed)
				powerSyncPub = powerSyncPriv.Public().(ed25519.PublicKey)
				return powerSyncPriv, powerSyncPub
			}
		}

		// Fallback to random
		pub, priv, err := ed25519.GenerateKey(rand.Reader)
		if err != nil {
			panic("failed to generate ed25519 key for powersync: " + err.Error())
		}
		powerSyncPriv = priv
		powerSyncPub = pub
	}

	return powerSyncPriv, powerSyncPub
}

// PowerSyncJWKSHandler exposes the public key for PowerSync to verify our JWTs.
func PowerSyncJWKSHandler() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		_, pub := getPowerSyncKeys()

		// Construct JWKS response for EdDSA
		// x is base64url encoded public key
		x := base64.RawURLEncoding.EncodeToString(pub)

		jwks := map[string]interface{}{
			"keys": []map[string]string{
				{
					"kty": "OKP",
					"crv": "Ed25519",
					"use": "sig",
					"kid": "powersync-key-1",
					"x":   x,
				},
			},
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(jwks)
	}
}

// PowerSyncTokenHandler generates a PowerSync-compatible JWT.
func PowerSyncTokenHandler(store *Store) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := ClaimsFromContext(r.Context())
		if claims == nil {
			jsonError(w, "unauthorized", http.StatusUnauthorized)
			return
		}

		priv, _ := getPowerSyncKeys()

		// PowerSync requires specific claims
		now := time.Now()
		exp := now.Add(24 * time.Hour).Unix()

		powerSyncClaims := map[string]interface{}{
			"iss": "ohc-backend",
			"sub": claims.Subject,
			"aud": "powersync",
			"iat": now.Unix(),
			"exp": exp,
			"organization_id": claims.OrganizationID,
		}

		hdr := map[string]string{
			"alg": "EdDSA",
			"typ": "JWT",
			"kid": "powersync-key-1",
		}

		hdrBytes, _ := json.Marshal(hdr)
		claimsBytes, _ := json.Marshal(powerSyncClaims)

		sigInput := base64.RawURLEncoding.EncodeToString(hdrBytes) + "." + base64.RawURLEncoding.EncodeToString(claimsBytes)

		sig := ed25519.Sign(priv, []byte(sigInput))

		token := sigInput + "." + base64.RawURLEncoding.EncodeToString(sig)

		response := map[string]interface{}{
			"token": token,
			"expires_at": exp,
			"power_sync_url": os.Getenv("OHC_POWERSYNC_URL"), // Or default if not set
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(response)
	}
}
