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

// APIURL is the endpoint for Minimax reasoning.
var APIURL = "https://api.minimax.io/v1/chat/completions"

// Client handles interaction with the Minimax Model 2.7.
type Client struct {
	APIKey string
}

// NewClient creates a new Minimax client.
func NewClient(apiKey string) *Client {
	return &Client{APIKey: apiKey}
}

var bufferPool = sync.Pool{
	New: func() interface{} {
		return new(bytes.Buffer)
	},
}

var sharedHTTPClient = &http.Client{
	Timeout: 30 * time.Second,
	Transport: &http.Transport{
		MaxIdleConns:        1000,
		MaxIdleConnsPerHost: 1000,
		IdleConnTimeout:     90 * time.Second,
	},
}

// Reason prompts the minimax API.

// CircuitBreaker handles transient errors when making API requests.
type CircuitBreaker struct {
	mu           sync.Mutex
	state        string
	failureCount int
	lastFailure  time.Time
}

const (
	StateClosed   = "CLOSED"
	StateOpen     = "OPEN"
	StateHalfOpen = "HALF_OPEN"
)

var DefaultBreaker = &CircuitBreaker{
	state: StateClosed,
}

// Reset the circuit breaker state explicitly.
func (cb *CircuitBreaker) Reset() {
	cb.mu.Lock()
	defer cb.mu.Unlock()
	cb.state = StateClosed
	cb.failureCount = 0
	cb.lastFailure = time.Time{}
}

func (cb *CircuitBreaker) allowRequest() bool {
	cb.mu.Lock()
	defer cb.mu.Unlock()

	switch cb.state {
	case StateClosed:
		return true
	case StateOpen:
		if time.Since(cb.lastFailure) > 5*time.Second {
			cb.state = StateHalfOpen
			return true
		}
		return false
	case StateHalfOpen:
		return true
	}
	return false
}

func (cb *CircuitBreaker) recordSuccess() {
	cb.mu.Lock()
	defer cb.mu.Unlock()
	cb.state = StateClosed
	cb.failureCount = 0
}

func (cb *CircuitBreaker) recordFailure() {
	cb.mu.Lock()
	defer cb.mu.Unlock()
	cb.failureCount++
	cb.lastFailure = time.Now()
	if cb.failureCount >= 3 {
		cb.state = StateOpen
	}
}

func (c *Client) Reason(ctx context.Context, prompt string) (string, error) {
	if !DefaultBreaker.allowRequest() {
		return "", errors.New("circuit breaker is open")
	}

	if c.APIKey == "" {
		return "", errors.New("minimax API key is not configured")
	}

	url := APIURL
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
		return "", err
	}

	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+c.APIKey)

	// ⚡ BOLT: [Reused HTTP Client]
	resp, err := sharedHTTPClient.Do(req)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		respBody, _ := io.ReadAll(resp.Body)
		DefaultBreaker.recordFailure()
		return "", fmt.Errorf("minimax API error (status %d): %s", resp.StatusCode, string(respBody))
	}

	DefaultBreaker.recordSuccess()

	var result struct {
		Choices []struct {
			Message struct {
				Content string `json:"content"`
			} `json:"message"`
		} `json:"choices"`
	}

	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return "", err
	}

	if len(result.Choices) == 0 {
		return "", errors.New("empty response from minimax")
	}

	return result.Choices[0].Message.Content, nil
}
