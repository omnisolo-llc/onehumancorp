package auth

import (
	"crypto/ed25519"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"sync"
	"time"
)

import (
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	powersyncPrivateKey ed25519.PrivateKey
	powersyncPublicKey  ed25519.PublicKey
	powersyncKeyOnce    sync.Once

	authMeter = otel.Meter("github.com/onehumancorp/mono/srcs/server/auth")
	powerSyncTokenRequests, _ = authMeter.Int64Counter(
		"auth.powersync.token.requests",
		metric.WithDescription("Number of PowerSync token requests"),
	)
	powerSyncJWKSRequests, _ = authMeter.Int64Counter(
		"auth.powersync.jwks.requests",
		metric.WithDescription("Number of PowerSync JWKS requests"),
	)
)

func initPowerSyncKeys() {
	powersyncKeyOnce.Do(func() {
		privKeyB64 := os.Getenv("POWERSYNC_PRIVATE_KEY_B64")
		pubKeyB64 := os.Getenv("POWERSYNC_PUBLIC_KEY_B64")

		if privKeyB64 != "" && pubKeyB64 != "" {
			privBytes, err1 := base64.StdEncoding.DecodeString(privKeyB64)
			pubBytes, err2 := base64.StdEncoding.DecodeString(pubKeyB64)

			if err1 == nil && err2 == nil && len(privBytes) == ed25519.PrivateKeySize && len(pubBytes) == ed25519.PublicKeySize {
				powersyncPrivateKey = ed25519.PrivateKey(privBytes)
				powersyncPublicKey = ed25519.PublicKey(pubBytes)
				slog.Info("Loaded PowerSync keys from environment")
				return
			}
			slog.Warn("Failed to decode provided PowerSync keys, falling back to generation")
		}

		slog.Warn("POWERSYNC_PRIVATE_KEY_B64 not set, generating ephemeral in-memory keys for PowerSync. This breaks in multi-pod deployments.")
		pub, priv, err := ed25519.GenerateKey(rand.Reader)
		if err != nil {
			panic(fmt.Sprintf("failed to generate PowerSync ed25519 keys: %v", err))
		}
		powersyncPublicKey = pub
		powersyncPrivateKey = priv
	})
}

// PowerSyncTokenHandler returns a JWT token for the PowerSync client to authenticate.
func PowerSyncTokenHandler() http.HandlerFunc {
	initPowerSyncKeys()

	return func(w http.ResponseWriter, r *http.Request) {
		if powerSyncTokenRequests != nil {
			powerSyncTokenRequests.Add(r.Context(), 1)
		}

		claims := ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "unauthorized: missing claims", http.StatusUnauthorized)
			return
		}

		// Ensure we only issue tokens to valid organization users
		orgID := claims.OrganizationID
		if orgID == "" {
			orgID = "default"
		}

		now := time.Now()
		// PowerSync requires specific claims
		// See: https://docs.powersync.com/usage/installation/authentication-setup/custom
		header := map[string]interface{}{
			"alg": "EdDSA",
			"typ": "JWT",
			"kid": "powersync-key-1",
		}

		payload := map[string]interface{}{
			"sub":             claims.Subject,
			"iat":             now.Unix(),
			"exp":             now.Add(24 * time.Hour).Unix(),
			"organization_id": orgID,
			"aud":             "powersync",
			"iss":             "ohc-auth",
		}

		headerJSON, _ := json.Marshal(header)
		payloadJSON, _ := json.Marshal(payload)

		headerB64 := base64.RawURLEncoding.EncodeToString(headerJSON)
		payloadB64 := base64.RawURLEncoding.EncodeToString(payloadJSON)

		unsignedToken := headerB64 + "." + payloadB64
		signature := ed25519.Sign(powersyncPrivateKey, []byte(unsignedToken))
		signatureB64 := base64.RawURLEncoding.EncodeToString(signature)

		tokenString := unsignedToken + "." + signatureB64

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{
			"token":      tokenString,
			"expires_at": now.Add(24 * time.Hour).Format(time.RFC3339),
		})
	}
}

// PowerSyncJWKSHandler returns the public keys to verify the PowerSync JWTs.
func PowerSyncJWKSHandler() http.HandlerFunc {
	initPowerSyncKeys()

	return func(w http.ResponseWriter, r *http.Request) {
		if powerSyncJWKSRequests != nil {
			powerSyncJWKSRequests.Add(r.Context(), 1)
		}

		w.Header().Set("Content-Type", "application/json")

		// Encode ed25519 public key as JWK
		// "kty": "OKP", "crv": "Ed25519"
		// x is base64url encoded public key

		// The PowerSync documentation states that it uses standard JWKS format.
		// For simplicity, we just provide the basic structure needed.

		encodedX := base64.RawURLEncoding.EncodeToString(powersyncPublicKey)

		jwks := map[string]interface{}{
			"keys": []map[string]interface{}{
				{
					"kty": "OKP",
					"crv": "Ed25519",
					"kid": "powersync-key-1",
					"x":   encodedX,
					"use": "sig",
				},
			},
		}

		json.NewEncoder(w).Encode(jwks)
	}
}
