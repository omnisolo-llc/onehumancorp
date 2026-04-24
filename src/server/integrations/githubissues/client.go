// Package githubissues provides a REST API client for GitHub Issues tracking.
// This is currently a stub for future implementation.
package githubissues

import "fmt"

// Client interacts with the GitHub REST API.
type Client struct {
	BaseURL  string
	APIToken string
}

// NewClient creates a GitHub Issues Client stub.
func NewClient() *Client {
	return &Client{}
}

// ListOpenIssues returns open issues.
func (c *Client) ListOpenIssues() ([]interface{}, error) {
	return nil, fmt.Errorf("github issues client: not implemented")
}
