package minimax

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"sync"
	"time"
)

// minimaxAPIURL is the endpoint for Minimax reasoning.
// ⚡ BOLT: [Configurable endpoint] - Randomized Selection from Top 5
var minimaxAPIURL = "https://api.minimax.io/v1/chat/completions"

// GetMinimaxAPIURL returns the current URL for the minimax API.
func GetMinimaxAPIURL() string {
	return minimaxAPIURL
}

// SetMinimaxAPIURL sets the URL for the minimax API.
func SetMinimaxAPIURL(url string) {
	minimaxAPIURL = url
}

type state int

const (
	stateClosed state = iota
	stateOpen
	stateHalfOpen
)

// MinimaxClient handles interaction with the Minimax Model 2.7.
type MinimaxClient struct {
	APIKey string

	mu           sync.RWMutex
	failures     int
	threshold    int
	state        state
	lastFailure  time.Time
	resetTimeout time.Duration
}

// NewMinimaxClient creates a new MinimaxClient.
func NewMinimaxClient(apiKey string) *MinimaxClient {
	return &MinimaxClient{
		APIKey:       apiKey,
		threshold:    3,
		resetTimeout: 10 * time.Second,
		state:        stateClosed,
	}
}

var bufferPool = sync.Pool{
	New: func() interface{} {
		return new(bytes.Buffer)
	},
}

var sharedHTTPClient = &http.Client{
	Timeout: 30 * time.Second,
}

func (c *MinimaxClient) allowRequest() bool {
	c.mu.RLock()
	st := c.state
	lf := c.lastFailure
	c.mu.RUnlock()

	if st == stateClosed {
		return true
	}
	if st == stateOpen {
		if time.Since(lf) > c.resetTimeout {
			c.mu.Lock()
			c.state = stateHalfOpen
			c.mu.Unlock()
			return true
		}
		return false
	}
	// stateHalfOpen: allow 1 request
	return true
}

func (c *MinimaxClient) recordSuccess() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.failures = 0
	c.state = stateClosed
}

func (c *MinimaxClient) recordFailure() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.failures++
	c.lastFailure = time.Now()
	if c.failures >= c.threshold {
		c.state = stateOpen
	}
}

// Reason sends a prompt to the Minimax API and returns the generated reasoning.
func (c *MinimaxClient) Reason(ctx context.Context, prompt string) (string, error) {
	if !c.allowRequest() {
		return "", errors.New("minimax API circuit breaker is open")
	}

	if c.APIKey == "" {
		c.recordFailure()
		return "", errors.New("minimax API key is not configured")
	}

	url := minimaxAPIURL
	// Optimization: construct the JSON payload manually to avoid
	// maps and slices allocations.
	buf := bufferPool.Get().(*bytes.Buffer)
	buf.Reset()
	defer bufferPool.Put(buf)

	buf.WriteString(`{"model":"MiniMax-M2.7","messages":[{"role":"user","content":`)
	enc := json.NewEncoder(buf)
	enc.SetEscapeHTML(false)
	_ = enc.Encode(prompt)
	// Encode adds a newline, so we slice it off and add the closing brackets
	buf.Truncate(buf.Len() - 1)
	buf.WriteString(`}]}`)

	req, err := http.NewRequestWithContext(ctx, "POST", url, buf)
	if err != nil {
		c.recordFailure()
		return "", err
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+c.APIKey)

	// ⚡ BOLT: [Reused HTTP Client] - Randomized Selection from Top 5
	// Prevents severe connection and resource leaks by reusing connection pools on every request.
	resp, err := sharedHTTPClient.Do(req)
	if err != nil {
		c.recordFailure()
		return "", err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		c.recordFailure()
		respBody, _ := io.ReadAll(resp.Body)
		return "", fmt.Errorf("minimax API error (status %d): %s", resp.StatusCode, string(respBody))
	}

	var result struct {
		Choices []struct {
			Message struct {
				Content string `json:"content"`
			} `json:"message"`
		} `json:"choices"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		c.recordFailure()
		return "", err
	}

	if len(result.Choices) == 0 {
		c.recordFailure()
		return "", errors.New("empty response from minimax")
	}

	c.recordSuccess()
	return result.Choices[0].Message.Content, nil
}
