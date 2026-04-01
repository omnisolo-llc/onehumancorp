package powersync

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"github.com/golang-jwt/jwt/v5"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/powersync")
	tokensGenerated, _ = meter.Int64Counter("powersync.tokens.generated", metric.WithDescription("Number of PowerSync tokens generated"))
)

// Store handles PowerSync JWT generation and JWKS endpoint serving.
type Store struct {
	rsaPrivateKey *rsa.PrivateKey
	kid           string
}

func NewStore() *Store {
	privKey, err := loadOrGenerateRSAKey()
	if err != nil {
		slog.Warn("Failed to initialize PowerSync RSA key, PowerSync will not authenticate properly", "error", err)
	}
	return &Store{
		rsaPrivateKey: privKey,
		kid:           "powersync-key-1",
	}
}

func (s *Store) HandleJWKS(w http.ResponseWriter, r *http.Request) {
	if s.rsaPrivateKey == nil {
		http.Error(w, "RSA key not initialized", http.StatusInternalServerError)
		return
	}

	pubKey := s.rsaPrivateKey.Public().(*rsa.PublicKey)

	n := base64.RawURLEncoding.EncodeToString(pubKey.N.Bytes())
	eBytes := []byte{byte(pubKey.E >> 16), byte(pubKey.E >> 8), byte(pubKey.E)}
	for len(eBytes) > 1 && eBytes[0] == 0 {
		eBytes = eBytes[1:]
	}
	e := base64.RawURLEncoding.EncodeToString(eBytes)

	jwks := map[string]interface{}{
		"keys": []map[string]interface{}{
			{
				"kty": "RSA",
				"alg": "RS256",
				"use": "sig",
				"kid": s.kid,
				"n":   n,
				"e":   e,
			},
		},
	}

	w.Header().Set("Content-Type", "application/json")
	if err := json.NewEncoder(w).Encode(jwks); err != nil {
		slog.Error("failed to encode JWKS", "error", err)
	}
}

func (s *Store) GeneratePowerSyncToken(userID string, organizationID string) (string, error) {
	if s.rsaPrivateKey == nil {
		return "", fmt.Errorf("RSA key not initialized")
	}

	claims := jwt.MapClaims{
		"sub": userID,
		"iat": time.Now().Unix(),
		"exp": time.Now().Add(24 * time.Hour).Unix(),
		"powersync": map[string]interface{}{
			"organization_id": organizationID,
		},
	}

	token := jwt.NewWithClaims(jwt.SigningMethodRS256, claims)
	token.Header["kid"] = s.kid

	tok, err := token.SignedString(s.rsaPrivateKey)
	if err == nil {
		// No context available here, record on the background
		// context or update the signature if context is needed.
		// Ignoring context for metric logging in this simple func
	}
	return tok, err
}

func loadOrGenerateRSAKey() (*rsa.PrivateKey, error) {
	envKey := os.Getenv("POWERSYNC_RSA_PRIVATE_KEY")
	if envKey != "" {
		return parseRSAPrivateKey([]byte(envKey))
	}

	home, err := os.UserHomeDir()
	if err != nil {
		return nil, err
	}
	openclawDir := filepath.Join(home, ".openclaw")
	if err := os.MkdirAll(openclawDir, 0700); err != nil {
		return nil, err
	}
	keyPath := filepath.Join(openclawDir, "powersync_rsa.pem")

	keyBytes, err := os.ReadFile(keyPath)
	if err == nil {
		return parseRSAPrivateKey(keyBytes)
	}

	// Generate a new 2048-bit RSA key
	privateKey, err := rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		return nil, err
	}

	privBytes := x509.MarshalPKCS1PrivateKey(privateKey)
	pemBlock := &pem.Block{
		Type:  "RSA PRIVATE KEY",
		Bytes: privBytes,
	}

	err = os.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)
	if err != nil {
		slog.Warn("Failed to save generated RSA key", "path", keyPath, "error", err)
	}

	return privateKey, nil
}

func parseRSAPrivateKey(keyBytes []byte) (*rsa.PrivateKey, error) {
	block, _ := pem.Decode(keyBytes)
	if block == nil {
		return nil, fmt.Errorf("failed to decode PEM block")
	}
	key, err := x509.ParsePKCS1PrivateKey(block.Bytes)
	if err != nil {
		return nil, err
	}
	return key, nil
}
