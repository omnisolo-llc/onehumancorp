package crypto

import (
	"testing"
)

func TestEncryptDecrypt(t *testing.T) {
	secret := "test-secret-key"
	plaintext := "hello-world-ohc-security"

	ciphertext, err := EncryptDeterministic(plaintext, secret)
	if err != nil {
		t.Fatalf("encryption failed: %v", err)
	}

	if ciphertext == "" {
		t.Fatal("expected non-empty ciphertext")
	}

	// Verify determinism
	ciphertext2, _ := EncryptDeterministic(plaintext, secret)
	if ciphertext != ciphertext2 {
		t.Errorf("encryption is not deterministic: %s != %s", ciphertext, ciphertext2)
	}

	decrypted, err := Decrypt(ciphertext, secret)
	if err != nil {
		t.Fatalf("decryption failed: %v", err)
	}

	if decrypted != plaintext {
		t.Errorf("decrypted text mismatch: got %q, want %q", decrypted, plaintext)
	}
}

func TestDecrypt_InvalidSecret(t *testing.T) {
	secret1 := "secret-1"
	secret2 := "secret-2"
	plaintext := "sensitive data"

	ciphertext, _ := EncryptDeterministic(plaintext, secret1)
	_, err := Decrypt(ciphertext, secret2)

	if err == nil {
		t.Error("expected decryption failure with wrong secret")
	}
}

func TestEncrypt_EmptySecret(t *testing.T) {
	_, err := EncryptDeterministic("data", "")
	if err == nil {
		t.Error("expected error with empty secret")
	}
}

func TestDecrypt_Errors(t *testing.T) {
	secret := "key"
	if _, err := Decrypt("", secret); err == nil {
		t.Error("expected error for empty ciphertext")
	}
	if _, err := Decrypt("zz", secret); err == nil {
		t.Error("expected error for invalid hex")
	}
	if _, err := Decrypt("000102030405060708090a", secret); err == nil {
		t.Error("expected error for short ciphertext")
	}
	if _, err := Decrypt("data", ""); err == nil {
		t.Error("expected error for empty secret")
	}
}
