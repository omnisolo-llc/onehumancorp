// Package plane provides a REST API client for Plane, the open-source
// issue tracking platform used by OHC as its default task management infrastructure.
//
// Environment variables consumed:
//
//	PLANE_URL      – base URL of the Plane instance (default: http://plane-api:8000)
//	PLANE_API_KEY  – API key used for authentication
package plane

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"time"
)

// DefaultBaseURL is the in-cluster URL for the Plane API service.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
const DefaultBaseURL = "http://plane-api:8000"

// Client interacts with the Plane REST API.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Client struct {
	BaseURL    string
	APIKey     string
	Workspace  string
	Project    string
	httpClient *http.Client
}

// NewClientFromEnv creates a Client using environment variables.
// Accepts no parameters.
// Returns *Client.
// Produces no errors.
// Has no side effects.
func NewClientFromEnv() *Client {
	base := os.Getenv("PLANE_URL")
	if base == "" {
		base = DefaultBaseURL
	}
	return &Client{
		BaseURL:    base,
		APIKey:     os.Getenv("PLANE_API_KEY"),
		Workspace:  os.Getenv("PLANE_WORKSPACE"),
		Project:    os.Getenv("PLANE_PROJECT"),
		httpClient: &http.Client{Timeout: 15 * time.Second},
	}
}

// Issue represents a Plane issue summary.
// Accepts no parameters.
// Returns nothing.
// Produces no errors.
// Has no side effects.
type Issue struct {
	ID          string `json:"id"`
	Name        string `json:"name"`
	Description string `json:"description_html"`
	State       string `json:"state"`
	Priority    string `json:"priority"`
}

type issueListResponse struct {
	Results []Issue `json:"results"`
}

// ListOpenIssues returns open issues in the workspace and project.
// Accepts no parameters.
// Returns ([]Issue, error).
// Produces errors: Explicit error handling.
// Has no side effects.
func (c *Client) ListOpenIssues() ([]Issue, error) {
	if c.Workspace == "" || c.Project == "" {
		return nil, fmt.Errorf("plane client: workspace and project must be set")
	}

	var resp issueListResponse
	path := fmt.Sprintf("/api/v1/workspaces/%s/projects/%s/issues/?state=open", c.Workspace, c.Project)
	if err := c.get(path, &resp); err != nil {
		return nil, fmt.Errorf("plane list open issues: %w", err)
	}
	return resp.Results, nil
}

// UpdateIssueStatus updates the state of an issue (e.g. from open to in_progress).
// Accepts parameters: issueID, stateID string (No Constraints).
// Returns error.
// Produces errors: Explicit error handling.
// Has no side effects.
func (c *Client) UpdateIssueStatus(issueID string, stateID string) error {
	path := fmt.Sprintf("/api/v1/workspaces/%s/projects/%s/issues/%s/", c.Workspace, c.Project, issueID)
	body := map[string]string{
		"state": stateID,
	}
	var resp interface{}
	if err := c.patch(path, body, &resp); err != nil {
		return fmt.Errorf("plane update issue status: %w", err)
	}
	return nil
}

// ── HTTP helpers ──────────────────────────────────────────────────────────────

func (c *Client) get(path string, dest interface{}) error {
	req, err := http.NewRequest(http.MethodGet, c.BaseURL+path, nil)
	if err != nil {
		return err
	}
	c.addHeaders(req)
	return c.do(req, dest)
}

func (c *Client) patch(path string, body, dest interface{}) error {
	data, err := json.Marshal(body)
	if err != nil {
		return err
	}
	req, err := http.NewRequest(http.MethodPatch, c.BaseURL+path, bytes.NewReader(data))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")
	c.addHeaders(req)
	return c.do(req, dest)
}

func (c *Client) addHeaders(req *http.Request) {
	req.Header.Set("Accept", "application/json")
	if c.APIKey != "" {
		req.Header.Set("x-api-key", c.APIKey)
	}
}

func (c *Client) do(req *http.Request, dest interface{}) error {
	resp, err := c.httpClient.Do(req)
	if err != nil {
		return err
	}
	defer func() { _, _ = io.Copy(io.Discard, resp.Body); _ = resp.Body.Close() }()

	if resp.StatusCode >= 400 {
		b, _ := io.ReadAll(resp.Body)
		return fmt.Errorf("plane API %s %s returned %d: %s", req.Method, req.URL.Path, resp.StatusCode, string(b))
	}

	if dest != nil {
		if err := json.NewDecoder(resp.Body).Decode(dest); err != nil {
			return fmt.Errorf("plane decode response: %w", err)
		}
	}
	return nil
}
