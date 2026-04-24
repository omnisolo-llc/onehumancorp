package growth

import (
	"strings"
	"testing"
)

func TestGenerateReferralLink(t *testing.T) {
	userID := "user123"
	link, err := GenerateReferralLink(userID)

	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if !strings.HasPrefix(link, "ohc://join?ref=") {
		t.Errorf("Link %s does not have expected prefix", link)
	}

	if !strings.Contains(link, "utm_source=standalone_desktop") {
		t.Errorf("Link %s missing utm_source", link)
	}

	if !strings.Contains(link, "inviter=user123") {
		t.Errorf("Link %s missing inviter", link)
	}
}

func TestGenerateReferralLink_EmptyUser(t *testing.T) {
	_, err := GenerateReferralLink("")
	if err == nil {
		t.Fatalf("Expected error for empty user ID, got nil")
	}
}
