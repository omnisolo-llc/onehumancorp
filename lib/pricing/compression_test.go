package pricing

import (
	"testing"
)

func TestReduceTokens(t *testing.T) {
	text := "This is a test of the token reduction system."
	reduced := ReduceTokens(text)
	if reduced == text {
		t.Errorf("Text was not reduced")
	}
}

func TestCompressText(t *testing.T) {
	text := "This is a test of the token compression system for the Agentic OS."
	compressed, err := CompressText(text)
	if err != nil {
		t.Fatalf("CompressText failed: %v", err)
	}
	if len(compressed) == 0 {
		t.Errorf("Compressed text is empty")
	}

	decompressed, err := DecompressText(compressed)
	if err != nil {
		t.Fatalf("DecompressText failed: %v", err)
	}

	// Verify lossless compression
	if decompressed != text {
		t.Errorf("Decompressed text does not match original: got %q, want %q", decompressed, text)
	}
}
