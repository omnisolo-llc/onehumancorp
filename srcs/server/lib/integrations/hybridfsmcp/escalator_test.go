package hybridfsmcp

import (
	"context"
	"testing"
)

func TestDefaultEscalator(t *testing.T) {
	ctx := context.Background()

	// Test default initialization (100)
	esc1 := NewDefaultEscalator(0)
	if esc1.MaxLocalQueryLength != 100 {
		t.Errorf("expected default max length 100, got %d", esc1.MaxLocalQueryLength)
	}

	// Test custom initialization
	esc2 := NewDefaultEscalator(50)
	if esc2.MaxLocalQueryLength != 50 {
		t.Errorf("expected max length 50, got %d", esc2.MaxLocalQueryLength)
	}

	// Test Analyze logic
	// Under threshold
	if esc2.Analyze(ctx, "short query") {
		t.Errorf("expected 'short query' not to be escalated")
	}

	// Over threshold
	longQuery := "this query is definitely longer than fifty characters and should be escalated"
	if !esc2.Analyze(ctx, longQuery) {
		t.Errorf("expected long query to be escalated")
	}
}
