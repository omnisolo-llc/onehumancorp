package auth

import (
	"testing"
)

func TestDeterministicEncryption(t *testing.T) {
	pt := "test@example.com"
	ct := encryptDeterministic(pt)
	if ct == pt {
		t.Errorf("Expected encrypted text to be different from plaintext")
	}

	pt2 := decryptDeterministic(ct)
	if pt2 != pt {
		t.Errorf("Expected decrypted text %q to match original %q", pt2, pt)
	}

	// Test deterministic property
	ct2 := encryptDeterministic(pt)
	if ct != ct2 {
		t.Errorf("Expected deterministic encryption to yield same ciphertext")
	}

	// Test fallback
	fb := decryptDeterministic("plain_unencrypted_email@example.com")
	if fb != "plain_unencrypted_email@example.com" {
		t.Errorf("Expected fallback for unencrypted string, got %q", fb)
	}
}
