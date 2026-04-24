package whatsapp

import "testing"

func TestNewClient(t *testing.T) {
	c := NewClient()
	if c == nil {
		t.Fatal("expected client, got nil")
	}
}

func TestSendMessage(t *testing.T) {
	c := NewClient()
	err := c.SendMessage()
	if err == nil {
		t.Fatal("expected error, got nil")
	}
	if err.Error() != "whatsapp client: not implemented" {
		t.Fatalf("expected 'whatsapp client: not implemented', got %v", err)
	}
}
