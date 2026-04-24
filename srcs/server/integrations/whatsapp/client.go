// Package whatsapp provides an API client for WhatsApp messaging.
// This is currently a stub for future implementation.
package whatsapp

import "fmt"

// Client interacts with the WhatsApp API.
type Client struct {
	BaseURL  string
	APIToken string
}

// NewClient creates a WhatsApp Client stub.
func NewClient() *Client {
	return &Client{}
}

// SendMessage sends a message to Whatsapp.
func (c *Client) SendMessage() error {
	return fmt.Errorf("whatsapp client: not implemented")
}
