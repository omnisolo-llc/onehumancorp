package hybridfsmcp

import (
	"context"
	"testing"
)

func TestDynamicEscalator(t *testing.T) {
	escalator, err := NewDynamicEscalator(10)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	ctx := context.Background()

	// Under threshold -> local fallback
	if escalator.Escalate(ctx, "short") {
		t.Errorf("expected false for short query")
	}

	// Over threshold -> escalate
	if !escalator.Escalate(ctx, "this is a very long query indeed") {
		t.Errorf("expected true for long query")
	}
}
