package integrations

import (
	"context"
	"os"
	"testing"
)

func TestSPIFFEValidator_ValidateSVID(t *testing.T) {
	t.Run("StandaloneMode", func(t *testing.T) {
		os.Unsetenv("OHC_MULTITENANT")
		v := NewSPIFFEValidator()
		err := v.ValidateSVID(context.Background(), "anything")
		if err != nil {
			t.Errorf("expected nil error in standalone mode, got %v", err)
		}
	})

	t.Run("CloudMode_Valid", func(t *testing.T) {
		os.Setenv("OHC_MULTITENANT", "true")
		os.Setenv("SPIFFE_TRUST_DOMAIN", "example.org")
		v := NewSPIFFEValidator()
		err := v.ValidateSVID(context.Background(), "spiffe://example.org/agent/1")
		if err != nil {
			t.Errorf("expected nil error, got %v", err)
		}
	})

	t.Run("CloudMode_InvalidFormat", func(t *testing.T) {
		os.Setenv("OHC_MULTITENANT", "true")
		v := NewSPIFFEValidator()
		err := v.ValidateSVID(context.Background(), "not-spiffe")
		if err == nil {
			t.Error("expected error for invalid format, got nil")
		}
	})

	t.Run("CloudMode_TrustDomainMismatch", func(t *testing.T) {
		os.Setenv("OHC_MULTITENANT", "true")
		os.Setenv("SPIFFE_TRUST_DOMAIN", "example.org")
		v := NewSPIFFEValidator()
		err := v.ValidateSVID(context.Background(), "spiffe://other.com/agent/1")
		if err == nil {
			t.Error("expected error for trust domain mismatch, got nil")
		}
	})
}

func TestSPIFFEValidator_VerifyAgentIdentity(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("SPIFFE_TRUST_DOMAIN", "example.org")
	v := NewSPIFFEValidator()

	t.Run("Match", func(t *testing.T) {
		err := v.VerifyAgentIdentity(context.Background(), "spiffe://example.org/agent/agent-1", "agent-1")
		if err != nil {
			t.Errorf("expected match, got %v", err)
		}
	})

	t.Run("Mismatch", func(t *testing.T) {
		err := v.VerifyAgentIdentity(context.Background(), "spiffe://example.org/agent/agent-1", "agent-2")
		if err == nil {
			t.Error("expected mismatch error, got nil")
		}
	})

	t.Run("NonStandardFormatMatch", func(t *testing.T) {
		err := v.VerifyAgentIdentity(context.Background(), "spiffe://example.org/workload/agent-1", "agent-1")
		if err != nil {
			t.Errorf("expected match via fallback, got %v", err)
		}
	})
}
