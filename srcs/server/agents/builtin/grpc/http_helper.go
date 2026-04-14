package agentgrpc

import (
	"bytes"
	"context"
	"fmt"
	"io"
	"net/http"
	"time"
)

// httpReq and httpResp are aliases so model_adapter.go can reference them
// without importing net/http directly.
type httpReq = http.Request
type httpResp = http.Response

// doHTTP performs a simple HTTP request and returns the response body bytes.
func doHTTP(ctx context.Context, method, url string, body []byte, headers map[string]string, timeout time.Duration) ([]byte, error) {
	client := &http.Client{Timeout: timeout}

	req, err := http.NewRequestWithContext(ctx, method, url, bytes.NewReader(body))
	if err != nil {
		return nil, fmt.Errorf("http request: %w", err)
	}
	for k, v := range headers {
		req.Header.Set(k, v)
	}

	resp, err := client.Do(req)
	if err != nil {
		return nil, fmt.Errorf("http do: %w", err)
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return nil, fmt.Errorf("http read body: %w", err)
	}

	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return nil, fmt.Errorf("http %d: %s", resp.StatusCode, truncateStr(string(respBody), 256))
	}

	return respBody, nil
}

func truncateStr(s string, n int) string {
	if len(s) <= n {
		return s
	}
	return s[:n] + "…"
}
