package interop

import "testing"

func TestValidateSPIFFEID(t *testing.T) {
	err := ValidateSPIFFEID("spiffe://ohc.org/agent1")
	if err != nil {
		t.Errorf("Expected nil, got %v", err)
	}

	err = ValidateSPIFFEID("invalid")
	if err == nil {
		t.Errorf("Expected error for invalid format, got nil")
	}
}
