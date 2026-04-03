package orchestration

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
)

// HTTPCloudClient implements CloudClient by making an HTTP POST request.
type HTTPCloudClient struct {
	endpoint string
}

// NewHTTPCloudClient creates a new HTTPCloudClient.
func NewHTTPCloudClient(endpoint string) *HTTPCloudClient {
	return &HTTPCloudClient{
		endpoint: endpoint,
	}
}

// PushSanitizedMemory pushes sanitized data to the configured HTTP endpoint.
func (c *HTTPCloudClient) PushSanitizedMemory(ctx context.Context, memoryID, sanitizedContext string) (string, error) {
	payload := map[string]string{
		"memory_id": memoryID,
		"context":   sanitizedContext,
	}
	body, err := json.Marshal(payload)
	if err != nil {
		return "", err
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, c.endpoint, bytes.NewBuffer(body))
	if err != nil {
		return "", err
	}
	req.Header.Set("Content-Type", "application/json")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 300 {
		return "", fmt.Errorf("unexpected status code: %d", resp.StatusCode)
	}

	// For simplicity, just return a dummy mission ID if the push was successful.
	return fmt.Sprintf("mission-%s", memoryID), nil
}
