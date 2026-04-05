package auth

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"log/slog"
	"net/http"
	"os"
	"sync"
	"time"
)

var (
	powersyncPrivKey ed25519.PrivateKey
	powersyncPubKey  ed25519.PublicKey
	powersyncKeyOnce sync.Once
)

func initPowerSyncKeys() {
	powersyncKeyOnce.Do(func() {
		privHex := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
		if privHex != "" {
			seed, err := hex.DecodeString(privHex)
			if err == nil && len(seed) == ed25519.SeedSize {
				powersyncPrivKey = ed25519.NewKeyFromSeed(seed)
				powersyncPubKey = powersyncPrivKey.Public().(ed25519.PublicKey)
				return
			}
			slog.Error("failed to decode OHC_POWERSYNC_PRIV_KEY", "error", err)
		}
		// Deterministic fallback using a hardcoded development seed to avoid multi-pod split-brain
		slog.Warn("OHC_POWERSYNC_PRIV_KEY not set or invalid, falling back to deterministic dev key")
		devSeed := make([]byte, ed25519.SeedSize)
		for i := range devSeed {
			devSeed[i] = byte(i) // Simple deterministic seed
		}
		powersyncPrivKey = ed25519.NewKeyFromSeed(devSeed)
		powersyncPubKey = powersyncPrivKey.Public().(ed25519.PublicKey)
	})
}


type powersyncJWK struct {
	Kty string `json:"kty"`
	Crv string `json:"crv"`
	X   string `json:"x"`
	Use string `json:"use"`
	Kid string `json:"kid"`
}

type powersyncJWKSResponse struct {
	Keys []powersyncJWK `json:"keys"`
}

// HandlePowerSyncJWKS serves the public key for PowerSync JWT verification.
// GET /api/auth/powersync/jwks
func (h *Handlers) HandlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	initPowerSyncKeys()

	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	key := powersyncJWK{
		Kty: "OKP",
		Crv: "Ed25519",
		X:   base64.RawURLEncoding.EncodeToString(powersyncPubKey),
		Use: "sig",
		Kid: "powersync-key-1",
	}

	writeJSON(w, http.StatusOK, powersyncJWKSResponse{Keys: []powersyncJWK{key}})
}

// HandlePowerSyncToken generates a short-lived JWT for PowerSync clients.
// GET /api/auth/powersync/token
func (h *Handlers) HandlePowerSyncToken(w http.ResponseWriter, r *http.Request) {
	initPowerSyncKeys()

	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := ClaimsFromContext(r.Context())
	if claims == nil {
		jsonError(w, "unauthorized: missing claims", http.StatusUnauthorized)
		return
	}

	now := time.Now().UTC()
	exp := now.Add(5 * time.Minute).Unix()

	// Build the JWT manually using Ed25519 (EdDSA)
	header := map[string]string{
		"alg": "EdDSA",
		"typ": "JWT",
		"kid": "powersync-key-1",
	}
	headerBytes, _ := json.Marshal(header)
	encodedHeader := base64.RawURLEncoding.EncodeToString(headerBytes)

	payload := map[string]interface{}{
		"sub":             claims.Subject,
		"organization_id": claims.OrganizationID,
		"iat":             now.Unix(),
		"exp":             exp,
		"aud":             "powersync",
		"iss":             "ohc-server",
	}
	payloadBytes, _ := json.Marshal(payload)
	encodedPayload := base64.RawURLEncoding.EncodeToString(payloadBytes)

	signingInput := encodedHeader + "." + encodedPayload
	signature := ed25519.Sign(powersyncPrivKey, []byte(signingInput))
	encodedSignature := base64.RawURLEncoding.EncodeToString(signature)

	tokenString := signingInput + "." + encodedSignature

	writeJSON(w, http.StatusOK, map[string]interface{}{
		"token":      tokenString,
		"expires_at": exp,
	})
}
