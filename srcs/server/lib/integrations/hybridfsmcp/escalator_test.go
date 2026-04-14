package hybridfsmcp

import (
	"context"
	"strings"
	"testing"
)

func TestComplexityAnalyzer(t *testing.T) {
	analyzer := NewComplexityAnalyzer(10) // 10 tokens = ~40 chars
	ctx := context.Background()

	shortQuery := "Hello world" // 11 chars
	if analyzer.ShouldEscalate(ctx, shortQuery) {
		t.Errorf("Short query should not escalate")
	}

	longQuery := strings.Repeat("A", 41) // 41 chars
	if !analyzer.ShouldEscalate(ctx, longQuery) {
		t.Errorf("Long query should escalate")
	}
}
