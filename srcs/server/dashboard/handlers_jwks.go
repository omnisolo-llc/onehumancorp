package dashboard

import (
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"fmt"
	"io/ioutil"
	"log/slog"
	"net/http"
	"os"
	"path/filepath"
	"sync"
)

var (
	rsaKeyOnce   sync.Once
	rsaPublicKey *rsa.PublicKey
)

// ensureRSAKey generates or loads an RSA private key for signing JWKS.
// Based on memory guidelines, uses POWERSYNC_RSA_PRIVATE_KEY env or local file storage.
func ensureRSAKey() (*rsa.PublicKey, error) {
	var err error
	rsaKeyOnce.Do(func() {
		privKeyEnv := os.Getenv("POWERSYNC_RSA_PRIVATE_KEY")
		var privKey *rsa.PrivateKey
		if privKeyEnv != "" {
			block, _ := pem.Decode([]byte(privKeyEnv))
			if block != nil {
				privKey, err = x509.ParsePKCS1PrivateKey(block.Bytes)
			} else {
				err = fmt.Errorf("failed to parse POWERSYNC_RSA_PRIVATE_KEY")
			}
		} else {
			// Local fallback for standalone mode
			homeDir, _ := os.UserHomeDir()
			openclawDir := filepath.Join(homeDir, ".openclaw")

			// Try .agent-task for standalone first
			keyPath := ".agent-task/powersync_rsa.pem"
			if os.Getenv("OHC_STANDALONE") != "true" {
				keyPath = filepath.Join(openclawDir, "powersync_rsa.pem")
			}

			if _, statErr := os.Stat(keyPath); os.IsNotExist(statErr) {
				privKey, err = rsa.GenerateKey(rand.Reader, 2048)
				if err == nil {
					if os.Getenv("OHC_STANDALONE") != "true" {
						_ = os.MkdirAll(openclawDir, 0700)
					} else {
						_ = os.MkdirAll(".agent-task", 0700)
					}
					privBytes := x509.MarshalPKCS1PrivateKey(privKey)
					pemBlock := &pem.Block{
						Type:  "RSA PRIVATE KEY",
						Bytes: privBytes,
					}
					_ = ioutil.WriteFile(keyPath, pem.EncodeToMemory(pemBlock), 0600)
				}
			} else {
				keyData, readErr := ioutil.ReadFile(keyPath)
				if readErr == nil {
					block, _ := pem.Decode(keyData)
					if block != nil {
						privKey, err = x509.ParsePKCS1PrivateKey(block.Bytes)
					} else {
						err = fmt.Errorf("failed to parse local powersync_rsa.pem")
					}
				} else {
					err = readErr
				}
			}
		}

		if err == nil && privKey != nil {
			rsaPublicKey = &privKey.PublicKey
		}
	})
	return rsaPublicKey, err
}

// handlePowerSyncJWKS provides the JSON Web Key Set (JWKS) required by PowerSync to verify JWTs.
func (s *Server) handlePowerSyncJWKS(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	pubKey, err := ensureRSAKey()
	if err != nil || pubKey == nil {
		slog.Error("failed to load/generate RSA key for JWKS", "error", err)
		http.Error(w, "internal server error", http.StatusInternalServerError)
		return
	}

	n := base64.RawURLEncoding.EncodeToString(pubKey.N.Bytes())
	e := base64.RawURLEncoding.EncodeToString([]byte{byte(pubKey.E >> 16), byte(pubKey.E >> 8), byte(pubKey.E)})

	// Pad E if needed as required by JWKS spec
	for len(e) < 4 && e[0] == 'A' {
		e = e[1:]
	}
	if len(e) == 0 {
		e = "AQAB"
	}

	jwks := map[string]interface{}{
		"keys": []map[string]interface{}{
			{
				"kty": "RSA",
				"alg": "RS256",
				"use": "sig",
				"kid": "powersync-key-1",
				"n":   n,
				"e":   e,
			},
		},
	}

	w.Header().Set("Content-Type", "application/json")
	_ = json.NewEncoder(w).Encode(jwks)
}
