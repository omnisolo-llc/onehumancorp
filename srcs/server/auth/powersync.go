package auth

import (
	"crypto"
	"crypto/ed25519"
	"crypto/rand"
	"encoding/json"
	"net/http"
	"time"
)

type PowerSyncKeypair struct {
	PrivateKey ed25519.PrivateKey
	PublicKey  ed25519.PublicKey
	KeyID      string
}

func GeneratePowerSyncKeypair() (*PowerSyncKeypair, error) {
	pub, priv, err := ed25519.GenerateKey(rand.Reader)
	if err != nil {
		return nil, err
	}
	return &PowerSyncKeypair{
		PrivateKey: priv,
		PublicKey:  pub,
		KeyID:      "powersync-key-1",
	}, nil
}

func IssuePowerSyncToken(claims *Claims, keypair *PowerSyncKeypair) (string, error) {
	now := time.Now().UTC()
	exp := now.Add(24 * time.Hour).Unix()

	hdr := map[string]string{
		"alg": "EdDSA",
		"typ": "JWT",
		"kid": keypair.KeyID,
	}

	payload := map[string]interface{}{
		"sub": claims.Subject,
		"iss": "onehumancorp",
		"iat": now.Unix(),
		"exp": exp,
		"organization_id": claims.OrganizationID,
	}

	hdrJSON, err := json.Marshal(hdr)
	if err != nil {
		return "", err
	}
	payJSON, err := json.Marshal(payload)
	if err != nil {
		return "", err
	}

	sigInput := b64url(hdrJSON) + "." + b64url(payJSON)

	sig, err := keypair.PrivateKey.Sign(rand.Reader, []byte(sigInput), crypto.Hash(0))
	if err != nil {
		return "", err
	}

	return sigInput + "." + b64url(sig), nil
}

func PowerSyncTokenHandler(s *Store, keypair *PowerSyncKeypair) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		claims := ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}

		token, err := IssuePowerSyncToken(claims, keypair)
		if err != nil {
			http.Error(w, "failed to generate token", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]string{
			"token": token,
			"expires_at": time.Now().Add(24 * time.Hour).Format(time.RFC3339),
		})
	}
}

func PowerSyncJWKSHandler(keypair *PowerSyncKeypair) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"keys": []map[string]string{
				{
					"kty": "OKP",
					"crv": "Ed25519",
					"kid": keypair.KeyID,
					"x":   b64url(keypair.PublicKey),
					"alg": "EdDSA",
					"use": "sig",
				},
			},
		})
	}
}
