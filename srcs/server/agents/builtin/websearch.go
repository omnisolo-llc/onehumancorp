package builtin

import (
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"regexp"
	"strings"
)

// WebSearchTool performs a web search using DuckDuckGo HTML endpoint.
// Falls back to a structured placeholder when the HTTP call fails.
var WebSearchTool = Tool{
	Name: "WebSearch",
	Description: "Search the web for information. Returns a list of relevant results. " +
		"Use this for current events, documentation, or any information not in your training data.",
	Parameters: json.RawMessage(`{
		"type": "object",
		"properties": {
			"query": {
				"type": "string",
				"description": "The search query."
			},
			"num_results": {
				"type": "integer",
				"description": "Maximum number of results to return (default 5, max 10)."
			}
		},
		"required": ["query"]
	}`),
	Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
		var input struct {
			Query      string `json:"query"`
			NumResults int    `json:"num_results"`
		}
		if err := json.Unmarshal(args, &input); err != nil {
			return "", err
		}
		if input.Query == "" {
			return "", fmt.Errorf("WebSearch: query is required")
		}
		if input.NumResults <= 0 {
			input.NumResults = 5
		}
		if input.NumResults > 10 {
			input.NumResults = 10
		}

		results, err := duckDuckGoSearch(ctx, input.Query, input.NumResults)
		if err != nil {
			// Return a helpful fallback rather than an error so the agent can continue.
			return fmt.Sprintf(
				"Web search for %q encountered an error (%v).\n"+
					"Search URL: https://duckduckgo.com/?q=%s",
				input.Query, err, url.QueryEscape(input.Query),
			), nil
		}
		return results, nil
	},
}

// reTitleURL extracts result titles and URLs from DuckDuckGo HTML.
// DuckDuckGo HTML results have: <a class="result__a" href="...">title</a>
var reDDGResult = regexp.MustCompile(`<a[^>]+class="result__a"[^>]+href="([^"]+)"[^>]*>([^<]+)</a>`)
var reHTMLTag = regexp.MustCompile(`<[^>]+>`)
var reMultiSpace = regexp.MustCompile(`\s+`)

func duckDuckGoSearch(ctx context.Context, query string, numResults int) (string, error) {
	searchURL := "https://html.duckduckgo.com/html/?q=" + url.QueryEscape(query)
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, searchURL, nil)
	if err != nil {
		return "", err
	}
	req.Header.Set("User-Agent", "Mozilla/5.0 (compatible; OHCAgent/1.0)")
	req.Header.Set("Accept", "text/html")

	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(io.LimitReader(resp.Body, 512*1024)) // 512 KB cap
	if err != nil {
		return "", err
	}

	matches := reDDGResult.FindAllStringSubmatch(string(body), numResults*2)
	if len(matches) == 0 {
		return fmt.Sprintf("No results found for %q. Try a different query.", query), nil
	}

	var sb strings.Builder
	sb.WriteString(fmt.Sprintf("Search results for %q:\n\n", query))
	seen := map[string]bool{}
	count := 0
	for _, m := range matches {
		if count >= numResults {
			break
		}
		rawURL := strings.TrimSpace(m[1])
		// handle relative duckduckgo urls if any
		if strings.HasPrefix(rawURL, "/") {
			rawURL = "https://duckduckgo.com" + rawURL
		}
		title := strings.TrimSpace(reHTMLTag.ReplaceAllString(m[2], ""))
		title = reMultiSpace.ReplaceAllString(title, " ")
		if title == "" || rawURL == "" || seen[rawURL] {
			continue
		}
		seen[rawURL] = true
		sb.WriteString(fmt.Sprintf("%d. %s\n   %s\n\n", count+1, title, rawURL))
		count++
	}
	if count == 0 {
		return fmt.Sprintf("No results found for %q.", query), nil
	}
	return sb.String(), nil
}