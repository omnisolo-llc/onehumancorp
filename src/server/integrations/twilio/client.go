// Package twilio provides an API client for Twilio messaging.
package twilio

import "fmt"

// Client interacts with the Twilio API.
type Client struct {
	BaseURL    string
	AccountSID string
	AuthToken  string
}

// NewClient creates a Twilio Client stub.
func NewClient() *Client {
	return &Client{}
}

// SendMessage sends a message via Twilio.
func (c *Client) SendMessage() error {
	return fmt.Errorf("twilio client: not implemented")
}
