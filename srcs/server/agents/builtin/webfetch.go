package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"regexp"
	"strings"
)

var reScript = regexp.MustCompile(`(?is)<script[^>]*>.*?</script>`)
var reStyle = regexp.MustCompile(`(?is)<style[^>]*>.*?</style>`)
var reTag = regexp.MustCompile(`<[^>]+>`)
var reBlankLines = regexp.MustCompile(`(\n\s*){3,}`)

// WebFetchTool fetches the content of a URL and returns it as readable text.
// When the response is HTML it strips tags to produce clean text.
// Mirrors CC-Source's WebFetchTool (prompt: fetch a URL and extract text).
var WebFetchTool = Tool{
	Name: "WebFetch",
	Description: "Fetch the content of a URL and return it as text. " +
		"HTML pages are stripped to plain text. " +
		"Use for reading documentation, web pages, or raw text/JSON endpoints.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"url": {
				"type": "string",
				"description": "The URL to fetch."
			},
			"max_bytes": {
				"type": "integer",
				"description": "Maximum bytes to read (default 131072 = 128 KiB, max 524288)."
			}
		},
		"required": ["url"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			URL      string `json:"url"`
			MaxBytes int    `json:"max_bytes"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}
		if input.URL == "" {
			return "", fmt.Errorf("WebFetch: url is required")
		}
		maxBytes := input.MaxBytes
		if maxBytes <= 0 {
			maxBytes = 128 * 1024 // 128 KiB default
		}
		if maxBytes > 512*1024 {
			maxBytes = 512 * 1024 // 512 KiB hard cap
		}

		req, err := http.NewRequestWithContext(ctx, http.MethodGet, input.URL, nil)
		if err != nil {
			return "", fmt.Errorf("WebFetch: %w", err)
		}
		req.Header.Set("User-Agent", "Mozilla/5.0 (compatible; OHCAgent/1.0)")
		req.Header.Set("Accept", "text/html,application/xhtml+xml,application/json,text/plain;q=0.9,*/*;q=0.8")

		resp, err := http.DefaultClient.Do(req)
		if err != nil {
			return "", fmt.Errorf("WebFetch: %w", err)
		}
		defer resp.Body.Close()

		if resp.StatusCode >= 400 {
			return "", fmt.Errorf("WebFetch: HTTP %d for %s", resp.StatusCode, input.URL)
		}

		body, err := io.ReadAll(io.LimitReader(resp.Body, int64(maxBytes)))
		if err != nil {
			return "", fmt.Errorf("WebFetch: read body: %w", err)
		}

		ct := resp.Header.Get("Content-Type")
		if strings.Contains(ct, "text/html") || strings.Contains(ct, "application/xhtml") {
			return extractTextFromHTML(string(body)), nil
		}
		return string(body), nil
	},
}

// extractTextFromHTML strips script, style, and HTML tags and returns readable text.
func extractTextFromHTML(html string) string {
	s := reScript.ReplaceAllString(html, " ")
	s = reStyle.ReplaceAllString(s, " ")
	s = reTag.ReplaceAllString(s, " ")
	// Decode common HTML entities.
	s = strings.ReplaceAll(s, "&amp;", "&")
	s = strings.ReplaceAll(s, "&lt;", "<")
	s = strings.ReplaceAll(s, "&gt;", ">")
	s = strings.ReplaceAll(s, "&quot;", `"`)
	s = strings.ReplaceAll(s, "&#39;", "'")
	s = strings.ReplaceAll(s, "&nbsp;", " ")
	// Collapse blank lines.
	s = reBlankLines.ReplaceAllString(s, "\n\n")
	return strings.TrimSpace(s)
}