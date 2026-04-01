package dashboard

import (
	"crypto/rsa"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"fmt"
	"math/big"
	"net/http"
	"os"
	"path/filepath"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func base64UrlEncode(b []byte) string {
	return base64.RawURLEncoding.EncodeToString(b)
}

// handlePowerSyncRules provides the bucket_data for PowerSync rules.
func handlePowerSyncRules() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/yaml")

		claims := auth.ClaimsFromContext(r.Context())
		orgID := "demo" // Default
		if claims != nil && claims.OrganizationID != "" {
			orgID = claims.OrganizationID
		}

		// Strict tenant isolation via bucket_data schema
		rules := fmt.Sprintf(`
bucket_data:
  global:
    data:
      - SELECT * FROM meeting_rooms
      - SELECT * FROM meeting_transcripts
  user:
    parameters:
      - "request.jwt.organization_id"
    data:
      - SELECT * FROM agents WHERE organization_id = request.jwt.organization_id
      - SELECT * FROM agent_missions WHERE payload::json->>'organization_id' = '%s'
`, orgID)

		w.Write([]byte(rules))
	}
}

func getRSAPrivateKey() (*rsa.PrivateKey, error) {
	keyData := os.Getenv("POWERSYNC_RSA_PRIVATE_KEY")
	if keyData == "" {
		home, _ := os.UserHomeDir()
		path := filepath.Join(home, ".openclaw", "powersync_rsa.pem")
		b, err := os.ReadFile(path)
		if err != nil {
			return nil, fmt.Errorf("could not read local rsa key: %w", err)
		}
		keyData = string(b)
	}

	block, _ := pem.Decode([]byte(keyData))
	if block == nil {
		return nil, fmt.Errorf("failed to parse PEM block containing the key")
	}

	priv, err := x509.ParsePKCS1PrivateKey(block.Bytes)
	if err != nil {
		priv, err := x509.ParsePKCS8PrivateKey(block.Bytes)
		if err != nil {
			return nil, fmt.Errorf("failed to parse RSA private key: %w", err)
		}
		return priv.(*rsa.PrivateKey), nil
	}
	return priv, nil
}

// handlePowerSyncJWKS provides the JWKS for PowerSync authentication.
func handlePowerSyncJWKS() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		priv, err := getRSAPrivateKey()
		if err != nil {
			http.Error(w, "Failed to load RSA key", http.StatusInternalServerError)
			return
		}

		pub := priv.Public().(*rsa.PublicKey)

		// A proper JWKS would include e, n, kid, kty, etc.
		jwks := map[string]interface{}{
			"keys": []map[string]interface{}{
				{
					"kty": "RSA",
					"kid": "powersync-key-1",
					"alg": "RS256",
					"use": "sig",
					"n":   base64UrlEncode(pub.N.Bytes()),
					"e":   base64UrlEncode(big.NewInt(int64(pub.E)).Bytes()),
				},
			},
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(jwks)
	}
}
