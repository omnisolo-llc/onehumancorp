package market_research

import "testing"

func TestResearch(t *testing.T) {
	if !Research("test query") {
		t.Error("Research should return true for a non-empty query")
	}
	if Research("") {
		t.Error("Research should return false for an empty query")
	}
}
