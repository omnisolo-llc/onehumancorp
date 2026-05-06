package interop

import (
	"errors"
	"strings"
)

// ValidateSPIFFEID validates a SPIFFE ID string
func ValidateSPIFFEID(spiffeID string) error {
	if !strings.HasPrefix(spiffeID, "spiffe://") {
		return errors.New("invalid SPIFFE ID format")
	}
	parts := strings.SplitN(spiffeID[9:], "/", 2)
	if len(parts) == 0 || parts[0] == "" {
		return errors.New("invalid SPIFFE ID: missing trust domain")
	}
	return nil
}
