// Package imessage provides an API client for iMessage messaging.
// This is currently a stub for future implementation.
package imessage

import "fmt"

// Client interacts with the iMessage API.
type Client struct {
	BaseURL  string
	APIToken string
}

// NewClient creates an iMessage Client stub.
func NewClient() *Client {
	return &Client{}
}

// SendMessage sends a message to iMessage.
func (c *Client) SendMessage() error {
	return fmt.Errorf("imessage client: not implemented")
}
