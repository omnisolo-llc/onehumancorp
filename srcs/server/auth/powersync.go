package auth

import (
	"crypto/ed25519"
	"encoding/base64"
	"net/http"
	"time"
	"github.com/golang-jwt/jwt/v5"
)

// PowerSyncAuthHandlers bundles the PowerSync auth HTTP handlers.
type PowerSyncAuthHandlers struct {
	store      *Store
	privateKey ed25519.PrivateKey
	publicKey  ed25519.PublicKey
}

// NewPowerSyncAuthHandlers creates an HTTP handler bundle backed by the given store.
func NewPowerSyncAuthHandlers(store *Store) *PowerSyncAuthHandlers {
	// In a real production deployment this key should be loaded from secure storage.
	// For this exercise we use a deterministic deterministic key from the store secret for horizontal scaling
	// We expand it to 32 bytes to form an ed25519 seed
	secret := store.secret
	seed := make([]byte, ed25519.SeedSize)
	for i := 0; i < ed25519.SeedSize; i++ {
		seed[i] = secret[i%len(secret)]
	}
	priv := ed25519.NewKeyFromSeed(seed)
	pub := priv.Public().(ed25519.PublicKey)

	return &PowerSyncAuthHandlers{
		store:      store,
		privateKey: priv,
		publicKey:  pub,
	}
}

// HandleToken validates credentials and returns a signed JWT for PowerSync. GET /api/auth/powersync/token
func (h *PowerSyncAuthHandlers) HandleToken(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := ClaimsFromContext(r.Context())
	if claims == nil {
		jsonError(w, "not authenticated", http.StatusUnauthorized)
		return
	}

	now := time.Now().UTC()
	exp := now.Add(tokenTTL)

	token := jwt.NewWithClaims(jwt.SigningMethodEdDSA, jwt.MapClaims{
		"sub":             claims.Subject,
		"organization_id": claims.OrganizationID,
		"iat":             now.Unix(),
		"exp":             exp.Unix(),
	})

	tokenString, err := token.SignedString(h.privateKey)
	if err != nil {
		jsonError(w, "failed to issue token", http.StatusInternalServerError)
		return
	}

	writeJSON(w, http.StatusOK, map[string]interface{}{
		"token":     tokenString,
		"expiresAt": exp,
	})
}

// HandleJWKS exposes the public keys needed by PowerSync to verify the token. GET /api/auth/powersync/jwks
func (h *PowerSyncAuthHandlers) HandleJWKS(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	jwks := map[string]interface{}{
		"keys": []map[string]interface{}{
			{
				"kty": "OKP",
				"crv": "Ed25519",
				"kid": "powersync-key-1",
				"x":   base64.RawURLEncoding.EncodeToString(h.publicKey),
				"use": "sig",
			},
		},
	}

	writeJSON(w, http.StatusOK, jwks)
}
