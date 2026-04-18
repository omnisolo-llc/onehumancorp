package auth

import (
	"bytes"
	"os"
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

func TestGetCryptoKeyStandalone(t *testing.T) {
	// Setup env
	t.Setenv("OHC_STANDALONE", "true")
	t.Setenv("OHC_SQLITE_KEY", "")
	t.Setenv("OHC_SQLITE_ENCRYPTION_KEY", "")

	// Ensure temp file path
	tmpFile := t.TempDir() + "/test_key"
	t.Setenv("OHC_SQLITE_KEY_FILE", tmpFile)

	key1 := getCryptoKey()
	key2 := getCryptoKey()

	if len(key1) != 32 {
		t.Errorf("expected 32 byte key, got %d", len(key1))
	}
	if !bytes.Equal(key1, key2) {
		t.Errorf("expected key to be persisted and same across calls")
	}

	// Verify file was written
	b, err := os.ReadFile(tmpFile)
	if err != nil {
		t.Fatalf("expected key file to exist: %v", err)
	}
	if len(b) < 32 { // hex encoded
		t.Errorf("expected key material in file, got %s", b)
	}
}
