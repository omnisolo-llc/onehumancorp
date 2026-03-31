// Package jira provides a REST API client for Jira issue tracking.
// This is currently a stub for future implementation.
package jira

import "fmt"

// Client interacts with the Jira REST API.
type Client struct {
	BaseURL  string
	APIToken string
	Email    string
}

// NewClient creates a Jira Client stub.
func NewClient() *Client {
	return &Client{}
}

// ListOpenIssues returns open issues.
func (c *Client) ListOpenIssues() ([]interface{}, error) {
	return nil, fmt.Errorf("jira client: not implemented")
}
