package secretssyncmcp

import (
	"bytes"
	"context"
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"path/filepath"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

// DBSecretsSyncProvider implements SecretsSyncProvider using the local db and cloud API.
type DBSecretsSyncProvider struct {
	dbWrapper   *db.DB
	cloudAPIURL string
}

// NewDBSecretsSyncProvider creates a new DBSecretsSyncProvider.
func NewDBSecretsSyncProvider(dbWrapper *db.DB, cloudAPIURL string) *DBSecretsSyncProvider {
	return &DBSecretsSyncProvider{
		dbWrapper:   dbWrapper,
		cloudAPIURL: cloudAPIURL,
	}
}

func getOrGenerateKey() ([]byte, error) {
	var homeDir string
	if testTempDir := os.Getenv("TEST_TMPDIR"); testTempDir != "" {
		homeDir = testTempDir
	} else {
		var err error
		homeDir, err = os.UserHomeDir()
		if err != nil {
			return nil, fmt.Errorf("could not get home dir: %w", err)
		}
	}

	ohcDir := filepath.Join(homeDir, ".ohc")
	if err := os.MkdirAll(ohcDir, 0700); err != nil {
		return nil, fmt.Errorf("could not create .ohc dir: %w", err)
	}

	keyPath := filepath.Join(ohcDir, "local_secrets.key")
	if _, err := os.Stat(keyPath); os.IsNotExist(err) {
		// Generate 32-byte key
		key := make([]byte, 32)
		if _, err := io.ReadFull(rand.Reader, key); err != nil {
			return nil, fmt.Errorf("could not generate key: %w", err)
		}
		if err := os.WriteFile(keyPath, key, 0600); err != nil {
			return nil, fmt.Errorf("could not write key file: %w", err)
		}
		return key, nil
	}

	// Ensure permissions are strict
	info, err := os.Stat(keyPath)
	if err != nil {
		return nil, fmt.Errorf("could not stat key file: %w", err)
	}
	if info.Mode().Perm() != 0600 {
		if err := os.Chmod(keyPath, 0600); err != nil {
			return nil, fmt.Errorf("could not fix key file permissions: %w", err)
		}
	}

	key, err := os.ReadFile(keyPath)
	if err != nil {
		return nil, fmt.Errorf("could not read key file: %w", err)
	}
	if len(key) != 32 {
		return nil, fmt.Errorf("invalid key length: expected 32 bytes, got %d", len(key))
	}

	return key, nil
}

func encryptValue(value string) (string, error) {
	key, err := getOrGenerateKey()
	if err != nil {
		return "", err
	}

	block, err := aes.NewCipher(key)
	if err != nil {
		return "", err
	}

	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", err
	}

	nonce := make([]byte, aesgcm.NonceSize())
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		return "", err
	}

	ciphertext := aesgcm.Seal(nil, nonce, []byte(value), nil)

	// Prepend nonce to ciphertext and base64 encode
	combined := append(nonce, ciphertext...)
	return base64.StdEncoding.EncodeToString(combined), nil
}

func decryptValue(encryptedValue string) (string, error) {
	key, err := getOrGenerateKey()
	if err != nil {
		return "", err
	}

	combined, err := base64.StdEncoding.DecodeString(encryptedValue)
	if err != nil {
		return "", err
	}

	block, err := aes.NewCipher(key)
	if err != nil {
		return "", err
	}

	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		return "", err
	}

	nonceSize := aesgcm.NonceSize()
	if len(combined) < nonceSize {
		return "", fmt.Errorf("ciphertext too short")
	}

	nonce, ciphertext := combined[:nonceSize], combined[nonceSize:]
	plaintext, err := aesgcm.Open(nil, nonce, ciphertext, nil)
	if err != nil {
		return "", err
	}

	return string(plaintext), nil
}

func (p *DBSecretsSyncProvider) sendToCloud(ctx context.Context, endpoint string, method string, payload interface{}, claims *auth.Claims) ([]byte, error) {
	if p.cloudAPIURL == "" {
		return nil, fmt.Errorf("cloud API URL is not configured")
	}

	var bodyReader io.Reader
	if payload != nil {
		jsonData, err := json.Marshal(payload)
		if err != nil {
			return nil, fmt.Errorf("marshal payload: %w", err)
		}
		bodyReader = bytes.NewBuffer(jsonData)
	}

	url := fmt.Sprintf("%s%s", p.cloudAPIURL, endpoint)
	req, err := http.NewRequestWithContext(ctx, method, url, bodyReader)
	if err != nil {
		return nil, fmt.Errorf("create request: %w", err)
	}

	if payload != nil {
		req.Header.Set("Content-Type", "application/json")
	}

	// Set SPIFFE authentication token header if identity token is provided in environment variables.
	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("Authorization", "Bearer "+spiffeToken)
	}

	// Add tenant info
	if claims != nil {
		req.Header.Set("X-Tenant-ID", claims.OrganizationID)
	}

	client := &http.Client{Timeout: 10 * time.Second}
	resp, err := client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("do request: %w", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("read response body: %w", err)
	}

	if resp.StatusCode >= 300 {
		return nil, fmt.Errorf("unexpected status %d: %s", resp.StatusCode, string(body))
	}

	return body, nil
}

// SyncSecretsDown pulls secret updates from the cloud to the local db.
func (p *DBSecretsSyncProvider) SyncSecretsDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	if p.dbWrapper == nil || !p.dbWrapper.IsSQLite() {
		return nil, fmt.Errorf("local database not configured or not running in standalone mode")
	}

	body, err := p.sendToCloud(ctx, "/api/v1/secrets/sync/down", http.MethodGet, nil, claims)
	if err != nil {
		return nil, fmt.Errorf("sync down from cloud failed: %w", err)
	}

	var response map[string]interface{}
	if err := json.Unmarshal(body, &response); err != nil {
		return nil, fmt.Errorf("failed to parse sync down payload: %w", err)
	}

	if secrets, ok := response["secrets"].([]interface{}); ok {
		for _, s := range secrets {
			if secretMap, ok := s.(map[string]interface{}); ok {
				id, _ := secretMap["id"].(string)
				key, _ := secretMap["key"].(string)
				value, _ := secretMap["value"].(string)

				if id != "" && key != "" {
					// Ensure the local secrets table exists
					_, _ = p.dbWrapper.Exec(ctx, "CREATE TABLE IF NOT EXISTS local_secrets (id TEXT PRIMARY KEY, key TEXT, value TEXT, synced_to_cloud BOOLEAN DEFAULT true)")

					// Real AES-GCM encryption for Standalone local secrets DB
					encryptedValue, err := encryptValue(value)
					if err != nil {
						return nil, fmt.Errorf("failed to encrypt secret %s: %w", key, err)
					}

					_, _ = p.dbWrapper.Exec(ctx, "INSERT INTO local_secrets (id, key, value, synced_to_cloud) VALUES ($1, $2, $3, true) ON CONFLICT(id) DO UPDATE SET value = excluded.value, key = excluded.key", id, key, encryptedValue)
				}
			}
		}
	}

	return map[string]interface{}{
		"status":  "success",
		"message": "Secrets sync down completed successfully.",
		"data":    response,
	}, nil
}

// SyncSecretsUp pushes local unsynced secrets to the cloud.
func (p *DBSecretsSyncProvider) SyncSecretsUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error) {
	if p.dbWrapper == nil || !p.dbWrapper.IsSQLite() {
		return nil, fmt.Errorf("local database not configured or not running in standalone mode")
	}

	_, _ = p.dbWrapper.Exec(ctx, "CREATE TABLE IF NOT EXISTS local_secrets (id TEXT PRIMARY KEY, key TEXT, value TEXT, synced_to_cloud BOOLEAN DEFAULT true)")

	rows, err := p.dbWrapper.Query(ctx, "SELECT id, key, value FROM local_secrets WHERE synced_to_cloud = false LIMIT 10")
	if err != nil {
		return nil, fmt.Errorf("query unsynced secrets: %w", err)
	}
	defer rows.Close()

	var secrets []map[string]interface{}
	var ids []string

	for rows.Next() {
		var id, key, encryptedValue string
		if err := rows.Scan(&id, &key, &encryptedValue); err != nil {
			continue
		}

		decryptedValue, err := decryptValue(encryptedValue)
		if err != nil {
			// Skip or log if we can't decrypt
			continue
		}

		secrets = append(secrets, map[string]interface{}{
			"id":    id,
			"key":   key,
			"value": decryptedValue,
		})
		ids = append(ids, id)
	}

	if len(secrets) == 0 {
		return map[string]interface{}{
			"status":       "success",
			"synced_count": 0,
			"message":      "No pending secrets to sync up.",
		}, nil
	}

	_, err = p.sendToCloud(ctx, "/api/v1/secrets/sync/up", http.MethodPost, map[string]interface{}{
		"secrets": secrets,
	}, claims)

	if err != nil {
		return nil, fmt.Errorf("sync up to cloud failed: %w", err)
	}

	for _, id := range ids {
		_, _ = p.dbWrapper.Exec(ctx, "UPDATE local_secrets SET synced_to_cloud = true WHERE id = $1", id)
	}

	return map[string]interface{}{
		"status":       "success",
		"synced_count": len(secrets),
	}, nil
}
