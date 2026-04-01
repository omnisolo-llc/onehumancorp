package main

import (
	"crypto/rand"
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
	"time"

	"github.com/golang-jwt/jwt/v5"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type TokenResponse struct {
	Token        string `json:"token"`
	ExpiresAt    string `json:"expires_at"`
	PowersyncURL string `json:"powersync_url"`
}

// ensurePowerSyncKey generates or loads an RSA private key for PowerSync JWKS.
func ensurePowerSyncKey() (*rsa.PrivateKey, error) {
	if keyEnv := os.Getenv("POWERSYNC_RSA_PRIVATE_KEY"); keyEnv != "" {
		block, _ := pem.Decode([]byte(keyEnv))
		if block == nil {
			return nil, fmt.Errorf("failed to parse PEM block from environment")
		}
		key, err := x509.ParsePKCS1PrivateKey(block.Bytes)
		if err == nil {
			return key, nil
		}
		// Try PKCS8
		key8, err8 := x509.ParsePKCS8PrivateKey(block.Bytes)
		if err8 == nil {
			if rsaKey, ok := key8.(*rsa.PrivateKey); ok {
				return rsaKey, nil
			}
		}
		return nil, fmt.Errorf("failed to parse RSA private key from environment")
	}

	homeDir, err := os.UserHomeDir()
	if err != nil {
		return nil, err
	}
	keyPath := filepath.Join(homeDir, ".ohc", "powersync_rsa.pem")

	if _, err := os.Stat(keyPath); os.IsNotExist(err) {
		// Generate new key
		privateKey, err := rsa.GenerateKey(rand.Reader, 2048)
		if err != nil {
			return nil, err
		}

		keyBytes := x509.MarshalPKCS1PrivateKey(privateKey)
		pemBlock := &pem.Block{
			Type:  "RSA PRIVATE KEY",
			Bytes: keyBytes,
		}

		if err := os.MkdirAll(filepath.Dir(keyPath), 0700); err != nil {
			return nil, err
		}

		file, err := os.OpenFile(keyPath, os.O_CREATE|os.O_WRONLY, 0600)
		if err != nil {
			return nil, err
		}
		defer file.Close()

		if err := pem.Encode(file, pemBlock); err != nil {
			return nil, err
		}

		return privateKey, nil
	}

	keyData, err := os.ReadFile(keyPath)
	if err != nil {
		return nil, err
	}

	block, _ := pem.Decode(keyData)
	if block == nil {
		return nil, fmt.Errorf("failed to parse PEM block from file")
	}

	key, err := x509.ParsePKCS1PrivateKey(block.Bytes)
	if err == nil {
		return key, nil
	}

	key8, err8 := x509.ParsePKCS8PrivateKey(block.Bytes)
	if err8 == nil {
		if rsaKey, ok := key8.(*rsa.PrivateKey); ok {
			return rsaKey, nil
		}
	}

	return nil, fmt.Errorf("failed to parse RSA private key from file")
}

func jwksHandler(w http.ResponseWriter, r *http.Request) {
	key, err := ensurePowerSyncKey()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	e := base64.RawURLEncoding.EncodeToString(big.NewInt(int64(key.PublicKey.E)).Bytes())
	n := base64.RawURLEncoding.EncodeToString(key.PublicKey.N.Bytes())

	jwks := map[string]interface{}{
		"keys": []map[string]interface{}{
			{
				"kty": "RSA",
				"kid": "powersync-key-1",
				"alg": "RS256",
				"use": "sig",
				"n":   n,
				"e":   e,
			},
		},
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(jwks)
}

func syncRulesHandler(w http.ResponseWriter, r *http.Request) {
	// For PowerSync sync rules, strictly use the JourneyApps bucket_data schema
	// (e.g., bucket_data: global: data: ...)
	w.Header().Set("Content-Type", "application/yaml")
	// Strict tenant isolation via user_data and token_parameters.organization_id filtering
	rules := `bucket_data:
  user_data:
    parameters:
      - SELECT token_parameters.organization_id as org_id
    data:
      - SELECT * FROM agent_missions WHERE organization_id = token_parameters.organization_id`

	w.Write([]byte(rules))
}

func tokenHandler(w http.ResponseWriter, r *http.Request) {
	// Require authentication to issue a PowerSync JWT
	user, err := auth.UserFromContext(r.Context())
	if err != nil {
		http.Error(w, "Unauthorized", http.StatusUnauthorized)
		return
	}

	key, err := ensurePowerSyncKey()
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	// Create a new JWT for PowerSync matching the required claims format
	claims := jwt.MapClaims{
		"iss": "ohc-server",
		"sub": user.ID,
		"aud": "powersync",
		"exp": time.Now().Add(1 * time.Hour).Unix(),
		"iat": time.Now().Unix(),
		"organization_id": user.OrganizationID,
	}

	token := jwt.NewWithClaims(jwt.SigningMethodRS256, claims)
	token.Header["kid"] = "powersync-key-1"

	signedToken, err := token.SignedString(key)
	if err != nil {
		http.Error(w, "failed to sign token", http.StatusInternalServerError)
		return
	}

	response := TokenResponse{
		Token:        signedToken,
		ExpiresAt:    time.Now().Add(1 * time.Hour).Format(time.RFC3339),
		PowersyncURL: "http://localhost:8081",
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(response)
}

type PowerSyncUploadRequest struct {
	Transaction struct {
		Crud []struct {
			Op   string `json:"op"`
			Data map[string]interface{} `json:"data"`
		} `json:"crud"`
	} `json:"transaction"`
}

func newUploadHandler(pool *db.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, req *http.Request) {
		user, err := auth.UserFromContext(req.Context())
		if err != nil {
			http.Error(w, "Unauthorized", http.StatusUnauthorized)
			return
		}

		var pReq PowerSyncUploadRequest
		if err := json.NewDecoder(req.Body).Decode(&pReq); err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		defer req.Body.Close()

		ctx := req.Context()
		for _, crud := range pReq.Transaction.Crud {
			id, ok := crud.Data["id"].(string)
			if !ok {
				continue
			}

			if crud.Op == "PUT" || crud.Op == "PATCH" {
				status, _ := crud.Data["status"].(string)
				payload, _ := crud.Data["payload"].(string)

				// Ensure tenant isolation: only allow inserting/updating for the current user's org
				_, execErr := pool.Exec(ctx, `
					INSERT INTO agent_missions (id, status, payload, organization_id, created_at)
					VALUES ($1, $2, $3, $4, NOW())
					ON CONFLICT(id) DO UPDATE SET
						status=EXCLUDED.status,
						payload=EXCLUDED.payload
					WHERE agent_missions.organization_id = $4`,
					id, status, payload, user.OrganizationID)
				if execErr != nil {
					http.Error(w, "database error", http.StatusInternalServerError)
					return
				}
			} else if crud.Op == "DELETE" {
				_, execErr := pool.Exec(ctx, "DELETE FROM agent_missions WHERE id = $1 AND organization_id = $2", id, user.OrganizationID)
				if execErr != nil {
					http.Error(w, "database error", http.StatusInternalServerError)
					return
				}
			}
		}

		w.WriteHeader(http.StatusOK)
	}
}
