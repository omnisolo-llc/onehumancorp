package harness

import (
	"testing"
)

func TestIsolationHarness_Interface(t *testing.T) {
	h := NewIsolationHarness()
	if h == nil {
		t.Fatal("NewIsolationHarness returned nil")
	}
}
